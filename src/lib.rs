pub mod processor {

    #[derive(Debug, PartialEq)]
    pub struct Address(pub u16);

    #[derive(Debug, PartialEq)]
    pub struct Constant8(pub u8);

    #[derive(Debug, PartialEq)]
    pub struct Constant4(pub u8);

    #[derive(Debug, PartialEq)]
    pub struct Register(pub u8);

    #[derive(Debug, PartialEq)]
    pub enum OpCode {
        ClearDisplay,
        Return,

        Jump(Address),
        Call(Address),

        SkipEqualConstant(Register, Constant8),
        SkipNotEqualConstant(Register, Constant8),
        SkipEqualRegister(Register, Register),

        LoadConstant(Register, Constant8),
        AddConstant(Register, Constant8),

        LoadRegister(Register, Register),

        OrRegister(Register, Register),
        AndRegister(Register, Register),
        XorRegister(Register, Register),

        AddRegister(Register, Register),
        SubRegister(Register, Register),
        ShiftRight(Register, Register),
        MinusSubRegister(Register, Register),
        ShiftLeft(Register, Register),

        SkipNotEqualRegister(Register, Register),

        LoadAddressConstant(Address),
        JumpWithOffset(Address),

        Rand(Register, Constant8),

        Draw(Register, Register, Constant4),

        SkipKeyEqual(Register),
        SkipKeyNotEqual(Register),

        LoadDelayTimer(Register),
        WaitKey(Register),

        StoreDelayTimer(Register),
        StoreBeepTimer(Register),

        LoadAddressOffset(Register),
        LoadSpriteAddress(Register),
        StoreBCD(Register),

        ContextStore(Register),
        ContextLoad(Register)
    }

    pub fn decode(instruction: u16) -> Option<OpCode> {
        use OpCode::*;

        if instruction == 0x00E0
        {
            return Some(ClearDisplay);
        }

        if instruction == 0x00EE
        {
            return Some(Return);
        }

        match first_nibble(instruction)
        {
            1 => return Some(Jump(Address(skip_first_nibble(instruction)))),
            2 => return Some(Call(Address(skip_first_nibble(instruction)))),
            3 => return Some(SkipEqualConstant(Register(second_nibble(instruction)), Constant8(last_byte(instruction)))),
            4 => return Some(SkipNotEqualConstant(Register(second_nibble(instruction)), Constant8(last_byte(instruction)))),
            _ => {},
        }

        return None;
    }

    fn first_nibble(word: u16) -> u8 {
        return ((word & 0xF000) >> 12) as u8;
    }

    fn second_nibble(word: u16) -> u8 {
        return ((word & 0x0F00) >> 8) as u8;
    }

    fn skip_first_nibble(word: u16) -> u16 {
        return word & 0x0FFF;
    }

    fn last_byte(word: u16) -> u8 {
        return (word & 0x00FF) as u8;
    }

}
