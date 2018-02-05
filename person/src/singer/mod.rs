pub mod justin;
pub mod rick;

pub trait Singer {
    fn sing(&self) -> String;
}

pub fn new(can_sing: bool) -> Box<Singer> {
    if can_sing {
        return Box::new(rick::new());
    } else {
        return Box::new(justin::new());
    }
}
