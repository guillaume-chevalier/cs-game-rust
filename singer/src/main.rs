mod singer;

fn main() {
    let good_singer = singer::new(true);
    let bad_singer = singer::new(false);

    println!("{}", good_singer.sing());
    println!("{}", bad_singer.sing());
}
