use crate::ram::Ram;
pub const PROGRAM_START:u16 = 0x200;

pub struct Cpu{
    vx:[u8;16],
    pc:u16,
    i:u16,

}

impl Cpu{
    pub fn new() -> Self{
        Cpu{
            vx:[0;16],
            pc:PROGRAM_START,
            i:0,
        }
    }

    pub fn run_instruction(&mut self,ram:&mut Ram){
        //利用self.pc取指令
        let run_instruction = ram.mem[self.pc as usize];
        //pc+1
        self.pc +=1 ;
        //解析指令： 操作码 -- 寄存器
        
    }
}