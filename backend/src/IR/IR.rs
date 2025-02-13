use inkwell::context::Context;
use inkwell::builder::Builder;
use inkwell::module::Module;
use inkwell::execution_engine::{ExecutionEngine, JITFunction};
use inkwell::values::IntValue;
use inkwell::OptimizationLevel;

fn build_if_else(context: &Context, module: &Module, builder: &Builder, function: inkwell::values::FunctionValue) {
    let i32_type = context.i32_type();
    let entry = context.append_basic_block(function, "entry");
    let then_block = context.append_basic_block(function, "then");
    let else_block = context.append_basic_block(function, "else");
    let merge_block = context.append_basic_block(function, "merge");
    
    builder.position_at_end(entry);
    let condition = function.get_nth_param(0).unwrap().into_int_value();
    let cmp = builder.build_int_compare(inkwell::IntPredicate::NE, condition, i32_type.const_int(0, false), "ifcond");
    builder.build_conditional_branch(cmp, then_block, else_block);
    
    builder.position_at_end(then_block);
    let then_val = i32_type.const_int(42, false);
    builder.build_unconditional_branch(merge_block);
    
    builder.position_at_end(else_block);
    let else_val = i32_type.const_int(0, false);
    builder.build_unconditional_branch(merge_block);
    
    builder.position_at_end(merge_block);
    let phi = builder.build_phi(i32_type, "iftmp");
    phi.add_incoming(&[(&then_val, then_block), (&else_val, else_block)]);
    builder.build_return(Some(&phi.as_basic_value().into_int_value()));
}

fn build_while_loop(context: &Context, module: &Module, builder: &Builder, function: inkwell::values::FunctionValue) {
    let i32_type = context.i32_type();
    let entry = context.append_basic_block(function, "entry");
    let loop_block = context.append_basic_block(function, "loop");
    let after_loop_block = context.append_basic_block(function, "after_loop");
    
    builder.position_at_end(entry);
    let mut counter = function.get_nth_param(0).unwrap().into_int_value();
    builder.build_unconditional_branch(loop_block);
    
    builder.position_at_end(loop_block);
    let condition = builder.build_int_compare(inkwell::IntPredicate::SGT, counter, i32_type.const_int(0, false), "loop_cond");
    builder.build_conditional_branch(condition, loop_block, after_loop_block);
    
    counter = builder.build_int_sub(counter, i32_type.const_int(1, false), "dec_counter");
    builder.build_unconditional_branch(loop_block);
    
    builder.position_at_end(after_loop_block);
    builder.build_return(Some(&counter));
}

fn main() {
    let context = Context::create();
    let module = context.create_module("rust_compiler");
    let builder = context.create_builder();
    let execution_engine = module.create_jit_execution_engine(OptimizationLevel::None).unwrap();
    let i32_type = context.i32_type();
    
    let fn_type = i32_type.fn_type(&[i32_type.into()], false);
    let function_if = module.add_function("conditional_function", fn_type, None);
    let function_while = module.add_function("while_loop", fn_type, None);
    
    build_if_else(&context, &module, &builder, function_if);
    build_while_loop(&context, &module, &builder, function_while);
    
    module.print_to_stderr();
    
    unsafe {
        let conditional_fn: JITFunction<unsafe extern "C" fn(i32) -> i32> = execution_engine.get_function("conditional_function").unwrap();
        let result_true = conditional_fn.call(1);
        let result_false = conditional_fn.call(0);
        println!("Result for a true condition: {}", result_true);
        println!("Result for a false condition: {}", result_false);
        
        let while_fn: JITFunction<unsafe extern "C" fn(i32) -> i32> = execution_engine.get_function("while_loop").unwrap();
        let loop_result = while_fn.call(5);
        println!("Result of the while loop with 5: {}", loop_result);
    }
}

