extern crate llvm_sys as sys;
extern crate llvm;

use llvm::prelude::*;
use llvm::target_machine::RelocMode;
use llvm::CodeGenOpt::Level;
use llvm::TargetMachine;
use llvm::execution_engine::ExecutionEngine;
use llvm::orc::ThreadSafeModule;
use llvm::passes::PassManagerBuilder;
use llvm::analysis::AnalysisPassManager;
use std::process::exit;

fn create_llvm_ir() {
    llvm::initialize_native_target();
    llvm::initialize_native_asmprinter();
    llvm::initialize_native_asmparser();
    llvm::initialize_native_disassembler();

    let context = llvm::Context::new();

    let module = llvm::Module::new("example_module", &context);

    create_add_function(&context, &module);

    create_subtract_function(&context, &module);

    create_conditional_function(&context, &module);

    if llvm::verify_module(module, llvm::PassManager::new()).is_err() {
        eprintln!("Module verification failed!");
        exit(1);
    }

    module.print_to_stderr();

    if let Err(e) = module.write_to_file("example_module.bc") {
        eprintln!("Failed to write bitcode file: {}", e);
    }
}

fn create_add_function(context: &llvm::Context, module: &llvm::Module) {
    let int_type = llvm::Type::int32_type(context);
    let params = vec![int_type, int_type];
    let fn_type = llvm::FunctionType::get(int_type, &params, false);
    let fn_value = module.add_function("add", fn_type, Some(llvm::Linkage::External));

    let entry = llvm::BasicBlock::append(context, fn_value, "entry");
    let builder = llvm::IRBuilder::new(context);
    builder.position_at_end(entry);

    let args = fn_value.params();
    let a = &args[0];
    let b = &args[1];

    let sum = builder.build_add(a, b, "tmp");
    builder.build_return(Some(&sum));
}

fn create_subtract_function(context: &llvm::Context, module: &llvm::Module) {
    let int_type = llvm::Type::int32_type(context);
    let params = vec![int_type, int_type];
    let fn_type = llvm::FunctionType::get(int_type, &params, false);
    let fn_value = module.add_function("subtract", fn_type, Some(llvm::Linkage::External));

    let entry = llvm::BasicBlock::append(context, fn_value, "entry");
    let builder = llvm::IRBuilder::new(context);
    builder.position_at_end(entry);

    let args = fn_value.params();
    let a = &args[0];
    let b = &args[1];

    let diff = builder.build_sub(a, b, "tmp");
    builder.build_return(Some(&diff));
}

fn create_conditional_function(context: &llvm::Context, module: &llvm::Module) {
    let int_type = llvm::Type::int32_type(context);
    let params = vec![int_type, int_type];
    let fn_type = llvm::FunctionType::get(int_type, &params, false);
    let fn_value = module.add_function("conditional", fn_type, Some(llvm::Linkage::External));

    let entry = llvm::BasicBlock::append(context, fn_value, "entry");
    let then_block = llvm::BasicBlock::append(context, fn_value, "then");
    let else_block = llvm::BasicBlock::append(context, fn_value, "else");
    let end_block = llvm::BasicBlock::append(context, fn_value, "end");

    let builder = llvm::IRBuilder::new(context);
    builder.position_at_end(entry);

    let args = fn_value.params();
    let a = &args[0];
    let b = &args[1];

    let cmp = builder.build_icmp(llvm::IntPredicate::SGT, a, b, "cmp");
    builder.build_conditional_branch(cmp, then_block, else_block);

    builder.position_at_end(then_block);
    let then_value = builder.build_add(a, b, "then_tmp");
    builder.build_unconditional_branch(end_block);

    builder.position_at_end(else_block);
    let else_value = builder.build_sub(a, b, "else_tmp");
    builder.build_unconditional_branch(end_block);

    builder.position_at_end(end_block);
    let phi = builder.build_phi(int_type, "phi_tmp");
    phi.add_incoming(&[(&then_value, then_block), (&else_value, else_block)]);
    builder.build_return(Some(&phi));
}

fn main() {
    create_llvm_ir();
}