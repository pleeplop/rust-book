fn main() {
    let mut s = String::from("Hello, world!");
    let index_of = first_word(&s[..]);
    let index_of2 = first_word("pouet");
    println!("first word index_of is: {:?}", index_of);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i,&item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}