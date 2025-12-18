use std::{alloc::Layout, sync::OnceLock};

use crate::vm::{Metadata, StringSymbol, TypeSymbol};


#[derive(Debug)]
pub struct ObjectDescription {
    pub name: StringSymbol,
    pub type_name: StringSymbol,
    pub size: usize,
    pub variants: Box<[VariantDescription]>,
    pub layout: Layout,
}

#[derive(Debug)]
pub struct VariantDescription {
    pub variant_names: Box<[StringSymbol]>,
    /// Offsets from the start of the data section of an object.
    /// 
    /// Due to tight packing, members are out of order but this
    /// field gives you the correct offset into the pointer for each member.
    pub packing_offsets: Box<[usize]>,
}

#[derive(Debug)]
pub struct ObjectDescTable {
    table: Vec<ObjectDescription>
}

impl ObjectDescTable  {
    pub fn new(max_type_symbol: TypeSymbol) -> Self {
        Self {
            table: Vec::with_capacity(max_type_symbol as usize)
        }
    }

    pub fn push_desc(&mut self, desc: ObjectDescription) {
        self.table.push(desc);
    }

    pub fn object_desc_table_init(self) {
        OBJECT_DESC_TABLE.set(self).expect("object_desc_table_init should only be called once")
    }

    pub fn get() -> &'static ObjectDescTable {
        OBJECT_DESC_TABLE.get().expect("object_desc_table_init was not called first")
    }
}

impl std::ops::Index<TypeSymbol> for ObjectDescTable {
    type Output = ObjectDescription;
    fn index(&self, index: TypeSymbol) -> &Self::Output {
        &self.table[index as usize]
    }
}

static OBJECT_DESC_TABLE: OnceLock<ObjectDescTable> = OnceLock::new();

pub struct Object {
    pub metadata: Metadata,
}