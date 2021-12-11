use std::fmt::{Display, Debug};

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.summarize());

    notify(&tweet);
    notify(&article);
    // notify("herp derp"); // trait not implemented on String
}

// These two fn declarations are equivalent
// pub fn notify<T: Summary>(item: &T) {
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// Small differences, be aware of which one is better in which situation
// First one is good if we want to allow item1 and item2 to be able to have different types
// Second one is good if we want them to be the same type
// pub fn notify(item1: &impl Summary, item2: &impl Summary) {
// pub fn notify<T: Summary>(item1: &T, item2: &T) {

// We can also ask type to implement several Traits with the + operator
// pub fn notify(item: &(impl Summary + Display)) {
// pub fn notify<T: Summary + Display>(item: &T) {

fn some_function1<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
    69
}
// equivalent, using the where clause
fn some_function2<T, U>(t: &T, u: &U) -> i32 
    where T: Display + Clone,
          U: Clone + Debug
{
    69
}

fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}

// Can't return different types based on condition even though both 
// implement the Summary trait
fn returns_summarizable_not_allowed(switch: bool) -> impl Summary {
    if switch {
        NewsArticle {
            headline: String::from(
                "Penguins win the Stanley Cup Championship!",
            ),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from(
                "The Pittsburgh Penguins once again are the best \
                 hockey team in the NHL.",
            ),
        }
    } else {
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from(
                "of course, as you probably already know, people",
            ),
            reply: false,
            retweet: false,
        }
    }
}

pub trait Summary {
    fn summarize_author(&self) -> String;

    // fn summarize(&self) -> String; // must be implemented

    // default implementation. Can be overridden, but not necessary
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// Implements summarize
//impl Summary for NewsArticle {
    //fn summarize(&self) -> String {
        //format!("{}, by {} ({})", self.headline, self.author, self.location)
    //}
//}

// Uses the default implementation
impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

// Overrides the default implementation of summarize
impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }

    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

// implements the ToString trait on any type that implements the Display trait
impl<T: Display> ToString for T {
    // --snip--
}
