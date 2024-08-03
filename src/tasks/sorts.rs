use rand::rngs::ThreadRng;
use rand::{self, Rng};

pub fn run() {
    let mut rng: ThreadRng = rand::thread_rng();
    let mut arr: Vec<i32> = (0..10).map(|_| rng.gen_range(0..100)).collect();
    println!("{:?}", arr);

    loop {
        println!("実行したいソートを選択してください");
        println!("1. バブルソート");
        println!("2. 選択ソート");
        // println!("3. クイックソート");
        // println!("4. マージソート");

        let mut choice = String::new();
        std::io::stdin().read_line(&mut choice).expect("入力エラー");

        match choice.trim() {
            "1" => {
                println!("バブルソート実行前: {:?}", arr);
                bubble_sort(&mut arr);
                println!("バブルソート実行後: {:?}", arr);
                break;
            }
            "2" => {
                println!("選択ソート実行前: {:?}", arr);
                selection_sort(&mut arr);
                println!("選択ソート実行後: {:?}", arr);
                break;
            }
            // "3" => {
            //     println!("クイックソート実行前: {:?}", arr);
            //     quick_sort(&mut arr);
            //     println!("クイックソート実行後: {:?}", arr);
            //     break;
            // },
            // "4" => {
            //     println!("マージソート実行前: {:?}", arr);
            //     merge_sort(&mut arr);
            //     println!("マージソート実行後: {:?}", arr);
            //     break;
            // },
            _ => {
                println!("無効な選択です。もう一度お試しください");
                continue;
            }
        }
    }
}

fn bubble_sort(arr: &mut Vec<i32>) {
    let len = arr.len();
    for i in 0..len - 1 {
        let mut swapped = false;
        for j in 0..len - i - 1 {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
                swapped = true;
            }
        }
        println!("{}回目のソート, {:?}", i, arr);
        if !swapped {
            break;
        }
    }
}

fn selection_sort(arr: &mut Vec<i32>) {
    let len = arr.len();
    for i in 0..len {
        let mut min_idx = i;
        for j in (i + 1)..len {
            if arr[min_idx] > arr[j] {
                min_idx = j;
            }
        }
        if min_idx != i {
            arr.swap(i, min_idx);
        }
        println!("{}回目のソート, {:?}", i + 1, arr);
    }
}
