use std::fs::File;
use std::io::Read;
use chip8::Chip8;
mod ram;
mod chip8;

fn main() {
    let mut file =  File::open("data/INVADERS").unwrap();
    let mut data = Vec::<u8>::new();
    println!("{:?}",data);
    let b = file.read_to_end(&mut data);
    println!("{:?}",data );

    let mut my_chip = Chip8::new();
    my_chip.load_from(&data);
    println!("{:?}",my_chip.ram);
}
