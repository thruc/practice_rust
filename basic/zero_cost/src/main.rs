// tarit dyn
trait Tweet {
    fn tweet(&self);

    fn tweet_twice(&self) {
        self.tweet();
        self.tweet();
    }

    fn shout(&self) {
        println!("Uoooooohh");
    }
}

struct Dove;
struct Duck;


impl Tweet for Dove {
    fn tweet(&self) {
        println!("Coo");
    }
}

impl Tweet for Duck {
    fn tweet(&self) {
        println!("Quack");
    }
}

// ジェネリスク どのような型でも共用する
fn make_tuple<T, S>(t: T, s: S) -> (T, S) {
    (t, s)
}

fn main() {
    // tarit dyn
    let dove = Dove {};
    dove.tweet();
    dove.tweet_twice();
    dove.shout();

    let duck = Duck {};

    let bird_vec: Vec<Box<dyn Tweet>> = vec![Box::new(dove), Box::new(duck)];
    for bird in bird_vec {
        bird.tweet();
    }

    // ジェネリスク
    let t1 = make_tuple(1, 2);
    let t2 = make_tuple("hello", "word");
    let t3 = make_tuple(vec![1, 2, 3], vec![4, 5]);
    let t4 = make_tuple(3, "years old");
    println!("t1:{:?}", t1);
    println!("t2:{:?}", t2);
    println!("t3:{:?}", t3);
    println!("t4:{:?}", t4);


}
