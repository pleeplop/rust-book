fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}

fn main() {
    let answer = do_twice(add_one, 5);
    println!("the answer is: {}", answer);
    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings_closure = list_of_numbers
        .iter()
        .map(|i| i.to_string())
        .collect();
    let list_of_strings_function = list_of_numbers
        .iter()
        .map(ToString::to_string)
        .collect();
}
