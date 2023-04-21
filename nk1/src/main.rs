fn delta(i: i32, j:i32) -> i32{
    let mut res : i32;
    if i == j{
        res = 1;
    }else{
        res = 0;
    }
    res
}

fn wymierna(x: i32) -> i32  {
    let mut res : i32 = 0;
    let mut abs_x : i32 = x.abs();
    if x!=0{
        res = 1/x;
    }else{
        res = -1;
    }
    res
}

fn silne_haslo(s: String) -> bool{
    let mut res :bool = true ;
    if s.len()<8{
        res = false;
        res
    }else{
        for i in s.chars() {
            if i.is_alphanumeric() {
                res = false;
            }else{
                res = true;
            }
        }
        res
    }
}
//druga metoda na zad 3, niestety nie dzialajaca
fn pwd_stength(s: String) -> bool{
    let mut res : bool = false;
    if s.len() < 8
        && s.bytes().any(|x| matches!('a'..='z'))
        && s.bytes().any(|x| matches!('A'..='Z'))
        && s.bytes().any(|x| matches!(33..=47 || 58..=64 || 91..=96 || 123..=126)){
        res = true;
    }else{
        res = false;
    }
    res
}

fn main() {
    let mut check = String::new();
    check = "aBcDeFgH!".parse().unwrap();
    println!("{}",silne_haslo(check));
    check = "aBcDeF!".parse().unwrap();
    println!("{}",silne_haslo(check));
    check = "M@rtinH3id3gg3r".parse().unwrap();
    println!("{}",silne_haslo(check));
}
