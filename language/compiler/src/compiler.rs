use std::collections::HashMap;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::execution_engine::{ExecutionEngine, JitFunction};
use inkwell::module::Module;
use inkwell::OptimizationLevel;
use inkwell::types::BasicTypeEnum;
use inkwell::values::FunctionValue;
use crate::file::FileStructureImpl;
use crate::function_compiler::compile_function;
use crate::types::TypeManager;

type Main = unsafe extern "C" fn() -> i64;

pub struct Compiler<'ctx> {
    pub context: &'ctx Context,
    pub module: Module<'ctx>,
    pub builder: Builder<'ctx>,
    execution_engine: ExecutionEngine<'ctx>,
    pub functions: HashMap<String, FunctionValue<'ctx>>,
    pub types: &'ctx TypeManager<'ctx>
}

impl<'ctx> Compiler<'ctx> {
    pub fn new(context: &'ctx Context, types: &'ctx TypeManager<'ctx>) -> Self {
        let module = context.create_module("main");
        let execution_engine = module.create_jit_execution_engine(OptimizationLevel::None).unwrap();
        return Self {
            context,
            module,
            builder: context.create_builder(),
            execution_engine,
            functions: HashMap::new(),
            types
        };
    }

    pub fn get_type(&self, name: &String) -> &BasicTypeEnum {
        return self.types.types.get(name).expect(&*("Couldn't find type named ".to_string() + name));
    }

    pub fn compile(&mut self, content: FileStructureImpl) -> Option<JitFunction<Main>> {
        let program = parser::parse(Box::new(content));

        match program.main {
            Some(main) => {
                let function = program.static_functions.get(main.as_str()).unwrap();
                self.functions.insert(function.name.value.clone(), compile_function(function, &self));
                let function =  unsafe { self.execution_engine.get_function("main::main") };
                return match function {
                    Ok(value) => Some(value),
                    Err(error) => panic!("{}", error)
                }
            },
            None => {}
        }

        return None;
    }
}