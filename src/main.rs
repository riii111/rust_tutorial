mod tasks;

use crate::tasks::fizz_buzz;
use crate::tasks::guessing_game;
use crate::tasks::leap_years;

fn main() {
    println!("1. 数当てゲーム");
    println!("2. FizzBuzz");
    println!("3. 閏年判定");

    let mut choice = String::new();
    std::io::stdin().read_line(&mut choice).expect("入力エラー");

    match choice.trim() {
        "1" => guessing_game::run(),
        "2" => fizz_buzz::run(),
        "3" => leap_years::run(),
        _ => println!("無効な選択です"),
    }
}
