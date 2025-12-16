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
    refcount : RefCounter,
    type_id: TypeSymbol,
    variant_id: VariantId,
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