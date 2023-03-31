fn rand(seed: &mut u32, min_rand: u32, max_rand: u32) -> u32 {
    let mut tmp: u32 = *seed;
    *seed += 1;
    for i in 1..3 {
        tmp *= *seed + i;
    }
    tmp = tmp % (max_rand - min_rand) + min_rand;
    tmp
}

fn rand2(seed: &mut f32, min_rand: f32, max_rand: f32) -> f32 {
    *seed = (75.0 * *seed + 74.0) % 65537.0;
    *seed % (max_rand - min_rand + 1.0) + min_rand
}

fn swap_arr(arr: &mut [i32], i: usize, j: usize) {
    let tmp = arr[i];
    arr[i] = arr[j];
    arr[j] = tmp;
}

fn rand_perm(arr: &mut [i32], seed: &mut u32) {
    let seed = seed;
    let mut tmp;
    let length = (arr.len() - 1) as u32;
    let mut _index = 0;
    for _i in 0..arr.len() {
        tmp = rand(seed, 0, length);
        swap_arr(arr, _index, tmp as usize);
        _index +=1 ;
    }
}

fn rand_perm2(arr: &mut [i32], seed: &mut u32) {
    for i in 0..arr.len() {
        swap_arr(arr, i, rand(seed, 0, arr.len() as u32) as usize);
    }
}

fn liczba_wystapien(napis : String, znak : char) -> u32{
    let mut counter = 0;
    for _i in napis.chars(){
        if _i==znak{
            counter += 1;
        }
    }
    counter
}

fn co_drugi_znak(napis: String) -> String {
    let mut output = String::new();
    let mut counter = 0;
    for _i in ite{
        if counter % 2 == 0 {
            output.push(_i);
        }
        counter+=1;
    }
    output
}

/*
fn co_drugi_znak_bez_ifa(napis: String) -> String {
    let mut output = String::new();
    let mut counter = 0;
    let new_string : Vec<char>  = napis.chars().collect();
    for _i in new_string.skip(2){
        output.push(new_string);
        _i+=1;
    }
    output
}
 */

fn szyfruj(napis: String, klucz:u32) -> String{
    let mut wynik = String::new();
    let mut tmp = String::new();
    let mut appearances = 0;
    for i in napis.chars(){
        tmp.push(i);
        appearances+=1;
        if appearances == klucz{
            for j in tmp.chars().rev(){
                wynik.push(j);
            }
            appearances=0;
            tmp = String::new();
        }
    }
    wynik.push_str(&tmp);
    wynik
}

fn wizytowka(imie: String, nazwisko:String) -> Option<String>{
    if imie.chars().nth(0).unwrap().is_numeric(){
        return None
    }
    for i in nazwisko.chars(){
        if i.is_numeric(){
            return None;
        }
    }
    let mut result = String::new();
    let tmp = imie.chars().nth(0).unwrap().to_ascii_uppercase();

    result.push(tmp);
    result.push_str(". ");
    let mut temp_chars = nazwisko.chars();
    result.push(temp_chars.nth(0).unwrap().to_ascii_uppercase());
    for i in temp_chars{
        result.push(i.to_ascii_lowercase());
    }
    Some(result)
}

fn rome_to_arabic(num: char) -> Option<u32>{
    let rome = ['I','V','X','L','C','D','M'];
    let arabic = [1,5,10,50,100,500,1000];
    for i in 0..7{
        if num == rom[i]{
            return Some(arabic[i]);
        }
    }
    None
}

fn rzymskie(napis : String) -> Option<u32>{
    let rome = ['I','V','X','L','C','D','M'];
    let arabic = [1,5,10,50,100,500,1000];
    let mut result: u32 = 0;
    let mut num1 = 10000;
    let mut num2 = 0;

    for i in napis.chars(){
        match rome_to_arabic(i) {
            Some(sth) => num2 = sth,
            None => return None,
        }
        if num2 > num1{
            result -= num1 * 2;
        }
        result += num2;
        num1 = num2;
    }
    Some(result)
}

fn na_rzymskie(liczba: u32) -> String{
    let mut roman = vec!(
        (1000,"M"),
        (900,"CM"),
        (500,"D"),
        (400,"CD"),
        (100,"C"),
        (90,"XD"),
        (50,"L"),
        (40,"XL"),
        (10,"X"),
        (9,"IX"),
        (5,"V"),
        (4,"IV"),
        (1,"I")
    );
    let mut result = String::new();
    let mut tmp = liczba;
    let mut i=0;

    while tmp > 0{
        for el in &roman{
            if el.0<=tmp{
                result.push_str(el.1);
                tmp -= el.0;
                break;
            }
        }
    }
    result
}


fn main() {
    let mut seed: u32 = 5;
    let mut arr = [1, 2, 3, 4, 5];
    println!("{}", rand(&mut seed, 0, 100));
    println!("{}", rand(&mut seed, 0, 100));
    println!("{}", rand(&mut seed, 0, 100));
    swap_arr(&mut arr, 2, 3);
    println!("{:?}", arr);
    rand_perm(&mut arr, &mut seed);
    println!("{:?}", arr);
    let napis = "Hello".to_string();
    println!("{}",liczba_wystapien(napis.clone(),'l'));
    println!("{}",co_drugi_znak(napis));
}
