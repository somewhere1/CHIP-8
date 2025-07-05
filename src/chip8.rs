use crate::ram::Ram;
use crate::cpu::Cpu;
use crate::cpu;
pub struct Chip8{
    pub ram:Ram,
    pub cpu:Cpu,
}

impl Chip8{
    pub fn new() -> Self{
        Chip8{
            ram:Ram::new(),
            cpu:Cpu::new()
        }
    }
    pub fn load_from(&mut self,data: &Vec<u8> ){
        //let offset = 0x200;
        //println!("loading File.......");
        for i in 0..data.len(){
            self.ram.write_byte((cpu::PROGRAM_START  + (i as u16)),data[i]);

        }

    }
    pub fn run_instruction(&mut self){
        self.cpu.run_instruction(&mut self.ram);
    }
}