use std::io;
fn main() {
    println!("Guess the number!"); // 数 を 当 て て ご ら ん
    println!("Please input your guess."); // ほ ら 、 予 想 を 入 力 し て ね
    let mut guess = String::new();
    io::stdin().read_line(&mut guess)
        .expect("Failed to read line"); // 行 の 読 み 込 み に 失 敗 し ま し た
    println!("You guessed: {}", guess); // 次 の よ う に 予 想 し ま し た: {}
}
