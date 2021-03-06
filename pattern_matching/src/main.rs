fn main() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {}, as background", color);
    } else if is_tuesday {
        println!("Tuesday is green day");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("using purle as the background color");
        } else {
            println!("Using orange as the background color");
        };
    } else {
        println!("Using blue as the background color");
    };

    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("top {}", top);
    };

    let v = vec!['a', 'b', 'c'];
    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    };

    let (x, y, z) = (1, 2, 3);

//    let (x, y) = (1, 2, 3);
    let point = (3, 5);
    print_coordinates(&point);

    let x = 3;
    match x {
        1 | 2 => println!("one or two"),
        3...5 => println!("three through five"),
        _ => println!("other"),
    };

    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {:?}", y),
        _ => println!("Default case, x = {:?}", x),
    };
    println!("at the end: x = {:?}, y = {:?}", x, y);

    let x = 'a';
    match x {
        'a'...'j' => println!("early ASCII letters"),
        'k'...'z' => println!("late ASCII letters"),
        _ => println!("something else"),
    };

    let p = Point { x: 0, y: 7 };
    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);
    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);

    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    };

    let msg = Message::ChangeColor(Color::Rgb(0, 160, 255));
    match msg {
        Message::Quit => println!("The quit variant has no data structure"),
        Message::Move { x, y } => println!("Move in the x direction {} and in the y direction \
        {}", x, y),
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(Color::Rgb(r, g, b)) =>
            println!("Change the color to red {}, green {} and blue {}", r, g, b),
        Message::ChangeColor(Color::Hsv(h, s, v)) =>
            println!("Change the color to hue {}, saturation {} and value {}", h, s, v),
    };

    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });

    foo(1, 2);

    let mut setting_value = Some(5);
    let new_setting_value = Some(10);
    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => println!("Can't overwrite an existing customized value"),
        _ => setting_value = new_setting_value,
    };
    println!("setting is {:?}", setting_value);

    let numbers = (1, 2, 3, 4, 5);
    match numbers {
        (first, _, third, _, fifth) => println!("Some numbers: {}, {}, {}", first, third, fifth),
    };
    let _x = 5;
    let y = 6;

    let s = Some(String::from("Hello!"));
    if let Some(_s) = s {
        println!("found a string");
    };
//    println!("{:?}", s);

    match p {
        Point { x, .. } => println!("x is {}", x),
    };
    match numbers {
        (first, .., fifth) => println!("first is {} and fith is {}", first, fifth),
    };
    let num = Some(4);
    match num {
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    };
    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("matched n = {:?}", n),
        _ => println!("Default case x = {:?}", x),
    }

    let x = 4;
    let y = false;
    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("false"),
    }
    let msg = SimpleMessage::Hello { id: 5 };
    match msg {
        SimpleMessage::Hello { id: id_variable @ 3...7 } => println!("Found an id in range {}",
                                                                     id_variable),
        SimpleMessage::Hello { id: 10...12 } => println!("Found an id in another range"),
        SimpleMessage::Hello { id } => println!("Found some other id {}", id),
    };
}

enum SimpleMessage {
    Hello { id: i32 },
}

fn foo(_: i32, y: i32) {
    println!("This code only use the y parameter {}", y);
}

enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

struct Point {
    x: i32,
    y: i32,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location ({},{})", x, y);
}
