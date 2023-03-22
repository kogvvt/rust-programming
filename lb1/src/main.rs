fn is_leap(year: i64) -> bool {
    let result;
    if year % 4 == 0 && year % 100 != 0 || year % 400 == 0 {
        println!("Leap!");
        result = true;
    } else {
        println!("Not leap");
        result = false;
    }
    result
}

fn zad2() {
    let month = 3;
    let year = 2023;
    if month == 1
        || month == 3
        || month == 5
        || month == 7
        || month == 8
        || month == 10
        || month == 12
    {
        println!("this month has 31 days!");
    } else if month == 2 {
        let days;
        if is_leap(year) {
            days = 29;
            println!("this month has {} days", days);
        } else {
            days = 28;
            println!("this month has {} days", days);
        }
    } else {
        println!("this month has 30 days!");
    }
}

fn zad4(temp : f64) ->f64 {
    (temp-32.0) * 5.0/9.0
}

fn zad3(temp : f64) ->f64 {
        (temp * 9.0) / 5.0 +32.0
}

fn zad5(hh1 : i32, mm1 : i32 , ss1 : i32, hh2 : i32, mm2 : i32, ss2 : i32) {
    let hh1 = 10;
    let hh2 = 8;
    let mm1 = 35;
    let mm2 = 24;
    let ss1 = 40;
    let ss2 = 25;
    // tot_sec1 = h1*24*60
    let tot_sec1 = ss1 + mm1 * 60 + hh1 * 3600;
    let tot_sec2 = ss2 + mm2 * 60 + hh2 * 3600;
    let sec_diff = tot_sec1 - tot_sec2;
    let res_hh = sec_diff / 3600;
    let res_mm = (sec_diff % 3600) / 60;
    let res_ss = sec_diff % 60;
    println!(
        "difference between times is : {} hours, {} minutes and {} seconds",
        res_hh, res_mm, res_ss
    );
}
fn zad6() {
    let mut result = 1;
    let mut ite = 1;
    let factorial = 4;
    while ite <= factorial {
        result *= ite;
        ite += 1;
    }
    println!("factorial of {} equals {}", factorial, result);
}

fn zad7() {
    let mut input = 234;
    let mut i = 1;
    while input > 0 {
        println!("Number on position {} is {}", i, input % 10);
        input = input / 10;
        i += 1;
    }
}
fn zad8() {
    let mut input = 234;
    let mut result = 0;
    while input > 0 {
        result += input % 10;
        input = input / 10;
    }
    println!("result = {}", result)
}
fn zad9() {
    let a = 3;
    let b = 4;
    let c = 5;

    let mut ite1 = a;
    while ite1 > 0 {
        let mut ite2 = b;
        while ite2 > ite1 {
            let mut ite3 = c;
            while ite3 > ite2 {
                if ite1 * ite1 + ite2 * ite2 == ite3 * ite3 {
                    println!("{}, {}, {} is a pythagorean triangle!", ite1, ite2, ite3);
                } else {
                    println!("This is not a pythagorean triangle!");
                }
                ite3 -= 1;
            }
            ite2 -= 1;
        }
        ite1 = -1;
    }
}

fn main() {
    zad2();
    zad3(43.0);
    zad4(100.0);
    zad5(22,33,44,24,44,55);
    zad6();
    zad7();
    zad8();
    zad9();
}