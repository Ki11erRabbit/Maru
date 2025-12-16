use std::ptr::NonNull;

use refcounter::RefCounter;

mod tables;
mod linker;
mod allocator;

pub type StringSymbol = u32;
pub type TypeSymbol = u32;
pub type FunctionSymbol = u32;
pub type FunctionPtr = extern "C" fn ();

#[repr(C)]
pub struct Metadata {
    refcount : RefCounter,
    type_id: u32,
}


#[repr(C)]
pub struct StackFrameCore {
    prev: Option<NonNull<StackFrame>>,
    next: Option<NonNull<StackFrame>>,
    variables_len: usize,
    variables: *mut u64,
}

#[repr(C)]
pub struct StackFrame {
    metadata: Metadata,
    core: StackFrameCore,
}