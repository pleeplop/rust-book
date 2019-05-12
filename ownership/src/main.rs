fn main() {
    let mut s = String::from("hello");  // s comes into scope
    let s1 = &s;
    let s2 = &s;
    let length = calculate_length(s2);
    println!("{} {}, length={}", s1, s2, length);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str(", world");
}

fn dangle() -> &String {
    let s = String::from("hello");
    &s
}