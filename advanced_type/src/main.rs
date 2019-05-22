type Kilometers = i32;

type Thunk = Box<dyn Fn() + Send + 'static>;

fn takes_long_type(f: Thunk) {
    unimplemented!()
}

fn returns_long_type() -> Thunk {
    unimplemented!()
}

fn main() {
    let x: i32 = 5;
    let y: Kilometers = 5;
    println!("x + y = {}", x + y);

    let f: Thunk = Box::new(|| println!("hi"));

    let s1: &str = "hello there!";
    let s2: &str = "How's it going?";
}
