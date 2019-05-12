
const MAX_POINTS: u32 = 100_000;

fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    println!("MAX_POINTS is: {}", MAX_POINTS);

    let b1: f64 = 51.0 / 2.0;
    println!("b1 is: {}", b1);

    let heart_eyed_cat = '😻';
    println!("char is: {}", heart_eyed_cat);

    let tup: (u8, i16, f64) = (0, 4, 32.0);
    println!("tup is: {:?}", tup);
    let (t1,t2,t3) = tup;
    println!("tup period ({} {} {})", tup.0, tup.1, tup.2);

    let months: [&str; 12] = ["january", "february", "march", "april", 
    "may", "june", "july", "august",
    "september", "october", "november", "december"];
    println!("array is: {:?}", months);

    let empty_array = [""; 12];
    println!("empty_array is: {:?}", empty_array);
}
