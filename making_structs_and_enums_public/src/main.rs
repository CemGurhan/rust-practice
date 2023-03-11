mod house;

use house::HouseStruct;

fn main() {
    let house1 = HouseStruct::create_house("window one", 24);
    println!("Our house: {:?}", house1);
}
