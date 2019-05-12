fn main() {
    println!("Hello, world!");
    
    let _x = {
        let y = 3;
        y + 1
    };
    another_function(five(), -12);
}

// my awesome comment
fn another_function(x: u32, y: i32) {
    println!("the parameter x is: {}", x);
    println!("the parameter y is: {}", y);;
}

fn five() -> u32 {
    5
}