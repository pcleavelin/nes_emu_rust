use std::fmt;

enum_from_primitive! {
    #[derive(Debug, PartialEq)]
    pub enum Op {
        BRKImmediate   = 0x00,
        ORAIndirectX   = 0x01,
        ORAZeroPage    = 0x05,
        ASLZeroPage    = 0x06,
        PHPImplied     = 0x08,
        ORAImmediate   = 0x09,
        ASLAccumulator = 0x0A,
        ORAAbsolute    = 0x0D,
        ASLAbsolute    = 0x0E,
        BPLRelative    = 0x10,
        ORAIndirectY   = 0x11,
        ORAZeroPageX   = 0x15,
        ASLZeroPageX   = 0x16,
        CLCImplied     = 0x18,
        ORAAbsoluteY   = 0x19,
        ORAAbsoluteX   = 0x1D,
        ASLAbsoluteX   = 0x1E,
        JSRAbsolute    = 0x20,
        BITZeroPage    = 0x24,
        ANDZeroPage    = 0x25,
        ROLZeroPage    = 0x26,
        PLPImplied     = 0x28,
        ANDImmediate   = 0x29,
        ROLAccumulator = 0x2A,
        BITAbsolute    = 0x2C,
        ANDAbsolute    = 0x2D,
        ROLAbsolute    = 0x2E,
        BMIRelative    = 0x30,
        ANDIndirectY   = 0x31,
        ANDZeroPageX   = 0x35,
        ROLZeroPageX   = 0x36,
        SECImplied     = 0x38,
        ANDAbsoluteY   = 0x39,
        ANDAbsoluteX   = 0x3D,
        ROLAbsoluteX   = 0x3E,
        RTIImplied     = 0x40,
        LSRZeroPage    = 0x46,
        EORZeroPage    = 0x45,
        PHAImplied     = 0x48,
        EORImmediate   = 0x49,
        LSRAccumulator = 0x4A,
        JMPAbsolute    = 0x4C,
        EORAbsolute    = 0x4D,
        LSRAbsolute    = 0x4E,
        BVCRelative    = 0x50,
        EORIndirectY   = 0x51,
        LSRZeroPageX   = 0x56,
        CLIImplied     = 0x58,
        EORAbsoluteY   = 0x59,
        RTSImplied     = 0x60,
        ADCIndirectX   = 0x61,
        ADCZeroPage    = 0x65,
        RORZeroPage    = 0x66,
        PLAImplied     = 0x68,
        ADCImmediate   = 0x69,
        RORAccumulator = 0x6A,
        JMPAbsIndirect = 0x6C,
        ADCAbsolute    = 0x6D,
        RORAbsolute    = 0x6E,
        BVSRelative    = 0x70,
        ADCIndirectY   = 0x71,
        ADCZeroPageX   = 0x75,
        RORZeroPageX   = 0x76,
        SEIImplied     = 0x78,
        ADCAbsoluteY   = 0x79,
        ADCAbsoluteX   = 0x7D,
        STAIndirectX   = 0x81,
        STYZeroPage    = 0x84,
        STAZeroPage    = 0x85,
        STXZeroPage    = 0x86,
        DEYImplied     = 0x88,
        TXAImplied     = 0x8A,
        STYAbsolute    = 0x8C,
        STXAbsolute    = 0x8E,
        STAAbsolute    = 0x8D,
        BCCRelative    = 0x90,
        STAIndirectY   = 0x91,
        STYZeroPageX   = 0x94,
        STAZeroPageX   = 0x95,
        STXZeroPageY   = 0x96,
        TYAImplied     = 0x98,
        STAAbsoluteY   = 0x99,
        TXSImplied     = 0x9A,
        STAAbsoluteX   = 0x9D,
        LDYImmediate   = 0xA0,
        LDAIndirectX   = 0xA1,
        LDXImmediate   = 0xA2,
        LDYZeroPage    = 0xA4,
        LDAZeroPage    = 0xA5,
        LDXZeroPage    = 0xA6,
        TAYImplied     = 0xA8,
        LDAImmediate   = 0xA9,
        TAXImplied     = 0xAA,
        LDAAbsolute    = 0xAD,
        LDYAbsolute    = 0xAC,
        LDXAbsolute    = 0xAE,
        BCSRelative    = 0xB0,
        LDAIndirectY   = 0xB1,
        LDYZeroPageX   = 0xB4,
        LDAZeroPageX   = 0xB5,
        LDXZeroPageY   = 0xB6,
        LDAAbsoluteY   = 0xB9,
        TSXImplied     = 0xBA,
        LDYAbsoluteX   = 0xBC,
        LDAAbsoluteX   = 0xBD,
        LDXAbsoluteY   = 0xBE,
        CPYImmediate   = 0xC0,
        CPYZeroPage    = 0xC4,
        CMPZeroPage    = 0xC5,
        DECZeroPage    = 0xC6,
        INYImplied     = 0xC8,
        CMPImmediate   = 0xC9,
        DEXImplied     = 0xCA,
        CPYAbsolute    = 0xCC,
        CMPAbsolute    = 0xCD,
        DECAbsolute    = 0xCE,
        BNERelative    = 0xD0,
        CMPIndirectY   = 0xD1,
        CMPZeroPageX   = 0xD5,
        DECZeroPageX   = 0xD6,
        CLDImplied     = 0xD8,
        CMPAbsoluteY   = 0xD9,
        CMPAbsoluteX   = 0xDD,
        DECAbsoluteX   = 0xDE,
        CPXImmediate   = 0xE0,
        CPXZeroPage    = 0xE4,
        SBCZeroPage    = 0xE5,
        INCZeroPage    = 0xE6,
        INXImplied     = 0xE8,
        SBCImmediate   = 0xE9,
        CPXAbsolute    = 0xEC,
        INCAbsolute    = 0xEE,
        SBCAbsolute    = 0xED,
        BEQRelative    = 0xF0,
        SBCIndirectY   = 0xF1,
        SBCZeroPageX   = 0xF5,
        INCZeroPageX   = 0xF6,
        SEDImplied     = 0xF8,
        SBCAbsoluteY   = 0xF9,
        SBCAbsoluteX   = 0xFD,
        INCAbsoluteX   = 0xFE,
    }
}

#[derive(Copy, Clone)]
pub struct Opcode {
    instruction: u32,
}

impl Opcode {
    pub fn new(op: u32) -> Opcode {
        Opcode {
            instruction: op
        }
    }

    pub fn op(&self) -> u8 {
        (self.instruction & 0xFF) as u8
    }

    pub fn imm1(&self) -> u8 {
        ((self.instruction >> 8) & 0xFF) as u8
    }

    pub fn imm2(&self) -> u8 {
        ((self.instruction >> 16) & 0xFF) as u8
    }

    pub fn abs_addr(&self) -> usize {
        (self.instruction >> 8) as usize
    }
}

impl fmt::Debug for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Opcode: {{\n\top: 0x{:02X},\n\timm1: 0x{:02X}\n\timm2: 0x{:02X}\n}}", 
               self.op(), self.imm1(), self.imm2())
    }
}
