fn main() {
    // 参照
    let s1: String = String::from("hello word");
    let s2: &str = &s1;
    let s3: String = s2.to_string();

    println!("{}",s1);
    println!("{}",s2);
    println!("{}",s3);

    // タプル
    let mut t = (1, "2");
    println!("タプル0={0},タプル1={1}", t.0, t.1);
    t.0 = 2;
    t.1 = "3";
    println!("タプル0={0},タプル1={1}", t.0, t.1);

    // 配列
    let mut a: [i32; 3] = [0, 1, 2];
    let b: [i32; 3] = [0; 3];

    println!("{:?}", &a[1..3]);

    a[1] = b[1];
    a[2] = b[2];

    println!("{:?}", &a[1..3]);

    // ユーザー定義型
    struct Person {
        name: String,
        age: u32,
    }

    let p = Person {
        name: String::from("John"),
        age: 8,
    };

    println!("{}", p.name);
    println!("{}", p.age);

    // enum
    /*
    enum Event {
        Quit,
        KeyDown(u8),
        MouseDown { x: i32, y: i32},
    }
    let e1 = Event::Quit;
    let e2 = Event::MouseDown { x: 10, y: 10 };
    let e3 = Event::KeyDown (1);
    */

    // 表示ライブラリの型

    // Option
    /*
    pub enum Option<T>{
        None,
        Some(T),
    }*/

    // Result
    /*pub enum Result<T, E>{
        Ok(T),
        Err(E),
    }*/

    let result: Result<i32, String> = Ok(200);

    match result {
        Ok(code) => println!("code: {}", code),
        Err(err) => println!("code: {}", err),
    }

    // Vec

    let v1 = vec![1, 2, 3, 4, 5]; // 1~5で初期化
    let v2 = vec![0; 5]; // 0を5つで初期化

    println!("{}", v1[3]); // []で要素を取得

    for e in &v1 {
        println!("{}", e);
    }

    for e in &v2 {
        println!("{}", e);
    }

    // Box
    let byte_array = [b'h', b'e', b'1', b'1', b'o'];
    print(Box::new(byte_array));
    let byte_array = [b'w', b'o', b'r', b'd', b'!'];
    print(Box::new(byte_array));

}

// Box
fn print(s: Box<[u8]>){
    println!("{:?}", s)
}
