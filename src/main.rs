use std::fs::File;
use std::io::Read;
use chip8::Chip8;
use bus::Bus;

mod ram;
mod chip8;
mod cpu;
mod bus;
mod keyboard;
mod display;

fn main() {
    let mut file =  File::open("data/INVADERS").unwrap();
    let mut data = Vec::<u8>::new();

    let b = file.read_to_end(&mut data);
    let mut my_chip = Chip8::new();
    my_chip.load_from(&data);
    //println!("{:?}",my_chip.ram);

    loop{
        my_chip.run_instruction();
    }
}
