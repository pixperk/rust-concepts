trait Summarizable {
    fn summarize(&self) -> String;
}

struct Novel {
    title: String,
    author: String,
}


impl Summarizable for Novel {
    fn summarize(&self) -> String {
        format!("'{}' by {}", self.title, self.author)
    }
}

struct Tweet {
    author: String,
    pages: i32,
}

impl Summarizable for Tweet {
    fn summarize(&self) -> String {
        format!("Tweet by {} with {} pages.", self.author, self.pages)
    }
}

struct Game{
    name : String,
    multiplayer : bool,
}

impl Summarizable for Game{
    fn summarize(&self) -> String {
        format!("{} game is multiplayer : {}", self.name, self.multiplayer)
    }
}

fn main() {
    let readables: Vec<Box<dyn Summarizable>> = vec![
        Box::new(Novel {
            title: String::from("Harry Potter"),
            author: String::from("J.K. Rowling"),
        }),
        Box::new(Tweet {
            author: String::from("xyz"),
            pages: 5,
        }),
        Box::new(Game {
            name: String::from("Minecraft"),
            multiplayer: false,
        }),
    ];

    for readable in &readables {
        println!("{}", readable.summarize());
    }
}
