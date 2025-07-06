
pub struct Keyboard{


}
impl Keyboard{
    pub fn new() -> Self{
            Keyboard{

            }
    }

    //Todo implement proper key handing
    pub fn key_pressed(&self,key_code:u8)-> bool{
        if key_code == 4{
            false
        }
        else{
            true
        }
    }
}