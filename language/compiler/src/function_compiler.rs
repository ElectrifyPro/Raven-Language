use std::collections::HashMap;
use inkwell::types::{AsTypeRef, FunctionType};
use inkwell::values::FunctionValue;
use llvm_sys::core::LLVMFunctionType;
use llvm_sys::prelude::LLVMTypeRef;
use ast::code::Effects;
use ast::function::Function;
use crate::compiler::Compiler;
use crate::types::{TypeManager, Value};

pub fn compile_function<'ctx>(function: &Function, compiler: &Compiler<'ctx>) -> FunctionValue<'ctx> {
    let return_type = match &function.return_type {
        Some(found) => *compiler.get_type(&found.value),
        None => compiler.context.void_type().as_type_ref()
    };

    let mut params: Vec<LLVMTypeRef> = function.fields.iter().map(|field| *compiler.get_type(&field.field_type.value)).collect();

    let fn_type = unsafe { FunctionType::new(
        LLVMFunctionType(return_type, params.as_mut_ptr(), params.len() as u32, false as i32)) };

    let fn_value = compiler.module.add_function(function.name.value.as_str(), fn_type, None);

    let block = compiler.context.append_basic_block(fn_value, "entry");
    compiler.builder.position_at_end(block);

    if !function.code.expressions.last().map_or(false, |expression| expression.effect.unwrap().is_return()) {
        match &function.return_type {
            Some(return_type) => panic!("Missing return type {} for function {}", return_type, function.name),
            None => compiler.builder.build_return(None)
        };
    }
    return fn_value;
}

pub fn compile_block<'ctx>(function: &Function, types: &TypeManager, variables: &HashMap<String, Value<'ctx>>, compiler: &Compiler<'ctx>) {
    for line in &function.code.expressions {
        compile_effect(function, compiler, types, variables, &line.effect);
    }
}

pub fn compile_effect<'ctx>(function: &Function, compiler: &Compiler<'ctx>, types: &TypeManager,
                            variables: &HashMap<String, Value<'ctx>>, effect: &Effects) -> Value<'ctx> {
    return match effect {
        Effects::IntegerEffect(effect) =>
            Value::new(types.get_type("i64").unwrap().as_ref(),
                       Box::new(compiler.context.i64_type().const_int(effect.number, true))),
        Effects::FloatEffect(effect) =>
            Value::new(types.get_type("f64").unwrap().as_ref(),
                       Box::new(compiler.context.f64_type().const_float(effect.number))),
        Effects::MethodCall(effect) => panic!("Method calls not implemented yet!"),
        Effects::VariableLoad(effect) =>
            variables.get(&effect.name.value).expect(format!("Unknown variable called {}", effect.name.value).as_str()).clone(),
        Effects::ReturnEffect(effect) => {
            let value = compile_effect(function, compiler, types, variables, &effect.effect);
            compiler.builder.build_return(Some(value.value.as_ref()));

            value
        },
        Effects::MathEffect(effect) => {
            let value = compile_effect(function, compiler, types, variables, &effect.effect);
            value.value_type.math_operation(effect.operator, compiler, value,
                                            compile_effect(function, compiler, types, variables, &effect.target))
        }
    };
}