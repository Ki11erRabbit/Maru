use crate::vm::{Metadata, StringSymbol};



pub struct ObjectDescription {
    pub name: StringSymbol,
    pub type_name: StringSymbol,
    pub size: usize,
    pub variants: Box<[VariantDescription]>,
}

pub struct VariantDescription {
    pub variant_names: Box<[StringSymbol]>,
    /// Offsets from the start of the data section of an object.
    /// 
    /// Due to tight packing, members are out of order but this
    /// field gives you the correct offset into the pointer for each member.
    pub packing_offsets: Box<[usize]>,
}


pub struct Object {
    pub metadata: Metadata,
}