use super::FunctionCreator;
use generator::{GeneratorType, GeneratorArg};
use generator::uint::UnsignedIntGenerator;

pub struct UnsignedInt0;
impl FunctionCreator for UnsignedInt0 {
    fn get_name(&self) -> &'static str {
        "uint"
    }

    fn get_arg_types(&self) -> (&'static [GeneratorType], bool) {
        (&[], false)
    }

    fn get_description(&self) -> &'static str {
        "generates an unsigned integer between 0 and 18,446,744,073,709,551,616 (2^64 - 1)"
    }

    fn create(&self, args: Vec<GeneratorArg>) -> GeneratorArg {
        GeneratorArg::UnsignedInt(UnsignedIntGenerator::with_default())
    }
}

pub struct UnsignedInt1;
impl FunctionCreator for UnsignedInt1 {
    fn get_name(&self) -> &'static str {
        "uint"
    }

    fn get_arg_types(&self) -> (&'static [GeneratorType], bool) {
        (&[GeneratorType::UnsignedInt], false)
    }

    fn get_description(&self) -> &'static str {
        "generates an unsigned integer between 0 and the given maximum"
    }

    fn create(&self, mut args: Vec<GeneratorArg>) -> GeneratorArg {
        let max = args.pop().unwrap().as_uint().unwrap();
        GeneratorArg::UnsignedInt(UnsignedIntGenerator::with_max(max))
    }
}

pub struct UnsignedInt2;
impl FunctionCreator for UnsignedInt2 {
    fn get_name(&self) -> &'static str {
        "uint"
    }

    fn get_arg_types(&self) -> (&'static [GeneratorType], bool) {
        (&[GeneratorType::UnsignedInt, GeneratorType::UnsignedInt], false)
    }

    fn get_description(&self) -> &'static str {
        "generates an unsigned integer within the given range"
    }

    fn create(&self, mut args: Vec<GeneratorArg>) -> GeneratorArg {
        let max = args.pop().unwrap().as_uint().unwrap();
        let min = args.pop().unwrap().as_uint().unwrap();

        GeneratorArg::UnsignedInt(UnsignedIntGenerator::new(min, max))
    }
}