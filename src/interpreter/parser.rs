use failure::Error;
use interpreter::ast::{MacroDef, Program};
use interpreter::grammar::{LibraryParser, ProgramParser};

pub fn parse_program(input: &str) -> Result<Program, Error> {
    ProgramParser::new()
        .parse(input)
        .map_err(|err| format_err!("Parse Error: {}", err))
}

pub fn parse_library(input: &str) -> Result<Vec<MacroDef>, Error> {
    LibraryParser::new()
        .parse(input)
        .map_err(|err| format_err!("Parse Error: {}", err))
}
