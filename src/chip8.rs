use crate::ram::Ram;

pub struct Chip8{
    pub ram:Ram
}

impl Chip8{
    pub fn new() -> Self{
        Chip8{
            ram:Ram::new()
        }
    }
    pub fn load_from(&mut self,data: &Vec<u8> ){
        let offset = 0x200;
        for i in 0..data.len(){
            self.ram.write_byte((offset+1) as u16,data[i]);
        }

    }
}