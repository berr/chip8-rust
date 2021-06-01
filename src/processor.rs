use std::{fs, io};
use std::io::Read;

pub struct Processor {
    memory: [u8; 4*1024],
    registers: [u16; 16],
    delay_timer: u16,
    sound_timer: u16,
    program_counter: u16,
    i_register: u16,
    stack: [u16; 12],
}

impl Processor {

    pub fn from_file(path: &str) -> io::Result<Self> {
        let mut memory = [0; 4*1024];


        let mut f = fs::File::open(path)?;
        let mut buf: Vec<u8> = vec![];
        f.read_to_end(&mut buf)?;

        let pc_base_address = 0x200 as u16;
        let mut i = 0;
        while i < buf.len()
        {
            memory[pc_base_address as usize + i] = buf[i];
            i += 1;
        }

        Ok(Processor{
            memory,
            registers: [0; 16],
            delay_timer: 0,
            sound_timer: 0,
            program_counter: pc_base_address,
            i_register: 0,
            stack: [0; 12],
        })
    }
}