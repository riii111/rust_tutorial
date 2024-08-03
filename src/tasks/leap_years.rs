pub fn run() {
    let years_check_list: Vec<i32> = vec![2000, 2004, 2008, 2013, 2100, 2400];

    for year in years_check_list {
        if is_leap_year(year) {
            println!("{:04}年は、閏年！", year);
        } else {
            println!("{:04}年は、閏年ではない", year);
        }
    }
}

fn is_leap_year(year: i32) -> bool {
    match (year % 4, year % 100, year % 400) {
        (_, _, 0) => true,  // 400で割り切れる
        (0, 0, _) => false, // 100で割り切れて400で割り切れない
        (0, _, _) => true,  // 上記以外で4で割り切れる
        _ => false,
    }
}
