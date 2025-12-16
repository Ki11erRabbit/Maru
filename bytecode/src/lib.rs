

pub enum Instruction {
    Load8,
    Load16,
    Load32,
    Load64,
    Loadf32,
    Loadf64,
    Copy,
    Clone,
    Move,
    Clear,
    Destroy,
    Forget,
    LoadReturn,
    FetchRef,
    MakeShared,
    SetGlobal,
    CopyGlobal,
    CloneGlobal,
    AddU,
    SubU,
    MulU,
    DivU,
    RemU,
    AddS,
    SubS,
    MulS,
    DivS,
    RemS,
    AddF,
    SubF,
    MulF,
    DivF,
    And,
    Or,
    Xor,
    Not,
    ShiftLeft,
    LogicalShiftRight,
    ArithmeticShiftRight,
    ByteSwap,
    EqI,
    NeqI,
    EqF,
    NeqF,
    LtU,
    GtU,
    LteU,
    GteU,
    LtS,
    GtS,
    LteS,
    GteS,
    LtF,
    GtF,
    LteF,
    GteF,
    CreateObject,
    IsNull,
    IsNaN,
    IsInfinity,
    GetField,
    CopyField,
    TakeField,
    SetField,
    MoveField,
    PlaceField,
    Call,
    CallTail,
    Invoke,
    InvokeTail,
    Return,
    ReturnTail,
    ReturnUnit,
    ReturnTailUnit,
    CreateClosure,
    Jump,
    If,
    Switch,
    Match,
    StartBlock
}

impl From<u8> for Instruction {
    fn from(value: u8) -> Self {
        match value {
            0 => Instruction::Load8,
            1 => Instruction::Load16,
            2 => Instruction::Load32,
            3 => Instruction::Load64,
            4 => Instruction::Loadf32,
            5 => Instruction::Loadf64,
            6 => Instruction::Copy,
            7 => Instruction::Clone,
            8 => Instruction::Move,
            9 => Instruction::Clear,
            10 => Instruction::Destroy,
            11 => Instruction::Forget,
            12 => Instruction::LoadReturn,
            13 => Instruction::FetchRef,
            14 => Instruction::MakeShared,
            15 => Instruction::SetGlobal,
            16 => Instruction::CopyGlobal,
            17 => Instruction::CloneGlobal,
            18 => Instruction::AddU,
            19 => Instruction::SubU,
            20 => Instruction::MulU,
            21 => Instruction::DivU,
            22 => Instruction::RemU,
            23 => Instruction::AddS,
            24 => Instruction::SubS,
            25 => Instruction::MulS,
            26 => Instruction::DivS,
            27 => Instruction::RemS,
            28 => Instruction::AddF,
            29 => Instruction::SubF,
            30 => Instruction::MulF,
            31 => Instruction::DivF,
            32 => Instruction::And,
            33 => Instruction::Or,
            34 => Instruction::Xor,
            35 => Instruction::Not,
            36 => Instruction::ShiftLeft,
            37 => Instruction::LogicalShiftRight,
            38 => Instruction::ArithmeticShiftRight,
            39 => Instruction::ByteSwap,
            40 => Instruction::EqI,
            41 => Instruction::NeqI,
            42 => Instruction::EqF,
            43 => Instruction::NeqF,
            44 => Instruction::LtU,
            45 => Instruction::GtU,
            46 => Instruction::LteU,
            47 => Instruction::GteU,
            48 => Instruction::LtS,
            49 => Instruction::GtS,
            50 => Instruction::LteS,
            51 => Instruction::GteS,
            52 => Instruction::LtF,
            53 => Instruction::GtF,
            54 => Instruction::LteF,
            55 => Instruction::GteF,
            56 => Instruction::CreateObject,
            57 => Instruction::IsNull,
            58 => Instruction::IsNaN,
            59 => Instruction::IsInfinity,
            60 => Instruction::GetField,
            61 => Instruction::CopyField,
            62 => Instruction::TakeField,
            63 => Instruction::SetField,
            64 => Instruction::MoveField,
            65 => Instruction::PlaceField,
            66 => Instruction::Call,
            67 => Instruction::CallTail,
            68 => Instruction::Invoke,
            69 => Instruction::InvokeTail,
            70 => Instruction::Return,
            71 => Instruction::ReturnTail,
            72 => Instruction::ReturnUnit,
            73 => Instruction::ReturnTailUnit,
            74 => Instruction::CreateClosure,
            75 => Instruction::Jump,
            76 => Instruction::If,
            77 => Instruction::Switch,
            78 => Instruction::Match,
            79 => Instruction::StartBlock,
            _ => panic!("Invalid instruction value"),
        }
    }
}

impl Into<u8> for Instruction {
    fn into(self) -> u8 {
        match self {
            Instruction::Load8 => 0,
            Instruction::Load16 => 1,
            Instruction::Load32 => 2,
            Instruction::Load64 => 3,
            Instruction::Loadf32 => 4,
            Instruction::Loadf64 => 5,
            Instruction::Copy => 6,
            Instruction::Clone => 7,
            Instruction::Move => 8,
            Instruction::Clear => 9,
            Instruction::Destroy => 10,
            Instruction::Forget => 11,
            Instruction::LoadReturn => 12,
            Instruction::FetchRef => 13,
            Instruction::MakeShared => 14,
            Instruction::SetGlobal => 15,
            Instruction::CopyGlobal => 16,
            Instruction::CloneGlobal => 17,
            Instruction::AddU => 18,
            Instruction::SubU => 19,
            Instruction::MulU => 20,
            Instruction::DivU => 21,
            Instruction::RemU => 22,
            Instruction::AddS => 23,
            Instruction::SubS => 24,
            Instruction::MulS => 25,
            Instruction::DivS => 26,
            Instruction::RemS => 27,
            Instruction::AddF => 28,
            Instruction::SubF => 29,
            Instruction::MulF => 30,
            Instruction::DivF => 31,
            Instruction::And => 32,
            Instruction::Or => 33,
            Instruction::Xor => 34,
            Instruction::Not => 35,
            Instruction::ShiftLeft => 36,
            Instruction::LogicalShiftRight => 37,
            Instruction::ArithmeticShiftRight => 38,
            Instruction::ByteSwap => 39,
            Instruction::EqI => 40,
            Instruction::NeqI => 41,
            Instruction::EqF => 42,
            Instruction::NeqF => 43,
            Instruction::LtU => 44,
            Instruction::GtU => 45,
            Instruction::LteU => 46,
            Instruction::GteU => 47,
            Instruction::LtS => 48,
            Instruction::GtS => 49,
            Instruction::LteS => 50,
            Instruction::GteS => 51,
            Instruction::LtF => 52,
            Instruction::GtF => 53,
            Instruction::LteF => 54,
            Instruction::GteF => 55,
            Instruction::CreateObject => 56,
            Instruction::IsNull => 57,
            Instruction::IsNaN => 58,
            Instruction::IsInfinity => 59,
            Instruction::GetField => 60,
            Instruction::CopyField => 61,
            Instruction::TakeField => 62,
            Instruction::SetField => 63,
            Instruction::MoveField => 64,
            Instruction::PlaceField => 65,
            Instruction::Call => 66,
            Instruction::CallTail => 67,
            Instruction::Invoke => 68,
            Instruction::InvokeTail => 69,
            Instruction::Return => 70,
            Instruction::ReturnTail => 71,
            Instruction::ReturnUnit => 72,
            Instruction::ReturnTailUnit => 73,
            Instruction::CreateClosure => 74,
            Instruction::Jump => 75,
            Instruction::If => 76,
            Instruction::Switch => 77,
            Instruction::Match => 78,
            Instruction::StartBlock => 79,
        }
    }
}

/// A target register
pub type Register = u32;
/// A identifier for a block, function, or other entity
pub type Id = u32;

/// A argument to the function call instruction
pub struct CallArgument {
    /// Whether or not to increment the reference count of the argument when passing it to the function
    pub increment_ref: bool,
    /// The register containing the argument value
    pub register: Register,
}

/// A branch option for the jump, if, switch, and match instructions
pub struct JumpBranch {
    pub block_id: Id,
    pub offset: i32,
}

/// A case for the switch instruction
pub struct SwitchCase {
    /// a constant value to compare against
    pub value: u64,
    pub branch: JumpBranch,
}

/// A case for the match instruction
pub struct MatchCase {
    /// The tag of the variant to match against
    pub tag: Id,
    pub branch: JumpBranch,
}

/// Decodes an instruction from a byte value
pub fn decode_instruction(value: u8) -> Instruction {
    value.into()
}

/// Decodes a length from a byte slice
pub fn decode_length(bytes: &[u8]) -> u32 {
    let length = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
    length
}

/// Decodes a `CallArgument` from a byte slice
pub fn decode_call_argument(bytes: &[u8]) -> CallArgument {
    let increment_ref = bytes[0] != 0;
    let register = u32::from_le_bytes([bytes[1], bytes[2], bytes[3], bytes[4]]);
    CallArgument { increment_ref, register }
}

/// Decodes a `JumpBranch` from a byte slice
pub fn decode_jump_branch(bytes: &[u8]) -> JumpBranch {
    let block_id = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
    let offset = i32::from_le_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]);
    JumpBranch { block_id, offset }
}

/// Decodes a `SwitchCase` from a byte slice
pub fn decode_switch_case(bytes: &[u8]) -> SwitchCase {
    let value = u64::from_le_bytes([
        bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
    ]);
    let branch = decode_jump_branch(&bytes[8..]);
    SwitchCase { value, branch }
}

/// Decodes a `MatchCase` from a byte slice
pub fn decode_match_case(bytes: &[u8]) -> MatchCase {
    let tag = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
    let branch = decode_jump_branch(&bytes[4..]);
    MatchCase { tag, branch }
}

/// Decodes an `Id` from a byte slice
pub fn decode_id(bytes: &[u8]) -> Id {
    u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
}