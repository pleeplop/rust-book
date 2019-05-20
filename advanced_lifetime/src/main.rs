struct Context<'s>(&'s str);

struct Parser<'c, 's: 'c> {
    context: &'c Context<'s>,
}

impl<'c, 's> Parser<'c, 's> {
    fn parse(&self) -> Result<(), &'s str> {
        Err(&self.context.0[1..])
    }
    fn parse_context(context: Context) -> Result<(), &str> {
        Parser { context: &context }.parse()
    }
}

struct Ref<'a, T: 'a>(&'a T);

trait Red {}

struct Ball<'a> {
    diameter: &'a i32,
}

impl<'a> Red for Ball<'a> {}

fn main() {
    let num = 5;
    let obj = Box::new(Ball { diameter: &num }) as Box<dyn Red>;
}
