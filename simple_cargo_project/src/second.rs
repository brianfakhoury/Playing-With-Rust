use std::time::Instant;

pub fn print_time(extra_string: String) {
    println!("{:?}", extra_string);
    let now = Instant::now();
    println!("{:#?}", now);
}
