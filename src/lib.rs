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

        CopyRegister(Register, Register),

        OrRegister(Register, Register),
        AndRegister(Register, Register),
        XorRegister(Register, Register),

        AddRegister(Register, Register),
        SubRegister(Register, Register),
        ShiftRightRegister(Register, Register),
        MinusSubRegister(Register, Register),
        ShiftLeftRegister(Register, Register),

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

        AddAddress(Register),
        LoadSpriteAddress(Register),
        StoreBCD(Register),

        ContextStore(Register),
        ContextLoad(Register)
    }

    pub fn decode(op_code: u16) -> Option<OpCode> {
        use OpCode::*;

        if op_code == 0x00E0
        {
            return Some(ClearDisplay);
        }

        if op_code == 0x00EE
        {
            return Some(Return);
        }

        match first_nibble(op_code)
        {
            0x1 => return Some(Jump(Address(skip_first_nibble(op_code)))),
            0x2 => return Some(Call(Address(skip_first_nibble(op_code)))),
            0x3 => return Some(SkipEqualConstant(Register(second_nibble(op_code)), Constant8(last_byte(op_code)))),
            0x4 => return Some(SkipNotEqualConstant(Register(second_nibble(op_code)), Constant8(last_byte(op_code)))),
            0x5 => if last_nibble(op_code) != 0 { return None; }
                   else { return Some(SkipEqualRegister(Register(second_nibble(op_code)), Register(third_nibble(op_code))))},
            0x6 => return Some(LoadConstant(Register(second_nibble(op_code)), Constant8(last_byte(op_code)))),
            0x7 => return Some(AddConstant(Register(second_nibble(op_code)), Constant8(last_byte(op_code)))),
            0x8 => {
                       let r1 = Register(second_nibble(op_code));
                       let r2 = Register(third_nibble(op_code));
                       match last_nibble(op_code)
                       {
                           0x0 => return Some(CopyRegister(r1, r2)),
                           0x1 => return Some(OrRegister(r1, r2)),
                           0x2 => return Some(AndRegister(r1, r2)),
                           0x3 => return Some(XorRegister(r1, r2)),
                           0x4 => return Some(AddRegister(r1, r2)),
                           0x5 => return Some(SubRegister(r1, r2)),
                           0x6 => return Some(ShiftRightRegister(r1, r2)),
                           0x7 => return Some(MinusSubRegister(r1, r2)),
                           0xE => return Some(ShiftLeftRegister(r1, r2)),
                           _ => return None,
                       }
                 }
            0x9 => if last_nibble(op_code) != 0 { return None; }
                   else { return Some(SkipNotEqualRegister(Register(second_nibble(op_code)), Register(third_nibble(op_code))))},
            0xA => return Some(LoadAddressConstant(Address(skip_first_nibble(op_code)))),
            0xB => return Some(JumpWithOffset(Address(skip_first_nibble(op_code)))),
            0xC => return Some(Rand(Register(second_nibble(op_code)), Constant8(last_byte(op_code)))),
            0xD => return Some(Draw(Register(first_nibble(op_code)), Register(second_nibble(op_code)), Constant4(last_nibble(op_code)))),
            0xE => {
                       match last_byte(op_code)
                       {
                           0x9E => return Some(SkipKeyEqual(Register(second_nibble(op_code)))),
                           0xA1 => return Some(SkipKeyNotEqual(Register(second_nibble(op_code)))),
                           _ => return None,
                       }

                   }
            0xF => {
                       match last_byte(op_code)
                       {
                           0x07 => return Some(LoadDelayTimer(Register(second_nibble(op_code)))),
                           0x0A => return Some(WaitKey(Register(second_nibble(op_code)))),
                           0x15 => return Some(StoreDelayTimer(Register(second_nibble(op_code)))),
                           0x18 => return Some(StoreBeepTimer(Register(second_nibble(op_code)))),
                           0x1E => return Some(AddAddress(Register(second_nibble(op_code)))),
                           0x29 => return Some(LoadSpriteAddress(Register(second_nibble(op_code)))),
                           0x33 => return Some(StoreBCD(Register(second_nibble(op_code)))),
                           0x55 => return Some(ContextStore(Register(second_nibble(op_code)))),
                           0x65 => return Some(ContextLoad(Register(second_nibble(op_code)))),
                           _ => return None,
                       }

                   }
            _ => {},
        }

        return None;
    }

    fn first_nibble(word: u16) -> u8
    {
        return ((word & 0xF000) >> 12) as u8;
    }

    fn second_nibble(word: u16) -> u8
    {
        return ((word & 0x0F00) >> 8) as u8;
    }

    fn third_nibble(word: u16) -> u8
    {
        return ((word & 0x00F0) >> 4) as u8;
    }

    fn last_nibble(word: u16) -> u8
    {
        return (word & 0x000F) as u8;
    }

    fn skip_first_nibble(word: u16) -> u16
    {
        return word & 0x0FFF;
    }

    fn last_byte(word: u16) -> u8
    {
        return (word & 0x00FF) as u8;
    }

}
