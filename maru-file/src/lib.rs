


pub type StringIndex = u32;
pub type BytecodeIndex = i32;

pub enum MaruTypeTag {
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
    Object(StringIndex),
}

/// A Maru file.
/// 
/// This struct represents a loaded Maru file.
pub struct MaruFile {
    /// The magic number of the Maru file.
    /// 
    /// This is always `0x4D` (the ASCII code for 'M').
    pub magic: u8,
    pub major_version: u8,
    pub minor_version: u8,
    pub patch_version: u8,
    pub module_name: StringIndex,
    pub objects: Vec<MaruObject>,
    pub functions: Vec<MaruFunction>,
    pub globals: Vec<MaruGlobal>,
    pub string_table: StringTable,
    pub bytecode_table: BytecodeTable,
    pub locations_map: LocationsMap,
}

/// A Maru object.
/// 
/// This struct represents an sum type in a Maru file.
pub struct MaruObject {
    /// The name of the type.
    /// 
    /// This would be something like `Option<T>`.
    pub name: StringIndex,
    /// The name of the type if it was monomorphized.
    /// 
    /// This would be something like `Option<i32>`.
    /// However, if the type is not monomorphized, then this would be the same as `name`.
    pub type_name: StringIndex,
    pub variants: Vec<MaruVariant>,
    /// This is for indicating if the type is referring to an internal type.
    /// If this is `0`, then the type is not internal.
    /// If this is `1` or greater, then the type is internal.
    pub internal: u32,
}

/// A Sum type variant.
/// 
/// This struct represents a variant of a sum type in a Maru file.
pub struct MaruVariant {
    /// The name of the variant.
    pub name: StringIndex,
    /// The name of the variant if it was monomorphized.
    pub type_name: StringIndex,
    /// The members of the variant.
    /// 
    /// Each member is a tuple of the member's name and its type.
    pub members: Vec<(StringIndex, MaruTypeTag)>,
}

/// A Maru function.
/// 
/// This struct represents a function in a Maru file.
pub struct MaruFunction {
    /// The name of the function.
    /// 
    /// This would be something like `main`.
    pub name: StringIndex,
    /// The name of the function if it was monomorphized.
    /// 
    /// This would be something like `main<i32>`.
    /// However, if the function is not monomorphized, then this would be the same as `name`.
    pub type_name: StringIndex,
    /// The parameters of the function.
    pub parameters: Vec<MaruTypeTag>,
    /// The return type of the function.
    pub return_type: MaruTypeTag,
    /// The index of the bytecode in the bytecode table.
    /// If this is `-1` or less, then the function is an internal function.
    pub bytecode_index: BytecodeIndex,
    /// The number of local variables in the function.
    /// 
    /// This is the number of registers used by the function.
    /// If this is `0`, then the function takes 0 arguments and has 0 local variables.
    /// This is should be the same as the number of parameters plus the number of local variables.
    /// 
    /// This may be zero if the function is an internal function.
    pub variables: u32,
}

/// A Maru global variable.
/// 
/// This struct represents a global variable in a Maru file.
pub struct MaruGlobal {
    pub name: StringIndex,
    pub type_tag: MaruTypeTag,
    /// The index of the initialization expression in the bytecode table.
    /// If this is `-1` or less, then then this global refers to an internal global.
    pub init_index: BytecodeIndex,
}

/// A string table.
/// 
/// This struct represents a table of strings in a Maru file.
pub struct StringTable {
    pub entries: Vec<String>,
}

/// A bytecode table.
/// 
/// This struct represents a table of bytecode in a Maru file.
pub struct BytecodeTable {
    pub entries: Vec<Box<[u8]>>,
}

/// A locations map.
/// 
/// This struct mirrors the `BytecodeTable` but instead of containing bytecode, it contains locations in the source code.
pub struct LocationsMap {
    pub entries: Vec<MaruLocation>,
}

/// A location in the source code.
pub struct MaruLocation {
    pub file: StringIndex,
    pub locations: Box<[(u32, u32)]>,
}