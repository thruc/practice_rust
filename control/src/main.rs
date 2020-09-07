fn main() {

    // if
    let num = 1;
    
    if 0 < num {
        println!("0 < num");
    } else if num < 0 {
        println!("num < 0");
    } else {
        println!("else");
    }

    // 型が全てそろっている必要がある
    let result = if 0 <= num {
        -num
    } else {
        num
    };

    println!("{}", result);

    // 繰り返し
    // loop
    let mut count = 0;

    let result = loop {
        println!("loop_count: {}", count);
        count += 1;
        if count == 10 {
            break count;
        }
    };

    println!("result: {}", result);

    // while
    count = 0;
    while count < 10 {
        println!("while_count: {}", count);
        count += 1;
    };

    // for 
    let mut _c: i32;
    
    for c in 0..10 {
        println!("for_count: {}", c);
    }

    let array = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

    for e in &array {
        println!("element: {}", e);
    }

    // ラベル
    'main: loop {
        println!("main loop start");
        loop {
            println!("sub loop start");

            
            

            println!("sub loop end");
            break 'main;// mainのloopもブレイクする
        }
    }

    // match
    let i: i32 = 1;
    match i {
        1 => println!("1"),
        2 => println!("2"),
        3 => println!("3"),
        _ => println!("misc"),// 全ての値にマッチ
    }

    enum Color {
        Red,
        Blue,
        Green,
    }

    let mut c = Color::Red;

    match c {
        Color::Red => println!("Red"),
        Color::Blue => println!("Blue"),
        Color::Green => println!("Green"),
    }
    c = Color::Blue;
    match c {
        Color::Red => println!("Red"),
        Color::Blue => println!("Blue"),
        Color::Green => println!("Green"),
    }
    c = Color::Green;
    match c {
        Color::Red => println!("Red"),
        Color::Blue => println!("Blue"),
        //Color::Green => println!("Green"), // 全ての列挙型を網羅しないとエラー
        _ => println!("misc"),// 全ての値にマッチ
    }

    let result: Result<i32, String> = Ok(100);
    let result_number = match result {
        Ok(number) => number,
        Err(messsage) => {
            println!("Error: {}", messsage);
            -1
        },
    };

    println!("result_number: {}", result_number);

    // Range
    for number in 0..5 {
        println!("range:{}", number);
    }

    // Iterator
    let it = Iter {
        current: 0,
        max: 10,
    };

    for num in it {
        println!("Iterator:{}", num);
    }


    
}

// Iterator
struct Iter {
    current: usize,
    max: usize,
}

impl Iterator for Iter {
    type Item = usize; // 出力する型の紐づけ

    // next()メソッド
    fn next(&mut self) -> Option<usize> {
        self.current += 1;
        if self.current - 1 < self.max {
            Some(self.current - 1)
        }else{
            None
        }
    }
}
