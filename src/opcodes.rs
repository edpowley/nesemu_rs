
#[derive(Debug)]
pub enum Mnemonic {
    XXX, // Invalid
    ADC, // Add with Carry
    AND, // Logical AND
    ASL, // Arithmetic Shift Left
    BCC, // Branch if Carry Clear
    BCS, // Branch if Carry Set
    BEQ, // Branch if Equal
    BIT, // Bit Test
    BMI, // Branch if Minus
    BNE, // Branch if Not Equal
    BPL, // Branch if Positive
    BRK, // Force Interrupt
    BVC, // Branch if Overflow Clear
    BVS, // Branch if Overflow Set
    CLC, // Clear Carry Flag
    CLD, // Clear Decimal Mode
    CLI, // Clear Interrupt Disable
    CLV, // Clear Overflow Flag
    CMP, // Compare
    CPX, // Compare X Register
    CPY, // Compare Y Register
    DEC, // Decrement Memory
    DEX, // Decrement X Register
    DEY, // Decrement Y Register
    EOR, // Exclusive OR
    INC, // Increment Memory
    INX, // Increment X Register
    INY, // Increment Y Register
    JMP, // Jump
    JSR, // Jump to Subroutine
    LDA, // Load Accumulator
    LDX, // Load X Register
    LDY, // Load Y Register
    LSR, // Logical Shift Right
    NOP, // No Operation
    ORA, // Logical Inclusive OR
    PHA, // Push Accumulator
    PHP, // Push Processor Status
    PLA, // Pull Accumulator
    PLP, // Pull Processor Status
    ROL, // Rotate Left
    ROR, // Rotate Right
    RTI, // Return from Interrupt
    RTS, // Return from Subroutine
    SBC, // Subtract with Carry
    SEC, // Set Carry Flag
    SED, // Set Decimal Flag
    SEI, // Set Interrupt Disable
    STA, // Store Accumulator
    STX, // Store X Register
    STY, // Store Y Register
    TAX, // Transfer Accumulator to X
    TAY, // Transfer Accumulator to Y
    TSX, // Transfer Stack Pointer to X
    TXA, // Transfer X to Accumulator
    TXS, // Transfer X to Stack Pointer
    TYA, // Transfer Y to Accumulator
}

#[derive(Debug)]
pub enum AddressMode {
    ABS, // Absolute
    ABX, // Absolute,X
    ABY, // Absolute,Y
    IDX, // (Indirect,X)
    IDY, // (Indirect),Y
    IMM, // Immediate
    IMP, // Accumulator / Implied
    IND, // Indirect
    REL, // Relative
    ZPG, // Zero Page
    ZPX, // Zero Page,X
    ZPY, // Zero Page,Y
}

#[derive(Debug)]
pub struct Opcode {
    pub mnemonic : Mnemonic,
    pub address_mode : AddressMode
}

pub fn decode(instruction: u8) -> Opcode {
    match instruction {
        0x00 => Opcode { mnemonic: Mnemonic::BRK, address_mode: AddressMode::IMP },
        0x01 => Opcode { mnemonic: Mnemonic::ORA, address_mode: AddressMode::IDX },
        0x02 => Opcode { mnemonic: Mnemonic::XXX, address_mode: AddressMode::IMP },
        0x03 => Opcode { mnemonic: Mnemonic::XXX, address_mode: AddressMode::IMP },
        0x04 => Opcode { mnemonic: Mnemonic::XXX, address_mode: AddressMode::IMP },
        0x05 => Opcode { mnemonic: Mnemonic::ORA, address_mode: AddressMode::ZPG },
        0x06 => Opcode { mnemonic: Mnemonic::ASL, address_mode: AddressMode::ZPG },
        0x07 => Opcode { mnemonic: Mnemonic::XXX, address_mode: AddressMode::IMP },
        0x08 => Opcode { mnemonic: Mnemonic::PHP, address_mode: AddressMode::IMP },
        0x09 => Opcode { mnemonic: Mnemonic::ORA, address_mode: AddressMode::IMM },
        0x0A => Opcode { mnemonic: Mnemonic::ASL, address_mode: AddressMode::IMP },
        0x0B => Opcode { mnemonic: Mnemonic::XXX, address_mode: AddressMode::IMP },
        0x0C => Opcode { mnemonic: Mnemonic::XXX, address_mode: AddressMode::IMP },
        0x0D => Opcode { mnemonic: Mnemonic::ORA, address_mode: AddressMode::ABS },
        0x0E => Opcode { mnemonic: Mnemonic::ASL, address_mode: AddressMode::ABS },
        0x0F => Opcode { mnemonic: Mnemonic::XXX, address_mode: AddressMode::IMP },
        0x10 => Opcode { mnemonic: Mnemonic::BPL, address_mode: AddressMode::REL },
        0x11 => Opcode { mnemonic: Mnemonic::ORA, address_mode: AddressMode::IDY },
        0x12 => Opcode { mnemonic: Mnemonic::XXX, address_mode: AddressMode::IMP },
        0x13 => Opcode { mnemonic: Mnemonic::XXX, address_mode: AddressMode::IMP },
        0x14 => Opcode { mnemonic: Mnemonic::XXX, address_mode: AddressMode::IMP },
        0x15 => Opcode { mnemonic: Mnemonic::ORA, address_mode: AddressMode::ZPX },
        0x16 => Opcode { mnemonic: Mnemonic::ASL, address_mode: AddressMode::ZPX },
        0x17 => Opcode { mnemonic: Mnemonic::XXX, address_mode: AddressMode::IMP },
        0x18 => Opcode { mnemonic: Mnemonic::CLC, address_mode: AddressMode::IMP },
        0x19 => Opcode { mnemonic: Mnemonic::ORA, address_mode: AddressMode::ABY },
        0x1A => Opcode { mnemonic: Mnemonic::XXX, address_mode: AddressMode::IMP },
        0x1B => Opcode { mnemonic: Mnemonic::XXX, address_mode: AddressMode::IMP },
        0x1C => Opcode { mnemonic: Mnemonic::XXX, address_mode: AddressMode::IMP },
        0x1D => Opcode { mnemonic: Mnemonic::ORA, address_mode: AddressMode::ABX },
        0x1E => Opcode { mnemonic: Mnemonic::ASL, address_mode: AddressMode::ABX },
        0x1F => Opcode { mnemonic: Mnemonic::XXX, address_mode: AddressMode::IMP },
        0x20 => Opcode { mnemonic: Mnemonic::JSR, address_mode: AddressMode::ABS },
        0x21 => Opcode { mnemonic: Mnemonic::AND, address_mode: AddressMode::IDX },
        0x22 => Opcode { mnemonic: Mnemonic::XXX, address_mode: AddressMode::IMP },
        0x23 => Opcode { mnemonic: Mnemonic::XXX, address_mode: AddressMode::IMP },
        0x24 => Opcode { mnemonic: Mnemonic::BIT, address_mode: AddressMode::ZPG },
        0x25 => Opcode { mnemonic: Mnemonic::AND, address_mode: AddressMode::ZPG },
        0x26 => Opcode { mnemonic: Mnemonic::ROL, address_mode: AddressMode::ZPG },
        0x27 => Opcode { mnemonic: Mnemonic::XXX, address_mode: AddressMode::IMP },
        0x28 => Opcode { mnemonic: Mnemonic::PLP, address_mode: AddressMode::IMP },
        0x29 => Opcode { mnemonic: Mnemonic::AND, address_mode: AddressMode::IMM },
        0x2A => Opcode { mnemonic: Mnemonic::ROL, address_mode: AddressMode::IMP },
        0x2B => Opcode { mnemonic: Mnemonic::XXX, address_mode: AddressMode::IMP },
        0x2C => Opcode { mnemonic: Mnemonic::BIT, address_mode: AddressMode::ABS },
        0x2D => Opcode { mnemonic: Mnemonic::AND, address_mode: AddressMode::ABS },
        0x2E => Opcode { mnemonic: Mnemonic::ROL, address_mode: AddressMode::ABS },
        0x2F => Opcode { mnemonic: Mnemonic::XXX, address_mode: AddressMode::IMP },
        0x30 => Opcode { mnemonic: Mnemonic::BMI, address_mode: AddressMode::REL },
        0x31 => Opcode { mnemonic: Mnemonic::AND, address_mode: AddressMode::IDY },
        0x32 => Opcode { mnemonic: Mnemonic::XXX, address_mode: AddressMode::IMP },
        0x33 => Opcode { mnemonic: Mnemonic::XXX, address_mode: AddressMode::IMP },
        0x34 => Opcode { mnemonic: Mnemonic::XXX, address_mode: AddressMode::IMP },
        0x35 => Opcode { mnemonic: Mnemonic::AND, address_mode: AddressMode::ZPX },
        0x36 => Opcode { mnemonic: Mnemonic::ROL, address_mode: AddressMode::ZPX },
        0x37 => Opcode { mnemonic: Mnemonic::XXX, address_mode: AddressMode::IMP },
        0x38 => Opcode { mnemonic: Mnemonic::SEC, address_mode: AddressMode::IMP },
        0x39 => Opcode { mnemonic: Mnemonic::AND, address_mode: AddressMode::ABY },
        0x3A => Opcode { mnemonic: Mnemonic::XXX, address_mode: AddressMode::IMP },
        0x3B => Opcode { mnemonic: Mnemonic::XXX, address_mode: AddressMode::IMP },
        0x3C => Opcode { mnemonic: Mnemonic::XXX, address_mode: AddressMode::IMP },
        0x3D => Opcode { mnemonic: Mnemonic::AND, address_mode: AddressMode::ABX },
        0x3E => Opcode { mnemonic: Mnemonic::ROL, address_mode: AddressMode::ABX },
        0x3F => Opcode { mnemonic: Mnemonic::XXX, address_mode: AddressMode::IMP },
        0x40 => Opcode { mnemonic: Mnemonic::RTI, address_mode: AddressMode::IMP },
        0x41 => Opcode { mnemonic: Mnemonic::EOR, address_mode: AddressMode::IDX },
        0x42 => Opcode { mnemonic: Mnemonic::XXX, address_mode: AddressMode::IMP },
        0x43 => Opcode { mnemonic: Mnemonic::XXX, address_mode: AddressMode::IMP },
        0x44 => Opcode { mnemonic: Mnemonic::XXX, address_mode: AddressMode::IMP },
        0x45 => Opcode { mnemonic: Mnemonic::EOR, address_mode: AddressMode::ZPG },
        0x46 => Opcode { mnemonic: Mnemonic::LSR, address_mode: AddressMode::ZPG },
        0x47 => Opcode { mnemonic: Mnemonic::XXX, address_mode: AddressMode::IMP },
        0x48 => Opcode { mnemonic: Mnemonic::PHA, address_mode: AddressMode::IMP },
        0x49 => Opcode { mnemonic: Mnemonic::EOR, address_mode: AddressMode::IMM },
        0x4A => Opcode { mnemonic: Mnemonic::LSR, address_mode: AddressMode::IMP },
        0x4B => Opcode { mnemonic: Mnemonic::XXX, address_mode: AddressMode::IMP },
        0x4C => Opcode { mnemonic: Mnemonic::JMP, address_mode: AddressMode::ABS },
        0x4D => Opcode { mnemonic: Mnemonic::EOR, address_mode: AddressMode::ABS },
        0x4E => Opcode { mnemonic: Mnemonic::LSR, address_mode: AddressMode::ABS },
        0x4F => Opcode { mnemonic: Mnemonic::XXX, address_mode: AddressMode::IMP },
        0x50 => Opcode { mnemonic: Mnemonic::BVC, address_mode: AddressMode::REL },
        0x51 => Opcode { mnemonic: Mnemonic::EOR, address_mode: AddressMode::IDY },
        0x52 => Opcode { mnemonic: Mnemonic::XXX, address_mode: AddressMode::IMP },
        0x53 => Opcode { mnemonic: Mnemonic::XXX, address_mode: AddressMode::IMP },
        0x54 => Opcode { mnemonic: Mnemonic::XXX, address_mode: AddressMode::IMP },
        0x55 => Opcode { mnemonic: Mnemonic::EOR, address_mode: AddressMode::ZPX },
        0x56 => Opcode { mnemonic: Mnemonic::LSR, address_mode: AddressMode::ZPX },
        0x57 => Opcode { mnemonic: Mnemonic::XXX, address_mode: AddressMode::IMP },
        0x58 => Opcode { mnemonic: Mnemonic::CLI, address_mode: AddressMode::IMP },
        0x59 => Opcode { mnemonic: Mnemonic::EOR, address_mode: AddressMode::ABY },
        0x5A => Opcode { mnemonic: Mnemonic::XXX, address_mode: AddressMode::IMP },
        0x5B => Opcode { mnemonic: Mnemonic::XXX, address_mode: AddressMode::IMP },
        0x5C => Opcode { mnemonic: Mnemonic::XXX, address_mode: AddressMode::IMP },
        0x5D => Opcode { mnemonic: Mnemonic::EOR, address_mode: AddressMode::ABX },
        0x5E => Opcode { mnemonic: Mnemonic::LSR, address_mode: AddressMode::ABX },
        0x5F => Opcode { mnemonic: Mnemonic::XXX, address_mode: AddressMode::IMP },
        0x60 => Opcode { mnemonic: Mnemonic::RTS, address_mode: AddressMode::IMP },
        0x61 => Opcode { mnemonic: Mnemonic::ADC, address_mode: AddressMode::IDX },
        0x62 => Opcode { mnemonic: Mnemonic::XXX, address_mode: AddressMode::IMP },
        0x63 => Opcode { mnemonic: Mnemonic::XXX, address_mode: AddressMode::IMP },
        0x64 => Opcode { mnemonic: Mnemonic::XXX, address_mode: AddressMode::IMP },
        0x65 => Opcode { mnemonic: Mnemonic::ADC, address_mode: AddressMode::ZPG },
        0x66 => Opcode { mnemonic: Mnemonic::ROR, address_mode: AddressMode::ZPG },
        0x67 => Opcode { mnemonic: Mnemonic::XXX, address_mode: AddressMode::IMP },
        0x68 => Opcode { mnemonic: Mnemonic::PLA, address_mode: AddressMode::IMP },
        0x69 => Opcode { mnemonic: Mnemonic::ADC, address_mode: AddressMode::IMM },
        0x6A => Opcode { mnemonic: Mnemonic::ROR, address_mode: AddressMode::IMP },
        0x6B => Opcode { mnemonic: Mnemonic::XXX, address_mode: AddressMode::IMP },
        0x6C => Opcode { mnemonic: Mnemonic::JMP, address_mode: AddressMode::IND },
        0x6D => Opcode { mnemonic: Mnemonic::ADC, address_mode: AddressMode::ABS },
        0x6E => Opcode { mnemonic: Mnemonic::ROR, address_mode: AddressMode::ABS },
        0x6F => Opcode { mnemonic: Mnemonic::XXX, address_mode: AddressMode::IMP },
        0x70 => Opcode { mnemonic: Mnemonic::BVS, address_mode: AddressMode::REL },
        0x71 => Opcode { mnemonic: Mnemonic::ADC, address_mode: AddressMode::IDY },
        0x72 => Opcode { mnemonic: Mnemonic::XXX, address_mode: AddressMode::IMP },
        0x73 => Opcode { mnemonic: Mnemonic::XXX, address_mode: AddressMode::IMP },
        0x74 => Opcode { mnemonic: Mnemonic::XXX, address_mode: AddressMode::IMP },
        0x75 => Opcode { mnemonic: Mnemonic::ADC, address_mode: AddressMode::ZPX },
        0x76 => Opcode { mnemonic: Mnemonic::ROR, address_mode: AddressMode::ZPX },
        0x77 => Opcode { mnemonic: Mnemonic::XXX, address_mode: AddressMode::IMP },
        0x78 => Opcode { mnemonic: Mnemonic::SEI, address_mode: AddressMode::IMP },
        0x79 => Opcode { mnemonic: Mnemonic::ADC, address_mode: AddressMode::ABY },
        0x7A => Opcode { mnemonic: Mnemonic::XXX, address_mode: AddressMode::IMP },
        0x7B => Opcode { mnemonic: Mnemonic::XXX, address_mode: AddressMode::IMP },
        0x7C => Opcode { mnemonic: Mnemonic::XXX, address_mode: AddressMode::IMP },
        0x7D => Opcode { mnemonic: Mnemonic::ADC, address_mode: AddressMode::ABX },
        0x7E => Opcode { mnemonic: Mnemonic::ROR, address_mode: AddressMode::ABX },
        0x7F => Opcode { mnemonic: Mnemonic::XXX, address_mode: AddressMode::IMP },
        0x80 => Opcode { mnemonic: Mnemonic::XXX, address_mode: AddressMode::IMP },
        0x81 => Opcode { mnemonic: Mnemonic::STA, address_mode: AddressMode::IDX },
        0x82 => Opcode { mnemonic: Mnemonic::XXX, address_mode: AddressMode::IMP },
        0x83 => Opcode { mnemonic: Mnemonic::XXX, address_mode: AddressMode::IMP },
        0x84 => Opcode { mnemonic: Mnemonic::STY, address_mode: AddressMode::ZPG },
        0x85 => Opcode { mnemonic: Mnemonic::STA, address_mode: AddressMode::ZPG },
        0x86 => Opcode { mnemonic: Mnemonic::STX, address_mode: AddressMode::ZPG },
        0x87 => Opcode { mnemonic: Mnemonic::XXX, address_mode: AddressMode::IMP },
        0x88 => Opcode { mnemonic: Mnemonic::DEY, address_mode: AddressMode::IMP },
        0x89 => Opcode { mnemonic: Mnemonic::XXX, address_mode: AddressMode::IMP },
        0x8A => Opcode { mnemonic: Mnemonic::TXA, address_mode: AddressMode::IMP },
        0x8B => Opcode { mnemonic: Mnemonic::XXX, address_mode: AddressMode::IMP },
        0x8C => Opcode { mnemonic: Mnemonic::STY, address_mode: AddressMode::ABS },
        0x8D => Opcode { mnemonic: Mnemonic::STA, address_mode: AddressMode::ABS },
        0x8E => Opcode { mnemonic: Mnemonic::STX, address_mode: AddressMode::ABS },
        0x8F => Opcode { mnemonic: Mnemonic::XXX, address_mode: AddressMode::IMP },
        0x90 => Opcode { mnemonic: Mnemonic::BCC, address_mode: AddressMode::REL },
        0x91 => Opcode { mnemonic: Mnemonic::STA, address_mode: AddressMode::IDY },
        0x92 => Opcode { mnemonic: Mnemonic::XXX, address_mode: AddressMode::IMP },
        0x93 => Opcode { mnemonic: Mnemonic::XXX, address_mode: AddressMode::IMP },
        0x94 => Opcode { mnemonic: Mnemonic::STY, address_mode: AddressMode::ZPX },
        0x95 => Opcode { mnemonic: Mnemonic::STA, address_mode: AddressMode::ZPX },
        0x96 => Opcode { mnemonic: Mnemonic::STX, address_mode: AddressMode::ZPY },
        0x97 => Opcode { mnemonic: Mnemonic::XXX, address_mode: AddressMode::IMP },
        0x98 => Opcode { mnemonic: Mnemonic::TYA, address_mode: AddressMode::IMP },
        0x99 => Opcode { mnemonic: Mnemonic::STA, address_mode: AddressMode::ABY },
        0x9A => Opcode { mnemonic: Mnemonic::TXS, address_mode: AddressMode::IMP },
        0x9B => Opcode { mnemonic: Mnemonic::XXX, address_mode: AddressMode::IMP },
        0x9C => Opcode { mnemonic: Mnemonic::XXX, address_mode: AddressMode::IMP },
        0x9D => Opcode { mnemonic: Mnemonic::STA, address_mode: AddressMode::ABX },
        0x9E => Opcode { mnemonic: Mnemonic::XXX, address_mode: AddressMode::IMP },
        0x9F => Opcode { mnemonic: Mnemonic::XXX, address_mode: AddressMode::IMP },
        0xA0 => Opcode { mnemonic: Mnemonic::LDY, address_mode: AddressMode::IMM },
        0xA1 => Opcode { mnemonic: Mnemonic::LDA, address_mode: AddressMode::IDX },
        0xA2 => Opcode { mnemonic: Mnemonic::LDX, address_mode: AddressMode::IMM },
        0xA3 => Opcode { mnemonic: Mnemonic::XXX, address_mode: AddressMode::IMP },
        0xA4 => Opcode { mnemonic: Mnemonic::LDY, address_mode: AddressMode::ZPG },
        0xA5 => Opcode { mnemonic: Mnemonic::LDA, address_mode: AddressMode::ZPG },
        0xA6 => Opcode { mnemonic: Mnemonic::LDX, address_mode: AddressMode::ZPG },
        0xA7 => Opcode { mnemonic: Mnemonic::XXX, address_mode: AddressMode::IMP },
        0xA8 => Opcode { mnemonic: Mnemonic::TAY, address_mode: AddressMode::IMP },
        0xA9 => Opcode { mnemonic: Mnemonic::LDA, address_mode: AddressMode::IMM },
        0xAA => Opcode { mnemonic: Mnemonic::TAX, address_mode: AddressMode::IMP },
        0xAB => Opcode { mnemonic: Mnemonic::XXX, address_mode: AddressMode::IMP },
        0xAC => Opcode { mnemonic: Mnemonic::LDY, address_mode: AddressMode::ABS },
        0xAD => Opcode { mnemonic: Mnemonic::LDA, address_mode: AddressMode::ABS },
        0xAE => Opcode { mnemonic: Mnemonic::LDX, address_mode: AddressMode::ABS },
        0xAF => Opcode { mnemonic: Mnemonic::XXX, address_mode: AddressMode::IMP },
        0xB0 => Opcode { mnemonic: Mnemonic::BCS, address_mode: AddressMode::REL },
        0xB1 => Opcode { mnemonic: Mnemonic::LDA, address_mode: AddressMode::IDY },
        0xB2 => Opcode { mnemonic: Mnemonic::XXX, address_mode: AddressMode::IMP },
        0xB3 => Opcode { mnemonic: Mnemonic::XXX, address_mode: AddressMode::IMP },
        0xB4 => Opcode { mnemonic: Mnemonic::LDY, address_mode: AddressMode::ZPX },
        0xB5 => Opcode { mnemonic: Mnemonic::LDA, address_mode: AddressMode::ZPX },
        0xB6 => Opcode { mnemonic: Mnemonic::LDX, address_mode: AddressMode::ZPY },
        0xB7 => Opcode { mnemonic: Mnemonic::XXX, address_mode: AddressMode::IMP },
        0xB8 => Opcode { mnemonic: Mnemonic::CLV, address_mode: AddressMode::IMP },
        0xB9 => Opcode { mnemonic: Mnemonic::LDA, address_mode: AddressMode::ABY },
        0xBA => Opcode { mnemonic: Mnemonic::TSX, address_mode: AddressMode::IMP },
        0xBB => Opcode { mnemonic: Mnemonic::XXX, address_mode: AddressMode::IMP },
        0xBC => Opcode { mnemonic: Mnemonic::LDY, address_mode: AddressMode::ABX },
        0xBD => Opcode { mnemonic: Mnemonic::LDA, address_mode: AddressMode::ABX },
        0xBE => Opcode { mnemonic: Mnemonic::LDX, address_mode: AddressMode::ABY },
        0xBF => Opcode { mnemonic: Mnemonic::XXX, address_mode: AddressMode::IMP },
        0xC0 => Opcode { mnemonic: Mnemonic::CPY, address_mode: AddressMode::IMM },
        0xC1 => Opcode { mnemonic: Mnemonic::CMP, address_mode: AddressMode::IDX },
        0xC2 => Opcode { mnemonic: Mnemonic::XXX, address_mode: AddressMode::IMP },
        0xC3 => Opcode { mnemonic: Mnemonic::XXX, address_mode: AddressMode::IMP },
        0xC4 => Opcode { mnemonic: Mnemonic::CPY, address_mode: AddressMode::ZPG },
        0xC5 => Opcode { mnemonic: Mnemonic::CMP, address_mode: AddressMode::ZPG },
        0xC6 => Opcode { mnemonic: Mnemonic::DEC, address_mode: AddressMode::ZPG },
        0xC7 => Opcode { mnemonic: Mnemonic::XXX, address_mode: AddressMode::IMP },
        0xC8 => Opcode { mnemonic: Mnemonic::INY, address_mode: AddressMode::IMP },
        0xC9 => Opcode { mnemonic: Mnemonic::CMP, address_mode: AddressMode::IMM },
        0xCA => Opcode { mnemonic: Mnemonic::DEX, address_mode: AddressMode::IMP },
        0xCB => Opcode { mnemonic: Mnemonic::XXX, address_mode: AddressMode::IMP },
        0xCC => Opcode { mnemonic: Mnemonic::CPY, address_mode: AddressMode::ABS },
        0xCD => Opcode { mnemonic: Mnemonic::CMP, address_mode: AddressMode::ABS },
        0xCE => Opcode { mnemonic: Mnemonic::DEC, address_mode: AddressMode::ABS },
        0xCF => Opcode { mnemonic: Mnemonic::XXX, address_mode: AddressMode::IMP },
        0xD0 => Opcode { mnemonic: Mnemonic::BNE, address_mode: AddressMode::REL },
        0xD1 => Opcode { mnemonic: Mnemonic::CMP, address_mode: AddressMode::IDY },
        0xD2 => Opcode { mnemonic: Mnemonic::XXX, address_mode: AddressMode::IMP },
        0xD3 => Opcode { mnemonic: Mnemonic::XXX, address_mode: AddressMode::IMP },
        0xD4 => Opcode { mnemonic: Mnemonic::XXX, address_mode: AddressMode::IMP },
        0xD5 => Opcode { mnemonic: Mnemonic::CMP, address_mode: AddressMode::ZPX },
        0xD6 => Opcode { mnemonic: Mnemonic::DEC, address_mode: AddressMode::ZPX },
        0xD7 => Opcode { mnemonic: Mnemonic::XXX, address_mode: AddressMode::IMP },
        0xD8 => Opcode { mnemonic: Mnemonic::CLD, address_mode: AddressMode::IMP },
        0xD9 => Opcode { mnemonic: Mnemonic::CMP, address_mode: AddressMode::ABY },
        0xDA => Opcode { mnemonic: Mnemonic::XXX, address_mode: AddressMode::IMP },
        0xDB => Opcode { mnemonic: Mnemonic::XXX, address_mode: AddressMode::IMP },
        0xDC => Opcode { mnemonic: Mnemonic::XXX, address_mode: AddressMode::IMP },
        0xDD => Opcode { mnemonic: Mnemonic::CMP, address_mode: AddressMode::ABX },
        0xDE => Opcode { mnemonic: Mnemonic::DEC, address_mode: AddressMode::ABX },
        0xDF => Opcode { mnemonic: Mnemonic::XXX, address_mode: AddressMode::IMP },
        0xE0 => Opcode { mnemonic: Mnemonic::CPX, address_mode: AddressMode::IMM },
        0xE1 => Opcode { mnemonic: Mnemonic::SBC, address_mode: AddressMode::IDX },
        0xE2 => Opcode { mnemonic: Mnemonic::XXX, address_mode: AddressMode::IMP },
        0xE3 => Opcode { mnemonic: Mnemonic::XXX, address_mode: AddressMode::IMP },
        0xE4 => Opcode { mnemonic: Mnemonic::CPX, address_mode: AddressMode::ZPG },
        0xE5 => Opcode { mnemonic: Mnemonic::SBC, address_mode: AddressMode::ZPG },
        0xE6 => Opcode { mnemonic: Mnemonic::INC, address_mode: AddressMode::ZPG },
        0xE7 => Opcode { mnemonic: Mnemonic::XXX, address_mode: AddressMode::IMP },
        0xE8 => Opcode { mnemonic: Mnemonic::INX, address_mode: AddressMode::IMP },
        0xE9 => Opcode { mnemonic: Mnemonic::SBC, address_mode: AddressMode::IMM },
        0xEA => Opcode { mnemonic: Mnemonic::NOP, address_mode: AddressMode::IMP },
        0xEB => Opcode { mnemonic: Mnemonic::XXX, address_mode: AddressMode::IMP },
        0xEC => Opcode { mnemonic: Mnemonic::CPX, address_mode: AddressMode::ABS },
        0xED => Opcode { mnemonic: Mnemonic::SBC, address_mode: AddressMode::ABS },
        0xEE => Opcode { mnemonic: Mnemonic::INC, address_mode: AddressMode::ABS },
        0xEF => Opcode { mnemonic: Mnemonic::XXX, address_mode: AddressMode::IMP },
        0xF0 => Opcode { mnemonic: Mnemonic::BEQ, address_mode: AddressMode::REL },
        0xF1 => Opcode { mnemonic: Mnemonic::SBC, address_mode: AddressMode::IDY },
        0xF2 => Opcode { mnemonic: Mnemonic::XXX, address_mode: AddressMode::IMP },
        0xF3 => Opcode { mnemonic: Mnemonic::XXX, address_mode: AddressMode::IMP },
        0xF4 => Opcode { mnemonic: Mnemonic::XXX, address_mode: AddressMode::IMP },
        0xF5 => Opcode { mnemonic: Mnemonic::SBC, address_mode: AddressMode::ZPX },
        0xF6 => Opcode { mnemonic: Mnemonic::INC, address_mode: AddressMode::ZPX },
        0xF7 => Opcode { mnemonic: Mnemonic::XXX, address_mode: AddressMode::IMP },
        0xF8 => Opcode { mnemonic: Mnemonic::SED, address_mode: AddressMode::IMP },
        0xF9 => Opcode { mnemonic: Mnemonic::SBC, address_mode: AddressMode::ABY },
        0xFA => Opcode { mnemonic: Mnemonic::XXX, address_mode: AddressMode::IMP },
        0xFB => Opcode { mnemonic: Mnemonic::XXX, address_mode: AddressMode::IMP },
        0xFC => Opcode { mnemonic: Mnemonic::XXX, address_mode: AddressMode::IMP },
        0xFD => Opcode { mnemonic: Mnemonic::SBC, address_mode: AddressMode::ABX },
        0xFE => Opcode { mnemonic: Mnemonic::INC, address_mode: AddressMode::ABX },
        0xFF => Opcode { mnemonic: Mnemonic::XXX, address_mode: AddressMode::IMP },    
    }
}
