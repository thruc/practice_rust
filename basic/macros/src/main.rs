use std::io::Write;
fn main() {
    // 文字列操作マクロ
    // format
    let mut s = String::from("ab23");
    s = format!("{}-{:?}", s, ("D", 5));
    println!("{}",s);
    s = format!("{}{}","abc","def");
    println!("{}",s);

    // concat
    let s1= concat!("A", "b2", 3);
    println!("{}",s1);

    // データ出力マクロ
    print!("print");
    println!("println:{}","print");
    eprint!("eprint");
    eprintln!("eprintln:{}","eprint");

    // 標準エラー出力
    let mut w = Vec::new();
    write!(&mut w, "{}", "ABC");
    writeln!(&mut w, "{}", "is 123");
    dbg!(w);// 標準エラー出力

    

    //ベクタを宣言するマクロ
    let v = vec![1, 2, 3];
    println!("vec[0]:{}", v[0]);

    // プログラム外のリソースにアクセスするマクロ
    println!("def in file:{}", file!());
    println!("def in line:{}", line!());
    println!("is test:{}", cfg!(unix));
    println!("CARGO_HOME:{}", env!("CARGO_HOME"));

    // アサーション用のマクロ
    assert!(true); // tureかどうか
    assert_eq!(1, 1); // 等しいかどうか
    assert_ne!(1, 0); // 等しくないかどうか
    // cargo run --release でコンパイルが成功する (debug時のみ実行)
    debug_assert!(false);
    debug_assert_eq!(1, 1);
    debug_assert_ne!(1, 0);

    let mut p1 = HappyPerson {
        name: "Mike".to_string(),
        state: Emotion::Happy,
    };
    let mut p2 = HappyPerson {
        name: "Takeshi".to_string(),
        state: Emotion::Anger,
    };

    println!("{}", p1.get_happy());
    println!("{}", p2.get_happy());

    //異常終了用マクロ
    //panic!("it will panic");

}

//実装補助用マクロ
enum Emotion {
    Anger,
    Happy,
}

trait Emotional {
    fn get_happy(&mut self) -> String;
    fn get_anger(&mut self) -> String;
    fn tell_status(&self) -> String;
}

struct HappyPerson {
    name: String,
    state: Emotion,
}

impl Emotional for HappyPerson {
    fn get_anger(&mut self) -> String {
        unimplemented!();
    }

    fn get_happy(&mut self) -> String {
        format!("{} is always happy", self.name)
    }

    fn tell_status(&self) -> String {
        todo!()
    }
}