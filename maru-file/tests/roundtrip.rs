use maru_file::*;

#[test]
fn test_maru_type_tag_roundtrip() {
    let tags = vec![
        MaruTypeTag::Unit,
        MaruTypeTag::Bool,
        MaruTypeTag::U8,
        MaruTypeTag::I8,
        MaruTypeTag::U16,
        MaruTypeTag::I16,
        MaruTypeTag::U32,
        MaruTypeTag::I32,
        MaruTypeTag::U64,
        MaruTypeTag::I64,
        MaruTypeTag::F32,
        MaruTypeTag::F64,
        MaruTypeTag::Object(42),
    ];
    for tag in tags {
        let b = tag.into_binary();
        let (tag2, rest) = MaruTypeTag::from_binary(&b).expect("from_binary failed");
        assert!(rest.is_empty(), "remaining bytes should be empty");
        assert_eq!(b, tag2.into_binary());
    }
}

#[test]
fn test_variant_object_function_global_stringtable_locations_bytecode_roundtrip() {
    // Variant
    let variant = MaruVariant {
        name: 1,
        type_name: 2,
        members: vec![(10, MaruTypeTag::U32)],
    };
    let vb = variant.into_binary();
    let (variant2, rest) = MaruVariant::from_binary(&vb).expect("variant from_binary");
    assert!(rest.is_empty());
    assert_eq!(vb, variant2.into_binary());

    // Object
    let (variant3, _) = MaruVariant::from_binary(&vb).expect("reparse variant for object");
    let object = MaruObject { name: 5, type_name: 6, variants: vec![variant3], internal: 1 };
    let ob = object.into_binary();
    let (object2, rest) = MaruObject::from_binary(&ob).expect("object from_binary");
    assert!(rest.is_empty());
    assert_eq!(ob, object2.into_binary());

    // Function
    let function = MaruFunction { name: 7, type_name: 8, parameters: vec![MaruTypeTag::U8, MaruTypeTag::F64], return_type: MaruTypeTag::I32, bytecode_index: -1, variables: 0 };
    let fb = function.into_binary();
    let (function2, rest) = MaruFunction::from_binary(&fb).expect("function from_binary");
    assert!(rest.is_empty());
    assert_eq!(fb, function2.into_binary());

    // Global
    let global = MaruGlobal { name: 9, type_tag: MaruTypeTag::Bool, init_index: -1 };
    let gb = global.into_binary();
    let (global2, rest) = MaruGlobal::from_binary(&gb).expect("global from_binary");
    assert!(rest.is_empty());
    assert_eq!(gb, global2.into_binary());

    // StringTable
    let st = StringTable { entries: vec!["hello".into(), "world".into()] };
    let stb = st.into_binary();
    let (st2, rest) = StringTable::from_binary(&stb).expect("string table from_binary");
    assert!(rest.is_empty());
    assert_eq!(stb, st2.into_binary());

    // BytecodeTable
    let bt = BytecodeTable { entries: vec![vec![1u8,2,3].into_boxed_slice(), vec![].into_boxed_slice()] };
    let btb = bt.into_binary();
    let (bt2, rest) = BytecodeTable::from_binary(&btb).expect("bytecode table from_binary");
    assert!(rest.is_empty());
    assert_eq!(btb, bt2.into_binary());

    // Location & LocationsMap
    let loc = MaruLocation::new(3, vec![(1,2), (3,4)]);
    let lb = loc.into_binary();
    let (loc2, rest) = MaruLocation::from_binary(&lb).expect("location from_binary");
    assert!(rest.is_empty());
    assert_eq!(lb, loc2.into_binary());

    let (loc3, _) = MaruLocation::from_binary(&lb).expect("reparse location for map");
    let lm = LocationsMap { entries: vec![loc3] };
    let lmb = lm.into_binary();
    let (lm2, rest) = LocationsMap::from_binary(&lmb).expect("locations map from_binary");
    assert!(rest.is_empty());
    assert_eq!(lmb, lm2.into_binary());
}

#[test]
fn test_maru_file_roundtrip_stability() {
    let mut file = MaruFile::new();
    file.major_version = 1;
    file.minor_version = 2;
    file.patch_version = 3;
    file.module_name = file.add_string("mod".into());

    let s1 = file.add_string("one".into());
    let s2 = file.add_string("two".into());

    let bc_index = file.add_bytecode(vec![10u8,20,30].into_boxed_slice());
    let _loc_index = file.add_location(MaruLocation::new(s1, vec![(0,1)]));

    file.add_global(MaruGlobal { name: s2, type_tag: MaruTypeTag::U8, init_index: bc_index });

    file.add_function(MaruFunction { name: s1, type_name: s1, parameters: vec![MaruTypeTag::U32], return_type: MaruTypeTag::Unit, bytecode_index: bc_index, variables: 1 });

    let b1 = file.into_binary();
    let file2 = MaruFile::from_binary(&b1).expect("MaruFile from_binary");
    let b2 = file2.into_binary();
    assert_eq!(b1, b2);
}

#[test]
fn test_invalid_maru_type_tag_binary() {
    assert!(MaruTypeTag::from_binary(&[]).is_err());
}

#[test]
fn test_string_table_empty_and_empty_string() {
    let st_empty = StringTable { entries: vec![] };
    let b = st_empty.into_binary();
    let (st2, rest) = StringTable::from_binary(&b).expect("empty string table");
    assert!(rest.is_empty());
    assert_eq!(st2.entries.len(), 0);

    let st_single_empty = StringTable { entries: vec!["".into()] };
    let b2 = st_single_empty.into_binary();
    let (st3, rest) = StringTable::from_binary(&b2).expect("single empty string");
    assert!(rest.is_empty());
    assert_eq!(st3.entries, vec!["".to_string()]);
}

#[test]
fn test_bytecode_table_empty_entries() {
    let bt = BytecodeTable { entries: vec![] };
    let b = bt.into_binary();
    let (bt2, rest) = BytecodeTable::from_binary(&b).expect("empty bytecode table");
    assert!(rest.is_empty());
    assert_eq!(bt2.entries.len(), 0);
}

#[test]
fn test_invalid_magic_in_maru_file() {
    // bad magic (first byte != 0x4D)
    let bad = vec![0u8, 0, 0, 0, 0, 0, 0, 0];
    assert!(MaruFile::from_binary(&bad).is_err());
}

#[test]
fn test_corrupt_bytecode_entry_length() {
    // entries_len = 1, entry_len = 10 but only 2 bytes provided -> should error
    let mut v = Vec::new();
    v.extend_from_slice(&(1u32).to_le_bytes()); // entries_len
    v.extend_from_slice(&(10u32).to_le_bytes()); // entry_len
    v.extend_from_slice(&[1u8, 2u8]); // only 2 bytes of the entry
    let res = BytecodeTable::from_binary(&v);
    assert!(res.is_err());
    let err = res.err().unwrap();
    assert!(err.contains("BytecodeTable"), "unexpected error: {}", err);
}

#[test]
fn test_corrupt_variant_invalid_type_tag() {
    // Construct a variant binary with an invalid type tag (255)
    let mut v = Vec::new();
    v.extend_from_slice(&1u32.to_le_bytes()); // name
    v.extend_from_slice(&2u32.to_le_bytes()); // type_name
    v.extend_from_slice(&1u32.to_le_bytes()); // members_len
    v.extend_from_slice(&10u32.to_le_bytes()); // member name
    v.push(255u8); // invalid MaruTypeTag tag
    let res = MaruVariant::from_binary(&v);
    assert!(res.is_err());
    let err = res.err().unwrap();
    assert!(err.contains("Unknown MaruTypeTag tag"), "unexpected error: {}", err);
}

#[test]
fn test_corrupt_maru_file_missing_object() {
    // MaruFile header with objects_len = 2 but only one object present
    let mut file_bytes = Vec::new();
    file_bytes.push(0x4D); // magic
    file_bytes.push(0); // major
    file_bytes.push(0); // minor
    file_bytes.push(0); // patch
    file_bytes.extend_from_slice(&0u32.to_le_bytes()); // module_name
    file_bytes.extend_from_slice(&2u32.to_le_bytes()); // objects_len = 2

    // one valid small object: name,type_name,variants_len=0, internal=0
    let obj = MaruObject { name: 1, type_name: 1, variants: vec![], internal: 0 };
    file_bytes.extend_from_slice(&obj.into_binary());

    // no second object, attempt to continue with empty functions/globals/string tables
    file_bytes.extend_from_slice(&0u32.to_le_bytes()); // functions_len
    file_bytes.extend_from_slice(&0u32.to_le_bytes()); // globals_len
    // minimal empty string table
    file_bytes.extend_from_slice(&0u32.to_le_bytes());

    let res = MaruFile::from_binary(&file_bytes);
    assert!(res.is_err());
}

#[test]
fn test_corrupt_function_trailing_fields() {
    // name, type_name, parameters_len=0, return_type=U8, but missing bytecode_index/variables
    let mut v = Vec::new();
    v.extend_from_slice(&1u32.to_le_bytes());
    v.extend_from_slice(&1u32.to_le_bytes());
    v.extend_from_slice(&0u32.to_le_bytes()); // parameters_len
    v.extend_from_slice(&MaruTypeTag::U8.into_binary()); // return type
    // leave off the required 8 trailing bytes
    let res = MaruFunction::from_binary(&v);
    assert!(res.is_err());
    let err = res.err().unwrap();
    assert!(err.contains("MaruFunction"), "unexpected error: {}", err);
}
