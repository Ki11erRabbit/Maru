use std::vec;




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

impl MaruTypeTag {
    pub fn into_binary(self) -> Vec<u8> {
        match self {
            MaruTypeTag::Unit => vec![0],
            MaruTypeTag::Bool => vec![1],
            MaruTypeTag::U8 => vec![2],
            MaruTypeTag::I8 => vec![3],
            MaruTypeTag::U16 => vec![4],
            MaruTypeTag::I16 => vec![5],
            MaruTypeTag::U32 => vec![6],
            MaruTypeTag::I32 => vec![7],
            MaruTypeTag::U64 => vec![8],
            MaruTypeTag::I64 => vec![9],
            MaruTypeTag::F32 => vec![10],
            MaruTypeTag::F64 => vec![11],
            MaruTypeTag::Object(index) => {
                let mut bytes = vec![12];
                bytes.extend_from_slice(&index.to_le_bytes());
                bytes
            }
        }
    }

    pub fn from_binary(binary: &[u8]) -> Result<(Self, &[u8]), String> {
        if binary.is_empty() {
            return Err("Binary is too short to contain a valid MaruTypeTag".to_string());
        }
        let tag = binary[0];
        let (tag, rest) = match tag {
            0 => (MaruTypeTag::Unit, &binary[1..]),
            1 => (MaruTypeTag::Bool, &binary[1..]),
            2 => (MaruTypeTag::U8, &binary[1..]),
            3 => (MaruTypeTag::I8, &binary[1..]),
            4 => (MaruTypeTag::U16, &binary[1..]),
            5 => (MaruTypeTag::I16, &binary[1..]),
            6 => (MaruTypeTag::U32, &binary[1..]),
            7 => (MaruTypeTag::I32, &binary[1..]),
            8 => (MaruTypeTag::U64, &binary[1..]),
            9 => (MaruTypeTag::I64, &binary[1..]),
            10 => (MaruTypeTag::F32, &binary[1..]),
            11 => (MaruTypeTag::F64, &binary[1..]),
            12 => {
                if binary.len() < 5 {
                    return Err("Binary is too short to contain a valid MaruTypeTag::Object".to_string());
                }
                let index = u32::from_le_bytes([binary[1], binary[2], binary[3], binary[4]]);
                (MaruTypeTag::Object(index), &binary[5..])
            }
            _ => return Err(format!("Unknown MaruTypeTag tag: {}", tag)),
        };
        Ok((tag, rest))
    }
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

impl MaruObject {
    pub fn into_binary(self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&self.name.to_le_bytes());
        bytes.extend_from_slice(&self.type_name.to_le_bytes());
        bytes.extend_from_slice(&(self.variants.len() as u32).to_le_bytes());
        for variant in self.variants {
            bytes.extend_from_slice(&variant.into_binary());
        }
        bytes.extend_from_slice(&self.internal.to_le_bytes());
        bytes
    }

    pub fn from_binary(binary: &[u8]) -> Result<(Self, &[u8]), String> {
        if binary.len() < 12 {
            return Err("Binary is too short to contain a valid MaruObject".to_string());
        }
        let name = u32::from_le_bytes([binary[0], binary[1], binary[2], binary[3]]);
        let type_name = u32::from_le_bytes([binary[4], binary[5], binary[6], binary[7]]);
        let variants_len = u32::from_le_bytes([binary[8], binary[9], binary[10], binary[11]]);
        let mut binary = &binary[12..];
        let mut variants = Vec::new();
        for _ in 0..variants_len {
            let (variant, new_binary) = MaruVariant::from_binary(binary)?;
            variants.push(variant);
            binary = new_binary;
        }
        if binary.len() < 4 {
            return Err("Binary is too short to contain a valid MaruObject internal field".to_string());
        }
        let internal = u32::from_le_bytes([binary[0], binary[1], binary[2], binary[3]]);
        let binary = &binary[4..];
        Ok((MaruObject { name, type_name, variants, internal }, binary))
    }
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

impl MaruVariant {
    pub fn into_binary(self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&self.name.to_le_bytes());
        bytes.extend_from_slice(&self.type_name.to_le_bytes());
        bytes.extend_from_slice(&(self.members.len() as u32).to_le_bytes());
        for (name, type_tag) in self.members {
            bytes.extend_from_slice(&name.to_le_bytes());
            bytes.extend_from_slice(&type_tag.into_binary());
        }
        bytes
    }

    pub fn from_binary(binary: &[u8]) -> Result<(Self, &[u8]), String> {
        if binary.len() < 8 {
            return Err("Binary is too short to contain a valid MaruVariant".to_string());
        }
        let name = u32::from_le_bytes([binary[0], binary[1], binary[2], binary[3]]);
        let type_name = u32::from_le_bytes([binary[4], binary[5], binary[6], binary[7]]);
        let members_len = u32::from_le_bytes([binary[8], binary[9], binary[10], binary[11]]);
        let mut binary = &binary[12..];
        let mut members = Vec::new();
        for _ in 0..members_len {
            let member_name = u32::from_le_bytes([binary[0], binary[1], binary[2], binary[3]]);
            let (type_tag, new_binary) = MaruTypeTag::from_binary(&binary[4..])?;
            members.push((member_name, type_tag));
            binary = new_binary;
        }
        Ok((MaruVariant { name, type_name, members }, binary))
    }
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

impl MaruFunction {
    pub fn into_binary(self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&self.name.to_le_bytes());
        bytes.extend_from_slice(&self.type_name.to_le_bytes());
        bytes.extend_from_slice(&(self.parameters.len() as u32).to_le_bytes());
        for param in self.parameters {
            bytes.extend_from_slice(&param.into_binary());
        }
        bytes.extend_from_slice(&self.return_type.into_binary());
        bytes.extend_from_slice(&self.bytecode_index.to_le_bytes());
        bytes.extend_from_slice(&self.variables.to_le_bytes());
        bytes
    }

    pub fn from_binary(binary: &[u8]) -> Result<(Self, &[u8]), String> {
        if binary.len() < 12 {
            return Err("Binary is too short to contain a valid MaruFunction".to_string());
        }
        let name = u32::from_le_bytes([binary[0], binary[1], binary[2], binary[3]]);
        let type_name = u32::from_le_bytes([binary[4], binary[5], binary[6], binary[7]]);
        let parameters_len = u32::from_le_bytes([binary[8], binary[9], binary[10], binary[11]]);
        let mut binary = &binary[12..];
        let mut parameters = Vec::new();
        for _ in 0..parameters_len {
            let (param, new_binary) = MaruTypeTag::from_binary(binary)?;
            parameters.push(param);
            binary = new_binary;
        }
        // Parse return type
        let (return_type, new_binary) = MaruTypeTag::from_binary(binary)?;
        if new_binary.len() < 8 {
            return Err("Binary is too short to contain a valid MaruFunction trailing fields".to_string());
        }
        let bytecode_index = i32::from_le_bytes([new_binary[0], new_binary[1], new_binary[2], new_binary[3]]);
        let variables = u32::from_le_bytes([new_binary[4], new_binary[5], new_binary[6], new_binary[7]]);
        let binary = &new_binary[8..];
        Ok((MaruFunction { name, type_name, parameters, return_type, bytecode_index, variables }, binary))
    }
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

impl MaruGlobal {
    pub fn into_binary(self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&self.name.to_le_bytes());
        bytes.extend_from_slice(&self.type_tag.into_binary());
        bytes.extend_from_slice(&self.init_index.to_le_bytes());
        bytes
    }

    pub fn from_binary(binary: &[u8]) -> Result<(Self, &[u8]), String> {
        if binary.len() < 8 {
            return Err("Binary is too short to contain a valid MaruGlobal".to_string());
        }
        let name = u32::from_le_bytes([binary[0], binary[1], binary[2], binary[3]]);
        let (type_tag, new_binary) = MaruTypeTag::from_binary(&binary[4..])?;
        if new_binary.len() < 4 {
            return Err("Binary is too short to contain a valid MaruGlobal init_index".to_string());
        }
        let init_index = i32::from_le_bytes([new_binary[0], new_binary[1], new_binary[2], new_binary[3]]);
        let binary = &new_binary[4..];
        Ok((MaruGlobal { name, type_tag, init_index }, binary))
    }
}

/// A string table.
/// 
/// This struct represents a table of strings in a Maru file.
pub struct StringTable {
    pub entries: Vec<String>,
}

impl StringTable {
    pub fn into_binary(self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&(self.entries.len() as u32).to_le_bytes());
        for entry in self.entries {
            let mut entry_bytes = entry.into_bytes();
            entry_bytes.push(0); // Null-terminate the string
            bytes.extend_from_slice(&entry_bytes);
        }
        bytes
    }

    pub fn from_binary(binary: &[u8]) -> Result<(Self, &[u8]), String> {
        if binary.len() < 4 {
            return Err("Binary is too short to contain a valid StringTable".to_string());
        }
        let entries_len = u32::from_le_bytes([binary[0], binary[1], binary[2], binary[3]]);
        let mut binary = &binary[4..];
        let mut entries = Vec::new();
        for _ in 0..entries_len {
            let mut entry_bytes = Vec::new();
            for byte in binary {
                if *byte == 0 {
                    break;
                }
                entry_bytes.push(*byte);
            }
            binary = &binary[entry_bytes.len() + 1..];
            let entry = String::from_utf8(entry_bytes).map_err(|_| "Invalid UTF-8 in string table".to_string())?;
            entries.push(entry);
            
        }
        Ok((StringTable { entries }, binary))
    }
}

/// A bytecode table.
/// 
/// This struct represents a table of bytecode in a Maru file.
pub struct BytecodeTable {
    pub entries: Vec<Box<[u8]>>,
}

impl BytecodeTable {
    pub fn into_binary(self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&(self.entries.len() as u32).to_le_bytes());
        for entry in self.entries {
            bytes.extend_from_slice(&(entry.len() as u32).to_le_bytes());
            bytes.extend_from_slice(&entry);
        }
        bytes
    }

    pub fn from_binary(binary: &[u8]) -> Result<(Self, &[u8]), String> {
        if binary.len() < 4 {
            return Err("Binary is too short to contain a valid BytecodeTable".to_string());
        }
        let entries_len = u32::from_le_bytes([binary[0], binary[1], binary[2], binary[3]]);
        let mut binary = &binary[4..];
        let mut entries = Vec::new();
        for _ in 0..entries_len {
            if binary.len() < 4 {
                return Err("Binary is too short to contain a valid BytecodeTable entry".to_string());
            }
            let entry_len = u32::from_le_bytes([binary[0], binary[1], binary[2], binary[3]]);
            if binary.len() < (entry_len as usize + 4) {
                return Err("Binary is too short to contain a valid BytecodeTable entry".to_string());
            }
            let entry = binary[4..(entry_len as usize + 4)].to_vec().into_boxed_slice();
            entries.push(entry);
            binary = &binary[(entry_len as usize + 4)..];
        }
        Ok((BytecodeTable { entries }, binary))
    }
}

/// A locations map.
/// 
/// This struct mirrors the `BytecodeTable` but instead of containing bytecode, it contains locations in the source code.
pub struct LocationsMap {
    pub entries: Vec<MaruLocation>,
}

impl LocationsMap {
    pub fn into_binary(self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&(self.entries.len() as u32).to_le_bytes());
        for entry in self.entries {
            bytes.extend_from_slice(&entry.into_binary());
        }
        bytes
    }

    pub fn from_binary(binary: &[u8]) -> Result<(Self, &[u8]), String> {
        if binary.len() < 4 {
            return Err("Binary is too short to contain a valid LocationsMap".to_string());
        }
        let entries_len = u32::from_le_bytes([binary[0], binary[1], binary[2], binary[3]]);
        let mut binary = &binary[4..];
        let mut entries = Vec::new();
        for _ in 0..entries_len {
            let (entry, new_binary) = MaruLocation::from_binary(binary)?;
            entries.push(entry);
            binary = new_binary;
        }
        Ok((LocationsMap { entries }, binary))
    }
}

/// A location in the source code.
pub struct MaruLocation {
    pub file: StringIndex,
    pub locations: Vec<(u32, u32)>,
}

impl MaruLocation {
    pub fn new(file: StringIndex, locations: Vec<(u32, u32)>) -> Self {
        MaruLocation { file, locations }
    }

    pub fn add_location(&mut self, start: u32, end: u32) {
        self.locations.push((start, end));
    }

    pub fn into_binary(self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&self.file.to_le_bytes());
        bytes.extend_from_slice(&(self.locations.len() as u32).to_le_bytes());
        for (start, end) in self.locations {
            bytes.extend_from_slice(&start.to_le_bytes());
            bytes.extend_from_slice(&end.to_le_bytes());
        }
        bytes
    }

    pub fn from_binary(binary: &[u8]) -> Result<(Self, &[u8]), String> {
        if binary.len() < 8 {
            return Err("Binary is too short to contain a valid MaruLocation".to_string());
        }
        let file = u32::from_le_bytes([binary[0], binary[1], binary[2], binary[3]]);
        let locations_len = u32::from_le_bytes([binary[4], binary[5], binary[6], binary[7]]);
        let mut binary = &binary[8..];
        let mut locations = Vec::new();
        for _ in 0..locations_len {
            if binary.len() < 8 {
                return Err("Binary is too short to contain a valid MaruLocation".to_string());
            }
            let start = u32::from_le_bytes([binary[0], binary[1], binary[2], binary[3]]);
            let end = u32::from_le_bytes([binary[4], binary[5], binary[6], binary[7]]);
            locations.push((start, end));
            binary = &binary[8..];
        }
        Ok((MaruLocation { file, locations }, binary))
    }
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

impl MaruFile {
    pub fn new() -> Self {
        MaruFile {
            magic: 0x4D,
            major_version: 0,
            minor_version: 0,
            patch_version: 0,
            module_name: 0,
            objects: Vec::new(),
            functions: Vec::new(),
            globals: Vec::new(),
            string_table: StringTable { entries: Vec::new() },
            bytecode_table: BytecodeTable { entries: Vec::new() },
            locations_map: LocationsMap { entries: Vec::new() },
        }
    }

    pub fn from_binary(binary: &[u8]) -> Result<Self, String> {
        if binary.len() <= 4 {
            return Err("Binary is too short to contain a valid Maru file".to_string());
        }
        let magic = binary[0];
        if magic != 0x4D {
            return Err("Invalid magic number".to_string());
        }
        let major_version = binary[1];
        let minor_version = binary[2];
        let patch_version = binary[3];

        let module_name = u32::from_le_bytes([binary[4], binary[5], binary[6], binary[7]]);
        let binary = &binary[8..];
        let objects_len = u32::from_le_bytes([binary[0], binary[1], binary[2], binary[3]]);
        let mut binary = &binary[4..];
        let mut objects = Vec::new();
        for _ in 0..objects_len {
            let (object, new_binary) = MaruObject::from_binary(binary)?;
            objects.push(object);
            binary = new_binary;
        }
        let functions_len = u32::from_le_bytes([binary[0], binary[1], binary[2], binary[3]]);
        let mut binary = &binary[4..];
        let mut functions = Vec::new();
        for _ in 0..functions_len {
            let (function, new_binary) = MaruFunction::from_binary(binary)?;
            functions.push(function);
            binary = new_binary;
        }

        let globals_len = u32::from_le_bytes([binary[0], binary[1], binary[2], binary[3]]);
        let mut binary = &binary[4..];
        let mut globals = Vec::new();
        for _ in 0..globals_len {
            let (global, new_binary) = MaruGlobal::from_binary(binary)?;
            globals.push(global);
            binary = new_binary;
        }

        let (string_table, binary) = StringTable::from_binary(binary)?;

        let (bytecode_table, binary) = BytecodeTable::from_binary(binary)?;
        let (locations_map, _) = LocationsMap::from_binary(binary)?;
        Ok(MaruFile {
            magic,
            major_version,
            minor_version,
            patch_version,
            module_name,
            objects,
            functions,
            globals,
            string_table,
            bytecode_table,
            locations_map,
        })
    }

    pub fn add_object(&mut self, object: MaruObject) {
        self.objects.push(object);
    }

    pub fn add_function(&mut self, function: MaruFunction) {
        self.functions.push(function);
    }

    pub fn add_global(&mut self, global: MaruGlobal) {
        self.globals.push(global);
    }

    pub fn add_string(&mut self, string: String) -> StringIndex {
        let index = self.string_table.entries.len() as StringIndex;
        self.string_table.entries.push(string);
        index
    }

    pub fn add_bytecode(&mut self, bytecode: Box<[u8]>) -> BytecodeIndex {
        let index = self.bytecode_table.entries.len() as BytecodeIndex;
        self.bytecode_table.entries.push(bytecode);
        index
    }

    pub fn add_location(&mut self, location: MaruLocation) -> BytecodeIndex {
        let index = self.locations_map.entries.len() as BytecodeIndex;
        self.locations_map.entries.push(location);
        index
    }

    pub fn get_string(&self, index: StringIndex) -> &str {
        &self.string_table.entries[index as usize]
    }

    pub fn get_bytecode(&self, index: BytecodeIndex) -> &[u8] {
        &self.bytecode_table.entries[index as usize]
    }

    pub fn get_location(&self, index: BytecodeIndex) -> &MaruLocation {
        &self.locations_map.entries[index as usize]
    }

    pub fn get_object(&self, name: StringIndex) -> Option<&MaruObject> {
        self.objects.iter().find(|obj| obj.name == name)
    }

    pub fn get_function(&self, name: StringIndex) -> Option<&MaruFunction> {
        self.functions.iter().find(|func| func.name == name)
    }

    pub fn get_global(&self, name: StringIndex) -> Option<&MaruGlobal> {
        self.globals.iter().find(|global| global.name == name)
    }

    pub fn into_binary(self) -> Vec<u8> {
        let mut output = vec![
            self.magic,
            self.major_version,
            self.minor_version,
            self.patch_version,
        ];

        // Write module name index
        output.extend_from_slice(&self.module_name.to_le_bytes());

        // Write objects
        output.extend_from_slice(&(self.objects.len() as u32).to_le_bytes());
        for obj in self.objects {
            output.extend_from_slice(&obj.into_binary());
        }

        // Write functions
        output.extend_from_slice(&(self.functions.len() as u32).to_le_bytes());
        for func in self.functions {
            output.extend_from_slice(&func.into_binary());
        }

        // Write globals
        output.extend_from_slice(&(self.globals.len() as u32).to_le_bytes());
        for global in self.globals {
            output.extend_from_slice(&global.into_binary());
        }

        // Write string table
        output.extend_from_slice(&self.string_table.into_binary());

        // Write bytecode table
        output.extend_from_slice(&self.bytecode_table.into_binary());

        // Write locations map
        output.extend_from_slice(&self.locations_map.into_binary());

        output
    }
}

// Tests moved to `tests/roundtrip.rs`