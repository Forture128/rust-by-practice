/// Chapter: 10.2. Traits: Defining Shared Behavior
/// This module defines a trait and two structs implementing the trait. It also provides a function
/// that returns a summarizable item based on a boolean switch.

pub trait Summary {
    /// Returns a summary of the item.
    fn summarize(&self) -> String;
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

/// Returns a summarizable item based on the given switch.
///
/// If `switch` is `true`, it returns a `NewsArticle`. If `switch` is `false`, it returns a `Tweet`.
///
/// # Examples
///
/// ```
/// let article = returns_summarizable(true);
/// let tweet = returns_summarizable(false);
///
/// println!("Article: {}", article.summarize());
/// println!("Tweet: {}", tweet.summarize());
/// ```

/// This code comment won't run
/// Because `if` and `else` have incompatible types
// fn returns_summarizable_origin(switch: bool) -> impl Summary {
//     if switch {
//         NewsArticle {
//             headline: String::from("Penguins win the Stanley Cup Championship!"),
//             location: String::from("Pittsburgh, PA, USA"),
//             author: String::from("Iceburgh"),
//             content: String::from(
//                 "The Pittsburgh Penguins once again are the best \
//                  hockey team in the NHL.",
//             ),
//         }
//     } else {
//         Tweet {
//             username: String::from("horse_ebooks"),
//             content: String::from("of course, as you probably already know, people"),
//             reply: false,
//             retweet: false,
//         }
//     }
// }

fn returns_summarizable(switch: bool) -> Box<dyn Summary> {
    if switch {
        Box::new(NewsArticle {
            headline: String::from("Penguins win the Stanley Cup Championship!"),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from(
                "The Pittsburgh Penguins once again are the best \
                 hockey team in the NHL.",
            ),
        })
    } else {
        Box::new(Tweet {
            username: String::from("horse_ebooks"),
            content: String::from("of course, as you probably already know, people"),
            reply: false,
            retweet: false,
        })
    }
}

fn main() {
    let article = returns_summarizable(true);
    let tweet = returns_summarizable(false);

    println!("Article: {}", article.summarize());
    println!("Tweet: {}", tweet.summarize());
}
