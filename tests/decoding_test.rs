extern crate chirp8;

use chirp8::processor;
use chirp8::processor::{OpCode, Address, Register, Constant8};
use chirp8::processor::OpCode::*;


#[test]
fn test_decode_fail()
{
    let op_code = processor::decode(0x0000);
    assert!(op_code.is_none());
}


#[test]
fn test_clear()
{
    let op_code = processor::decode(0x00E0).unwrap();
    assert_eq!(op_code, ClearDisplay);
}


#[test]
fn test_return()
{
    let op_code = processor::decode(0x00EE).unwrap();
    assert_eq!(op_code, Return);
}


#[test]
fn test_jump()
{
    let op_code1 = processor::decode(0x1000).unwrap();
    assert_eq!(op_code1, Jump(Address(0x0000)));

    let op_code2 = processor::decode(0x13AB).unwrap();
    assert_eq!(op_code2, Jump(Address(0x03AB)));

    let op_code3 = processor::decode(0x1FFF).unwrap();
    assert_eq!(op_code3, Jump(Address(0x0FFF)));
}

#[test]
fn test_call()
{
    let op_code1 = processor::decode(0x2000).unwrap();
    assert_eq!(op_code1, Call(Address(0x0000)));

    let op_code2 = processor::decode(0x2FA2).unwrap();
    assert_eq!(op_code2, Call(Address(0x0FA2)));

    let op_code3 = processor::decode(0x2FFF).unwrap();
    assert_eq!(op_code3, Call(Address(0x0FFF)));
}

#[test]
fn test_skip_equal()
{
    let op_code1 = processor::decode(0x30FF).unwrap();
    assert_eq!(op_code1, SkipEqualConstant(Register(0x0), Constant8(0xFF)));

    let op_code2 = processor::decode(0x3A1E).unwrap();
    assert_eq!(op_code2, SkipEqualConstant(Register(0xA), Constant8(0x1E)));
}

#[test]
fn test_skip_not_equal()
{
    let op_code1 = processor::decode(0x40FF).unwrap();
    assert_eq!(op_code1, SkipNotEqualConstant(Register(0x0), Constant8(0xFF)));

    let op_code2 = processor::decode(0x4A1E).unwrap();
    assert_eq!(op_code2, SkipNotEqualConstant(Register(0xA), Constant8(0x1E)));
}