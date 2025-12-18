use std::ptr::NonNull;

use refcounter::RefCounter;

mod tables;
mod linker;
mod allocator;

pub type StringSymbol = u32;
pub type TypeSymbol = u32;
pub type VariantId = u32;
pub type FunctionSymbol = u32;
pub type FunctionPtr = extern "C" fn ();


pub enum VmType {
    Unit,
    Bool,
    U8,
    I8,
    U16,
    I16,
    U32,
    I32,
    U64,
    I64,
    F32,
    F64,
    Object(TypeSymbol)
}

#[repr(C)]
pub struct Metadata {
    pub refcount : RefCounter,
    pub type_id: TypeSymbol,
    pub variant_id: VariantId,
}


#[repr(C)]
pub struct StackFrameCore {
    pub prev: Option<NonNull<StackFrame>>,
    pub next: Option<NonNull<StackFrame>>,
    pub return_slot: u64,
    pub closure_slot: u64,
    pub variables_len: usize,
    pub variables: *mut u64,
    pub variables_type: *mut VmType,
}

impl StackFrameCore {
    pub fn new(variables_len: usize) -> StackFrameCore {
        use std::alloc::*;
        let layout = Layout::array::<u64>(variables_len).unwrap();
        let variables = unsafe { alloc(layout) as *mut u64 };
        if variables.is_null() {
            handle_alloc_error(layout);
        }
        let layout = Layout::array::<VmType>(variables_len).unwrap();
        let variables_type = unsafe { alloc(layout)  as *mut VmType };
        if variables_type.is_null() {
            handle_alloc_error(layout);
        }

        StackFrameCore { 
            prev: None, 
            next: None, 
            return_slot: 0, 
            closure_slot: 0, 
            variables_len, 
            variables, 
            variables_type
        }
    }

    pub fn free_memory(&mut self) {
        use std::alloc::{Layout, dealloc};
        let variables_len = self.variables_len;

        let layout = Layout::array::<u64>(variables_len).unwrap();
        unsafe { dealloc( self.variables as *mut u8, layout) };
        let layout = Layout::array::<VmType>(variables_len).unwrap();
        unsafe { dealloc(self.variables_type as *mut u8, layout) };
        self.variables = std::ptr::null_mut();
        self.variables = std::ptr::null_mut();
    }
}

#[repr(C)]
pub struct StackFrame {
    metadata: Metadata,
    core: StackFrameCore,
}