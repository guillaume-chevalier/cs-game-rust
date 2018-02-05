pub struct Person {
    pub name: String,
    can_sing: bool,
}

pub fn new(name: &str, can_sing: bool) -> Person {
   Person {
       name: String::from(name),
       can_sing: can_sing,
   }
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
