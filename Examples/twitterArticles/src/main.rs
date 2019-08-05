pub trait Summary {
    fn summarize (&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
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

// This function only allows params that implements the trait Summary
pub fn notify(item: impl Summary) {
    println!("Breaking news: {}", item.summarize());
}

// This function only allows params that implements the trait Summary and Display(Trait bounds)
// Trait bound
// pub fn notify<T: Summary + Display>(item1: T, item2: T) {
//     println!("Breaking news: {}", item.summarize());
// }

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from("The Pittsburgh Penguins once again are the best
        hockey team in the NHL."),
    };

    println!("New article available! {}", article.summarize());
    println!("1 new tweet: {}", tweet.summarize());

    notify(tweet);
    notify(article);
}
