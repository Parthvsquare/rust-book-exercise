fn main() {
    println!("Hello, world!");
    traits_type();
    default_traits();
}

fn traits_type() {
    pub trait Summary {
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

    let tweet = Tweet {
        username: String::from("horse_book"),
        content: String::from("horse_book_content"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    pub fn notify<T: Summary>(items: T) {
        println!("Breaking news! {}", items.summarize())
    }

    notify(tweet)
}

fn default_traits() {
    pub trait Summary {
        fn summarize(&self) -> String;
        fn summarize_author(&self) -> String {
            format!("Read more from ...{}", self.summarize())
        }
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

    let tweet = NewsArticle {
        headline: String::from("horse_book"),
        content: String::from("horse_book_content"),
        author: String::from("jatt"),
        location: String::from("50000"),
    };

    println!("1 new tweet: {}", tweet.summarize_author())
}
