use text_io::read;

#[derive(Debug)]
enum Components {
    Value(i32),
    Plus,
}

fn parse_into_components(raw_string: String) -> Vec<Components> {
    raw_string
        .split_whitespace()
        .map(|x| match x.parse::<i32>() {
            Ok(n) => Components::Value(n),
            Err(_) => Components::Plus,
        })
        .collect()
}

fn compute_from_components(comp_array: Vec<Components>) -> String {
    let mut computed: i32 = 0;
    for component in comp_array {
        match component {
            Components::Value(n) => computed = computed + n,
            Components::Plus => (),
        }
    }
    computed.to_string()
}

fn main() {
    let a: String = read!("{}\n");
    let parsed = parse_into_components(a);
    println!("Parsed into {:?}", parsed);
    let answer = compute_from_components(parsed);
    println!("Final answer: {}", answer);
}
