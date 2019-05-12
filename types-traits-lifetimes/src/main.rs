use types_traits_lifetimes::{Tweet, Summary, NewsArticle, notify};
use std::fmt::Display;

struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }

    fn y(&self) -> &U {
        &self.y
    }

    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point { x: self.x, y: other.y}
    }
}

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Pair<T> {
        Self {x, y}
    }
}

impl<T: PartialOrd + Display> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
    fn announce_and_return_part(&self, announcement:  &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn main() {
    struct_lifetime();
}

fn struct_lifetime() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.')
        .next()
        .expect("Could not find a 'a'");
    let i = ImportantExcerpt { part: first_sentence };

}

fn lifetime() {
    let string2 = String::from("xyz");
    let string1 = String::from("abcd");
    let annoncement = "pouet";

    let result = longest(string1.as_str(), string2.as_str(), annoncement);
    println!("The longest string is {}", result);

}

fn longest<'a, T>(x: &'a str, y: &'a str, annoncement: T) -> &'a str
    where T: Display
{
    println!("Announcement! {}", annoncement);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn summarize() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from("The Pittsburgh Penguins once again are the best
    hockey team in the NHL."),
    };

    println!("New article available! {}", article.summarize());
    notify(article);

}

fn type_parameter() {
    let number_list = vec![34, 50, 25, 100, 65];
    let largest = largest_by_ref(&number_list);
    println!("The largest number is {}", largest);

    let pint = Point { x: 32, y: 43 };
    let pfloat = Point { x: 0.0, y: 0.0 };

}

fn largest<T: PartialOrd + Copy>(number_list: &[T]) -> T {
    let mut largest = number_list[0];
    for &number in number_list.iter() {
        if number > largest {
            largest = number;
        }
    }
    largest
}

fn largest_by_ref<T: PartialOrd>(number_list: &[T]) -> &T {
    let mut largest = &number_list[0];
    for number in number_list {
        if *number > *largest {
            largest = number;
        }
    }
    largest
}
