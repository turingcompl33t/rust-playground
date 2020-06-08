// traits.rs

pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct Article {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for Article {
    fn summarize(&self) -> String {
        format!("{} by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

fn notify<T>(summarizable : &T) 
    where T : Summary
{
    println!("[NOTIFY] {}", summarizable.summarize());
}

fn main() {
    let article = Article {
        headline: String::from("A Headline"),
        location: String::from("Bermuda"),
        author: String::from("Alonzo Church"),
        content: String::from("Turing was wrong!")
    };

    let tweet = Tweet {
        username: String::from("turing"),
        content: String::from("feels good to have discovered universal computation"),
        reply: false,
        retweet: false
    };

    println!("{}", article.summarize());
    println!("{}", tweet.summarize());

    notify(&article);
    notify(&tweet);
}