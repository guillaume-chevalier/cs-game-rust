use singer::Singer;

pub struct Rick {}

pub fn new() -> Rick {
    Rick {}
}

impl Singer for Rick {
    fn sing(&self) -> String {
        return String::from("Let's get schwifty in here !");
    }
}
