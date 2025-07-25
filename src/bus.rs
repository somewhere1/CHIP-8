use crate::keyboard::Keyboard;
use crate::display::Display;
use crate::ram::Ram;

pub struct Bus{
    ram:Ram,
    keyboard:Keyboard,
    dispaly:Display
}

impl Bus{
    pub fn new() -> Bus{
        Bus{
        ram:Ram::new(),
        keyboard:Keyboard::new(),
        dispaly:Display::new()
        }

    }
    pub fn ram_read_byte(&self,address: u16) -> u8{
        self.ram.read_byte(address)
    }

    pub fn ram_write_byte(&mut self,address:u16,value:u8){
        self.ram.write_byte(address,value);
    }

    pub fn debug_draw_byte(&mut self,b:u8,x:u8,y:u8) -> bool{
        self.dispaly.debug_draw_byte(b,x,y)
    }
    pub fn present_screen(&self)
    {
        self.dispaly.present();
    }
    pub fn clear_screen(&mut self){
        self.dispaly.clear()
    }
    pub fn key_pressed(&self,key_code:u8) -> bool{

        self.keyboard.key_pressed(key_code)
    }
}