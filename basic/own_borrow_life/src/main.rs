struct Color {
    r: i32,
    g: i32,
    b: i32,
}

/*
fn calc_data(data: String) -> String {
    println!("{}",data);
    data
}*/
//借用
fn calc_data(data: &String) {
    println!("{}", data);
}
//RAII
struct Droppable;

impl Drop for Droppable {
    fn drop(&mut self) {
        println!("Resource will be released");
    }
}

fn main() {
    {
        // ムーブセマンティック
        let a = Color {
            r: 255,
            g: 255,
            b: 255,
        };
        let b = a; //所有権が譲渡される
                   //println!("{} {} {}", a.r, a.g, a.b);// 所有権がないのでエラー
        println!("{} {} {}", b.r, b.g, b.b);
    }
    //借用
    {
        /*
        let mut data ="important data".to_string();

        data = calc_data(data);

        println!("{}", data);
        */
        //以下のように参照を渡す方法を借用
        let data = "important data".to_string();

        calc_data(&data);

        println!("{}", data);

        let x = 5;
        let y = &x;
        let z = &x;
        dbg!(x);
        dbg!(y);
        dbg!(z);
    }
    {
        let mut x = 5;
        let y = &mut x;
        //let z = &mut x;//cannot borrow `x` as mutable more than once at a time//2回目の可変の参照渡し
        //dbg!(x);//cannot use `x` because it was mutably borrowed
        dbg!(y);
        //dbg!(z);
    }
    {
        let mut x = 5;
        let y = &x;

        
        //let z = &mut x;//cannot borrow `x` as mutable because it is also borrowed as immutable
        dbg!(y);
        //dbg!(z);

    }
    // RAII
    {
        {
            let d = Droppable;

        }
        println!("The Droppable should be released at the end of block")
    }
}
