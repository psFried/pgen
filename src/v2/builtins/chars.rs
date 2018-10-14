use failure::Error;
use std::rc::Rc;
use v2::{
    AnyFunction, BuiltinFunctionPrototype, FunctionPrototype, CreateFunctionResult, DataGenOutput, DynUintFun,
    ProgramContext, RunnableFunction, GenType, Arguments
};

#[derive(Debug)]
struct CharGenerator {
    min_inclusive: DynUintFun,
    max_exclusive: DynUintFun,
}

impl RunnableFunction<char> for CharGenerator {
    fn gen_value(&self, context: &mut ProgramContext) -> Result<char, Error> {
        let min = self.min_inclusive.gen_value(context)?;
        let max = self.max_exclusive.gen_value(context)?;

        let as_u64 = context.gen_range(min, max);

        ::std::char::from_u32(as_u64 as u32).ok_or_else(|| {
            format_err!("Invalid unicode codepoint: {}, generated from range: min_inclusive: {}, max_exclusive: {}", as_u64, min, max)
        })
    }

    fn write_value(
        &self,
        context: &mut ProgramContext,
        output: &mut DataGenOutput,
    ) -> Result<u64, Error> {
        let value = self.gen_value(context)?;
        output.write(&value)
    }
}

const MIN_ARG: &str = "min_inclusive";
const MAX_ARG: &str = "max_inclusive";

fn create_char_gen(args: Arguments) -> CreateFunctionResult {
    #[cfg_attr(rustfmt, rustfmt_skip)]
    let (min, max) = args.require_2_args(
        MIN_ARG, AnyFunction::require_uint,
        MAX_ARG, AnyFunction::require_uint,
    )?;

    Ok(AnyFunction::Char(Rc::new(CharGenerator {
        min_inclusive: min,
        max_exclusive: max
    })))
}

pub const CHAR_GEN_BUILTIN: &'static FunctionPrototype = &FunctionPrototype::Builtin(&BuiltinFunctionPrototype {
    function_name: "char",
    description: "selects a single random character from within the provided range of unicode codepoints",
    arguments: &[(MIN_ARG, GenType::Uint), (MAX_ARG, GenType::Uint)],
    variadic: false,
    create_fn: &create_char_gen,
});
