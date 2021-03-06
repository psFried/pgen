use crate::interpreter::{DgenParseError, Interpreter, UnreadSource};
use crate::{AnyFunction, DataGenOutput, ProgramContext};
use failure::Error;
use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::io::{self, Write};

const MAX_EMPTY_LINES: u32 = 2;

pub struct Repl {
    context: ProgramContext,
    interpreter: Interpreter,
    editor: Editor<()>,
    module_source: String,
    partial_source: String,
    consecutive_blank_lines: u32,
    awaiting_incomplete_input: bool,
}

const MODULE_NAME: &str = "default";

fn execute_fn(function: AnyFunction, context: &mut ProgramContext) -> Result<(), Error> {
    let out = io::stdout();
    let mut lock = out.lock();

    let result = {
        let mut dgen_out = DataGenOutput::new(&mut lock);
        function.write_value(context, &mut dgen_out).and_then(|_| {
            // need to write a newline at the end to ensure that the last line of output doesn't get clobbered
            // by the next readline prompt
            dgen_out
                .write_str("\n")
                .and_then(|_| {
                    // probably not necessary but it's good to do the write thing
                    dgen_out.flush().map_err(Into::into)
                })
                .map_err(Into::into)
        })
    };

    if let Err(err) = result {
        writeln!(lock, "Program Error: {}", err)?;
    }
    Ok(())
}

impl Repl {
    pub fn new(context: ProgramContext, interpreter: Interpreter) -> Repl {
        Repl {
            context,
            interpreter,
            editor: Editor::new(),
            module_source: String::with_capacity(1024),
            partial_source: String::with_capacity(512),
            consecutive_blank_lines: 0,
            awaiting_incomplete_input: false,
        }
    }

    pub fn run(mut self) -> Result<(), Error> {
        self.help();
        loop {
            let prompt = if self.awaiting_incomplete_input {
                "> ... "
            } else {
                "> "
            };
            match self.editor.readline(prompt) {
                Ok(next_line) => {
                    self.handle_new_input(next_line)?;
                }
                Err(ReadlineError::Eof) => return Ok(()),
                Err(ReadlineError::Interrupted) => return Ok(()),
                Err(ReadlineError::Io(e)) => return Err(e.into()),

                #[cfg(unix)]
                Err(ReadlineError::Errno(e)) => bail!("Syscall Error: {}", e),
                #[cfg(unix)]
                Err(ReadlineError::Utf8Error) => bail!("UTF8 Error"),

                #[cfg(windows)]
                Err(ReadlineError::Decode(e)) => bail!("Decoding Error: {}", e),
                #[cfg(windows)]
                Err(ReadlineError::WindowResize) => return Ok(()),
            };
        }
    }

    fn handle_new_input(&mut self, new_input: String) -> Result<(), Error> {
        if self.handle_meta_command(new_input.as_str()) {
            self.editor.add_history_entry(new_input);
            return Ok(());
        }

        self.partial_source.push_str(new_input.as_str());

        if new_input.as_str().trim().is_empty() && self.consecutive_blank_lines < MAX_EMPTY_LINES {
            self.consecutive_blank_lines += 1;
            return Ok(());
        } else {
            self.awaiting_incomplete_input = false;
        }

        let mut new_combined_input = self.module_source.clone();
        new_combined_input.push_str("\n");
        new_combined_input.push_str(self.partial_source.as_str());

        self.interpreter.remove_module(MODULE_NAME);
        let result = self
            .interpreter
            .eval_any(UnreadSource::String(new_combined_input));
        match result {
            Ok(Some(function)) => {
                let history_entry = self.partial_source.as_str().replace("\n", " ");
                self.partial_source.clear();
                self.editor.add_history_entry(history_entry);
                execute_fn(function, &mut self.context)?;
            }
            Ok(None) => {
                println!("Added function");
                let history_entry = self.partial_source.as_str().replace("\n", " ");
                self.partial_source.clear();
                self.editor.add_history_entry(history_entry);
                self.push_partial_source_to_module();
            }
            Err(ref err) if is_unexpected_eof_parse_err(err) => {
                if self.consecutive_blank_lines >= MAX_EMPTY_LINES {
                    self.partial_source.clear();
                    println!("Error: {}", err);
                } else {
                    self.awaiting_incomplete_input = true;
                }
            }
            Err(err) => {
                println!("Error: {}", err);
                // still push to the history, just to make it easier for folks to fix typos
                let history_entry = self.partial_source.as_str().replace("\n", " ");
                self.partial_source.clear();
                self.editor.add_history_entry(history_entry);
            }
        }
        self.consecutive_blank_lines = 0;
        Ok(())
    }

    fn clear(&mut self) {
        self.interpreter.remove_module(MODULE_NAME);
        self.module_source.clear();
        self.partial_source.clear();
    }

    fn show(&mut self) {
        println!("# Current Source: \n {}", self.module_source);
    }

    fn help(&mut self) {
        println!("{}", HELP_TXT);
    }

    fn handle_meta_command(&mut self, line: &str) -> bool {
        match get_metacommand(line) {
            Some(MetaCommand::Clear) => {
                self.clear();
                true
            }
            Some(MetaCommand::Show) => {
                self.show();
                true
            }
            Some(MetaCommand::Help) => {
                self.help();
                true
            }
            None => false,
        }
    }

    fn push_partial_source_to_module(&mut self) {
        self.module_source.push_str("\n");
        self.module_source.push_str(self.partial_source.as_str());
        self.partial_source.clear();
    }
}

#[derive(Debug)]
enum MetaCommand {
    Clear,
    Show,
    Help,
}

impl ::std::str::FromStr for MetaCommand {
    type Err = ();
    fn from_str(val: &str) -> Result<MetaCommand, ()> {
        match val {
            "clear" => Ok(MetaCommand::Clear),
            "show" => Ok(MetaCommand::Show),
            "help" => Ok(MetaCommand::Help),
            _ => Err(()),
        }
    }
}

fn get_metacommand(line: &str) -> Option<MetaCommand> {
    let line = line.trim();
    line.split("\\s+")
        .nth(0)
        .and_then(|split| split.parse().ok())
}

fn is_unexpected_eof_parse_err(err: &Error) -> bool {
    if let Some(parse_err) = err.downcast_ref::<DgenParseError>() {
        if parse_err.is_unexpected_eof() {
            return true;
        }
    }
    false
}

const HELP_TXT: &str = r##"dgen shell:
You can type dgen code as you normally would. All language features are supported.
Special commands are:

show -> prints the current module source
clear -> clears all  functions that have been declared
help -> prints this message
"##;
