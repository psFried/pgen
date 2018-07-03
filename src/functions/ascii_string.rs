use super::FunctionCreator;
use generator::{GeneratorType, GeneratorArg};


pub struct RandomAsciiString1;
impl FunctionCreator for RandomAsciiString1 {
    fn get_name(&self) -> &'static str {
        "asciiString"
    }

    fn get_arg_types(&self) -> (&'static [GeneratorType], bool) {
        (&[GeneratorType::UnsignedInt], false)
    }

    fn create(&self, mut args: Vec<GeneratorArg>) -> GeneratorArg {
        use generator::string::StringGenerator;
        let len_gen = args.pop().unwrap().as_uint().unwrap();
        GeneratorArg::String(StringGenerator::with_length(len_gen))
    }
}

/// 0-arg version of asciiString
pub struct RandomAsciiString0;
impl FunctionCreator for RandomAsciiString0 {
    fn get_name(&self) -> &'static str {
        "asciiString"
    }

    fn get_arg_types(&self) -> (&'static [GeneratorType], bool) {
        (&[], false)
    }

    fn create(&self, _args: Vec<GeneratorArg>) -> GeneratorArg {
        use generator::string::{default_charset, default_string_length_generator, StringGenerator};
        GeneratorArg::String(StringGenerator::new(default_string_length_generator(), default_charset()))
    }
}

pub struct AlphaNumeric;
impl FunctionCreator for AlphaNumeric {
    fn get_name(&self) -> &'static str {
        "alphanum"
    }

    fn get_arg_types(&self) -> (&'static [GeneratorType], bool) {
        (&[], false)
    }

    fn create(&self, _args: Vec<GeneratorArg>) -> GeneratorArg {
        GeneratorArg::Char(::generator::string::default_charset())
    }
}