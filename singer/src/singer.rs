pub struct Person {
    can_sing: bool,
}

pub fn new(can_sing: bool) -> Person {
    Person { can_sing: can_sing }
}

impl Person {
    pub fn sing(&self) -> String {
        if self.can_sing {
            return String::from("Let's get schwifty in here !");
        } else {
            return String::from("Baby Baby Baby hooo !");
        }
    }
}
