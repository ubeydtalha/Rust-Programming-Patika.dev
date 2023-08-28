

fn print_summary<T:Summary>(item : &T){
    println!("Summary: {}", item.summarize());
}

trait Summary{
    fn summarize(&self) -> String;
}

fn print_double_summary<T,U>(
        item1 : T,
        item2 : U 
    ) 
    where 
        T:Summary, 
        U:Summary + Clone
    {


        println!("Summary: {}", item1.summarize());
        println!("Summary: {}", item2.summarize());
    
        let cloned_item2 = item2.clone();
        println!("Summary: {}", cloned_item2.summarize());
    
    }

struct NewsArticle{
    headline : String,
    location : String,
    author : String,
    content : String
}

impl Summary for NewsArticle{
    fn summarize(&self) -> String{
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

struct Tweet{
    username : String,
    content : String,
    reply : bool,
    retweet : bool
}

impl Summary for Tweet{
    fn summarize(&self) -> String{
        format!("{}: {}", self.username, self.content)
    }
}


impl Clone for Tweet{
    fn clone(&self) -> Tweet{
        Tweet{
            username : self.username.clone(),
            content : self.content.clone(),
            reply : self.reply,
            retweet : self.retweet
        }
    }
}


fn main() {
    
    let tweet = Tweet{
        username : String::from("horse_ebooks"),
        content : String::from("of course, as you probably already know, people"),
        reply : false,
        retweet : false
    };

    let article = NewsArticle{
        headline : String::from("Penguins win the Stanley Cup Championship!"),
        location : String::from("Pittsburgh, PA, USA"),
        author : String::from("Iceburgh"),
        content : String::from("The Pittsburgh Penguins once again are the best hockey team in the NHL.")
    };

    print_summary(&tweet);
    print_summary(&article);

    print_double_summary(article,tweet );


}
