extern crate chirp8;

use chirp8::instructions::{Address, Register, Constant8, Constant4, decode};
use chirp8::instructions::OpCode::*;


#[test]
fn test_clear()
{
    assert_eq!(decode(0x00E0).unwrap(), ClearDisplay);
}

#[test]
fn test_return()
{
    assert_eq!(decode(0x00EE).unwrap(), Return);
}

#[test]
fn test_invalid_instructions_starting_with_0()
{
    for op_code in 0x0000..=0x0FFF
    {
        if op_code == 0x00E0 || op_code == 0x00EE
        {
            continue;
        }

        assert!(decode(op_code).is_none());
    }
}

#[test]
fn test_jump()
{
    assert_eq!(decode(0x13AB).unwrap(), Jump(Address(0x03AB)));
}

#[test]
fn test_call()
{
    assert_eq!(decode(0x2FA2).unwrap(), Call(Address(0x0FA2)));
}

#[test]
fn test_skip_equal()
{
    assert_eq!(decode(0x3A1E).unwrap(), SkipEqualConstant(Register(0xA), Constant8(0x1E)));
}

#[test]
fn test_skip_not_equal()
{
    assert_eq!(decode(0x4A1E).unwrap(), SkipNotEqualConstant(Register(0xA), Constant8(0x1E)));
}

#[test]
fn test_skip_registers_equal()
{
    assert_eq!(decode(0x5330).unwrap(), SkipEqualRegister(Register(0x3), Register(0x3)));
}

#[test]
fn test_invalid_instructions_starting_with_5()
{
    assert!(decode(0x5331).is_none());
    assert!(decode(0x5332).is_none());
    assert!(decode(0x5333).is_none());
    assert!(decode(0x5334).is_none());
    assert!(decode(0x5335).is_none());
    assert!(decode(0x5336).is_none());
    assert!(decode(0x5337).is_none());
    assert!(decode(0x5338).is_none());
    assert!(decode(0x5339).is_none());
    assert!(decode(0x533A).is_none());
    assert!(decode(0x533B).is_none());
    assert!(decode(0x533C).is_none());
    assert!(decode(0x533D).is_none());
    assert!(decode(0x533E).is_none());
    assert!(decode(0x533F).is_none());
}

#[test]
fn test_load_byte()
{
    assert_eq!(decode(0x6A02).unwrap(), LoadConstant(Register(0xA), Constant8(0x02)));
}

#[test]
fn test_add_constant()
{
    assert_eq!(decode(0x7FBA).unwrap(), AddConstant(Register(0xF), Constant8(0xBA)));
}

#[test]
fn test_copy_register()
{
    assert_eq!(decode(0x8330).unwrap(), CopyRegister(Register(0x3), Register(0x3)));
}

#[test]
fn test_or_register()
{
    assert_eq!(decode(0x8331).unwrap(), OrRegister(Register(0x3), Register(0x3)));
}

#[test]
fn test_and_register()
{
    assert_eq!(decode(0x8332).unwrap(), AndRegister(Register(0x3), Register(0x3)));
}

#[test]
fn test_xor_register()
{
    assert_eq!(decode(0x8333).unwrap(), XorRegister(Register(0x3), Register(0x3)));
}

#[test]
fn test_add_register()
{
    assert_eq!(decode(0x8334).unwrap(), AddRegister(Register(0x3), Register(0x3)));
}

#[test]
fn test_sub_register()
{
    assert_eq!(decode(0x8335).unwrap(), SubRegister(Register(0x3), Register(0x3)));
}

#[test]
fn test_shift_right_register()
{
    assert_eq!(decode(0x8336).unwrap(), ShiftRightRegister(Register(0x3), Register(0x3)));
}

#[test]
fn test_minus_sub_register()
{
    assert_eq!(decode(0x8337).unwrap(), MinusSubRegister(Register(0x3), Register(0x3)));
}

#[test]
fn test_shift_left_register()
{
    assert_eq!(decode(0x833E).unwrap(), ShiftLeftRegister(Register(0x3), Register(0x3)));
}

#[test]
fn test_invalid_instructions_starting_with_8()
{
    assert!(decode(0x8008).is_none());
    assert!(decode(0x8009).is_none());
    assert!(decode(0x800A).is_none());
    assert!(decode(0x800B).is_none());
    assert!(decode(0x800C).is_none());
    assert!(decode(0x800D).is_none());
    assert!(decode(0x800F).is_none());
}

#[test]
fn test_skip_registers_not_equal()
{
    assert_eq!(decode(0x9330).unwrap(), SkipNotEqualRegister(Register(0x3), Register(0x3)));
}

#[test]
fn test_skip_registers_not_equal_invalid_instructions()
{
    assert!(decode(0x9331).is_none());
    assert!(decode(0x9332).is_none());
    assert!(decode(0x9333).is_none());
    assert!(decode(0x9334).is_none());
    assert!(decode(0x9335).is_none());
    assert!(decode(0x9336).is_none());
    assert!(decode(0x9337).is_none());
    assert!(decode(0x9338).is_none());
    assert!(decode(0x9339).is_none());
    assert!(decode(0x933A).is_none());
    assert!(decode(0x933B).is_none());
    assert!(decode(0x933C).is_none());
    assert!(decode(0x933D).is_none());
    assert!(decode(0x933E).is_none());
    assert!(decode(0x933F).is_none());
}

#[test]
fn test_load_address_constant()
{
    assert_eq!(decode(0xAFAB).unwrap(), LoadAddressConstant(Address(0xFAB)));
}

#[test]
fn test_jump_with_offset()
{
    assert_eq!(decode(0xBFAB).unwrap(), JumpWithOffset(Address(0xFAB)));
}

#[test]
fn test_random_generator()
{
    assert_eq!(decode(0xC23F).unwrap(), Rand(Register(0x2), Constant8(0x3F)));
}

#[test]
fn test_draw_generator()
{
    assert_eq!(decode(0xC23F).unwrap(), Rand(Register(0x2), Constant8(0x3F)));
}

#[test]
fn test_skip_key_equal()
{
    assert_eq!(decode(0xEF9E).unwrap(), SkipKeyEqual(Register(0xF)));
}

#[test]
fn test_skip_key_not_equal()
{
    assert_eq!(decode(0xEFA1).unwrap(), SkipKeyNotEqual(Register(0xF)));
}

#[test]
fn test_load_delay_timer()
{
    assert_eq!(decode(0xF307).unwrap(), LoadDelayTimer(Register(0x3)));
}

#[test]
fn test_wait_key_timer()
{
    assert_eq!(decode(0xF40A).unwrap(), WaitKey(Register(0x4)));
}

#[test]
fn test_store_delay_timer()
{
    assert_eq!(decode(0xF915).unwrap(), StoreDelayTimer(Register(0x9)));
}

#[test]
fn test_store_beep_timer()
{
    assert_eq!(decode(0xFB18).unwrap(), StoreBeepTimer(Register(0xB)));
}

#[test]
fn test_add_address()
{
    assert_eq!(decode(0xF01E).unwrap(), AddAddress(Register(0x0)));
}

#[test]
fn test_load_sprite_address()
{
    assert_eq!(decode(0xF229).unwrap(), LoadSpriteAddress(Register(0x2)));
}

#[test]
fn test_store_bcd()
{
    assert_eq!(decode(0xFF33).unwrap(), StoreBCD(Register(0xF)));
}

#[test]
fn test_context_store()
{
    assert_eq!(decode(0xF155).unwrap(), ContextStore(Register(0x1)));
}

#[test]
fn test_context_load()
{
    assert_eq!(decode(0xFC65).unwrap(), ContextLoad(Register(0xC)));
}

#[test]
fn test_invalid_instructions_starting_with_E()
{
    for op_code in 0xEF00..=0xEFFF {
        if op_code == 0xEFA1 || op_code == 0xEF9E
        {
            continue;
        }

        assert!(decode(op_code).is_none());
    }
}

#[test]
fn test_invalid_instructions_starting_with_F()
{
    for op_code in 0xFF00..=0xFFFF {
        if op_code == 0xFF07 || op_code == 0xFF0A || op_code == 0xFF15 || op_code == 0xFF18 ||
            op_code == 0xFF1E || op_code == 0xFF29 || op_code == 0xFF33 || op_code == 0xFF55 ||
            op_code == 0xFF65
        {
            continue;
        }

        assert!(decode(op_code).is_none());
    }
}