use traits::{Summary, Tweet, BlogPost, NewsArticle, notify, Pair};

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

    let post = BlogPost {
        title: String::from("My Title"),
        author: String::from("Foo Bar"),
        body: String::from("This is my blog post."),
        date: String::from("Tuesday"),
    };

    println!("1 new blog post: {}", post.summarize());

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

    notify(&article);

    let pair = Pair::new(5, 10);
    pair.cmp_display();
}
