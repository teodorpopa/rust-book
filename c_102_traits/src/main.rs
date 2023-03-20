use c_102_traits::{Summary, Tweet, NewsArticle};

fn main() {
   let tweet = Tweet {
       username: String::from("horse_ebooks"),
       content: String::from("of course, as you probably already know, people"),
       reply: false,
       retweet: false,
   };

   println!("1 new tweet: {}", tweet.summarize());
   

   let article = NewsArticle {
       headline: String::from("Penguins win the Stanley Cup Championship!"),
       location: String::from("Pittsburg, PA, USA"),
       author: String::from("Iceburgh"),
       content: String::from(
           "The Pittsburg Penguins once again are the best \
           hockey tema in the NHL.",
        ),
        username: String::from("Teo"),
   };
   println!("New article available! {}", article.summarize());
}
