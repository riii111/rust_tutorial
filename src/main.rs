mod tasks;

use crate::tasks::fizz_buzz;
use crate::tasks::guessing_game;

fn main() {
    println!("1. 数当てゲーム");
    println!("2. FizzBuzz");

    let mut choice = String::new();
    std::io::stdin().read_line(&mut choice).expect("入力エラー");

    match choice.trim() {
        "1" => guessing_game::run(),
        "2" => fizz_buzz::run(),
        _ => println!("無効な選択です"),
    }
}
