use singer::Singer;

pub struct Justin {}

pub fn new() -> Justin {
    Justin {}
}

impl Singer for Justin {
    fn sing(&self) -> String {
        return String::from("Baby Baby Baby hooo !");
    }
}
