
// fn 
fn add(a: i32, b:i32) -> i32 {
    a + b
}

fn abs(number: i32) -> i32 {

    if number < 0 {
        return -number;
    }
    number 
}

// impl
struct Person {
    name: String,
    age: u32,
}

impl Person {
    fn say_name(&self) -> &Self{
        println!("I am {}", self.name);
        self // メソッドチェーン
    }

    fn say_age(&self) -> &Self{
        println!("I am {} years old", self.age);
        self // メソッドチェーン
    }
}

fn main() {

    // fn
    let x = add(1, 2);
    println!("x = {}", x);
    let y = abs(-1);
    println!("y = {}", y);

    // impl
    let p1 = Person{
        name: String::from("Taro"),
        age: 20,
    };

    let p2 = Person{
        name: String::from("Mike"),
        age: 18,
    };

    p1.say_name();
    p1.say_age();
    p2.say_name().say_age(); // メソッドチェーン

}
