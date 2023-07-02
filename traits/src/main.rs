use core::fmt::Debug;
use core::fmt::Display;

trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
    fn title(&self) -> String {
        String::from("(No title)")
    }
    fn text_type(&self) -> String {
        String::from("text")
    }
}

#[derive(Debug)]
struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String,
}
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
    fn title(&self) -> String {
        format!("{}", self.headline)
    }
    fn text_type(&self) -> String {
        String::from("article")
    }
}

struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool,
}
impl Summary for Tweet {
    // cannot be split into several impl blocks
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
    fn title(&self) -> String {
        format!("{}", self.content)
    }
    fn text_type(&self) -> String {
        String::from("tweet")
    }
}

#[derive(Debug)]
struct GenericText {
    content: String,
}
impl Summary for GenericText {}

trait DefaultCallOther {
    fn fun1(&self) {
        println!("fun1<default>");
        self.fun2();
        println!("end fun1<default>");
    }
    fn fun2(&self);
}

struct CallOther {}
impl DefaultCallOther for CallOther {
    fn fun2(&self) {
        println!("fun2<CallOther>");
    }
}

struct Pair<T> {
    x: T,
    y: T,
}
impl<T> Pair<T> {
    fn new(x:T, y:T) -> Self {
        Self { x, y }
    }
}
// conditionally implement only if T implements Display and PartialOrd
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
impl<T: Display> Display for Pair<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

trait MyDisplay {
    fn my_display(&self);
}
// conditionally implement for types implementing trait Display
impl<T: Display> MyDisplay for T {
    fn my_display(&self) {
        println!("{self}");
    }
}

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
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };
    let gen_text = GenericText {
        content: String::from("Default implementations can call other methods in the same trait"),
    };
    notify(&tweet);
    notify2(&article);
    notify3(&gen_text);
    let summarizable = gen_summarizable();
    let result = notify_pair(&summarizable, &1234);
    println!("result={result}");

    let call_other = CallOther {};
    call_other.fun1();

    let pair1 = Pair { x: 1, y: 2 };
    let pair2 = Pair { x: 20, y: 10 };
    let pair3 = Pair { x: Pair { x: 0, y:0 }, y: Pair { x: -1, y: -2 } };
    pair1.cmp_display();
    pair2.cmp_display();
    //pair3.cmp_display();
    pair1.my_display();
}

fn notify(item: &impl Summary) {
    println!("1 new {}: {}", item.text_type(), item.title());
    println!("    summary: {}", item.summarize());
}

fn notify2(item: &(impl Summary + Debug)) {
    println!("1 new {}: {}", item.text_type(), item.title());
    println!("    summary: {}", item.summarize());
    dbg!(item);
}

fn notify3<T: Summary + Debug>(item: &T) {
    println!("1 new {}: {}", item.text_type(), item.title());
    println!("    summary: {}", item.summarize());
    dbg!(item);
}

fn notify_pair<T, U>(item: &T, value: &U) -> U
where
    T: Summary + Debug,
    U: Display + Copy,
{
    println!("1 new {}: {}", item.text_type(), item.title());
    println!("    summary: {}", item.summarize());
    dbg!(item);
    println!("value={value}");
    *value
}

fn gen_summarizable() -> impl Summary + Debug {
    GenericText {
        content: String::from("A text that can be summarized."),
    }
}
