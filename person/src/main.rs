mod singer;

fn main() {
    let singer = singer::new(false);
    println!("{}", singer.sing());
}
