use std::alloc::Layout;

use crate::vm::{Metadata, StringSymbol, TypeSymbol};



pub struct ObjectDescription {
    pub name: StringSymbol,
    pub type_name: StringSymbol,
    pub size: usize,
    pub variants: Box<[VariantDescription]>,
    pub layout: Layout,
}

pub struct VariantDescription {
    pub variant_names: Box<[StringSymbol]>,
    /// Offsets from the start of the data section of an object.
    /// 
    /// Due to tight packing, members are out of order but this
    /// field gives you the correct offset into the pointer for each member.
    pub packing_offsets: Box<[usize]>,
}


pub struct ObjectDescTable {
    table: Vec<ObjectDescription>
}

impl std::ops::Index<TypeSymbol> for ObjectDescTable {
    type Output = ObjectDescription;
    fn index(&self, index: TypeSymbol) -> &Self::Output {
        &self.table[index as usize]
    }
}

pub struct Object {
    pub metadata: Metadata,
}