extern crate llvm_sys;

use llvm_sys::{LLVMBuilder, LLVMIntPredicate, LLVMModule};
use llvm_sys::bit_writer::*;
use llvm_sys::core::*;
use llvm_sys::core::*;
use llvm_sys::prelude::*;
use llvm_sys::target::*;
use llvm_sys::target_machine::*;
use llvm_sys::transforms::pass_manager_builder::*;
use std::ffi::{CStr, CString};

macro_rules! c_str {
    ($s:expr) => (
        concat!($s, "\0").as_ptr() as *const i8
    );
}

pub fn get_default_target_triple() -> CString {
    let target_triple;
    unsafe {
        let target_triple_ptr = LLVMGetDefaultTargetTriple();
        target_triple = CStr::from_ptr(target_triple_ptr as *const _).to_owned();
        LLVMDisposeMessage(target_triple_ptr);
    }

    target_triple
}

fn compile() {
    unsafe {
// setup
        let context = LLVMContextCreate();
        let module = LLVMModuleCreateWithName(c_str!("main"));
        let builder = LLVMCreateBuilderInContext(context);

// common types
        let void_type = LLVMVoidTypeInContext(context);
        let i8_type = LLVMIntTypeInContext(context, 8);
        let i8_pointer_type = LLVMPointerType(i8_type, 0);

// declare that there's a `void log(i8*)` function in the environment
// but don't provide a block (aka body) so that it in the wasm module
// it'll be imported
        let log_func_type = LLVMFunctionType(void_type, [i8_pointer_type].as_ptr() as *mut _, 1, 0);
        let log_func = LLVMAddFunction(module, c_str!("log"), log_func_type);

// our "main" function which we'll need to call explicitly from JavaScript
// after we've instantiated the WebAssembly.Instance
        let main_func_type = LLVMFunctionType(void_type, ptr::null_mut(), 0, 0);
        let main_func = LLVMAddFunction(module, c_str!("main"), main_func_type);
        let main_block = LLVMAppendBasicBlockInContext(context, main_func, c_str!("main"));
        LLVMPositionBuilderAtEnd(builder, main_block);

// main's function body
        let hello_world_str = LLVMBuildGlobalStringPtr(builder, c_str!("hello, world."), c_str!(""));
        let log_args = [hello_world_str].as_ptr() as *mut _;
// calling `log("hello, world.")`
        LLVMBuildCall(builder, log_func, log_args, 1, c_str!(""));
        LLVMBuildRetVoid(builder);

// write our bitcode file
// NOTE: you can read a bitcode file by using `llvm-dis <bc filename>.bc` and then looking at the resulting text file '<filename>.ll`
//let default_triple_cstring = get_default_target_triple();
//let default_triple = default_triple_cstring.to_str().unwrap();
        let target_triple_ptr = LLVMGetDefaultTargetTriple();

        LLVMSetTarget(module, target_triple_ptr);
        LLVMWriteBitcodeToFile(module, c_str!("main.bc"));

// clean up
        LLVMDisposeBuilder(builder);
        LLVMDisposeModule(module);
        LLVMContextDispose(context);
    }
}