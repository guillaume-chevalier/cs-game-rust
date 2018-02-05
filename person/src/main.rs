mod person;

fn main() {
    let rick = person::new("Rick", true);
    println!(
        "My name is {} and let me sing something for you !",
        rick.name
    );
    println!("{}", rick.sing());
}
