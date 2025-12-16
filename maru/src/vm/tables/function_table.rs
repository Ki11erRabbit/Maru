use std::{cell::UnsafeCell, sync::{Arc, atomic::AtomicUsize}};

use crate::vm::{FunctionPtr, StringSymbol, VmType};

pub enum GetFunctionResult {
    Ptr(FunctionPtr),
    Bytecode(&'static [u8])
}

pub enum FunctionData {
    Bytecode(Box<[u8]>),
    Native,
}

pub struct Function {
    pub name: StringSymbol,
    pub type_name: StringSymbol,
    pub parameters: Box<[VmType]>,
    pub return_type: VmType,
    pub function: FunctionData,
    pub variable_count: u32,
    call_counter: Arc<AtomicUsize>,
    function_ptr: UnsafeCell<Option<FunctionPtr>>,
}

impl Function {


    pub fn set_function_ptr(&self, ptr: FunctionPtr) {
        unsafe {
            match self.function_ptr.get().as_mut().unwrap() {
                Some(_) => panic!("Function pointer already set"),
                x => {
                    *x = Some(ptr);
                }
            }
        }
    }

    pub fn get_function(&self) -> GetFunctionResult {
        unsafe {
            match self.function_ptr.get().as_mut().unwrap() {
                Some(ptr) => {
                    GetFunctionResult::Ptr(*ptr)
                },
                _ => {
                    match &self.function {
                        FunctionData::Native => todo!("load a builtin function"),
                        FunctionData::Bytecode(code) => {
                            let len = code.len();
                            let code = code.as_ptr().as_ref().unwrap();
                            let slice = std::slice::from_raw_parts(code, len);
                            GetFunctionResult::Bytecode(slice)
                        }
                    }
                }
            }
        }
    }
}