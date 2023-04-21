//Zad1 - 1 metoda
fn powtorki(t: &[i32]) -> Vec<i32> {
    let mut ret: Vec<i32> = Vec::new();
    let mut last = t[0] + 1;
    let mut count = 0;
    for i in t.iter() {
        if *i == last {
            count += 1;
        } else if count > 0 {
            for _ in 0..=count {
                ret.push(last);
            }
            count = 0;
        }
        last = *i;
    }
    ret
}
//druga metoda:
fn repeats(v: &Vec<i32>) -> Vec<&i32> {
    let mut result = Vec::new();
    for i in 1..(v.len() - 1) {
        if (v.get(i).unwrap() == v.get(i - 1).unwrap()
            || v.get(i).unwrap() == v.get(i + 1).unwrap())
        {
            result.push(v.get(i).unwrap());
        }
    }
    result
}
//ZAD2 : 1 sposób
fn unikalne(t: &[i32]) -> Vec<i32> {
    //map(|x| (x, zlicz(x,t))).filter(|p| p.1 == 1).map(|p| p.0);
    let mut ret: Vec<i32> = Vec::new();
    let mut t2 = t.to_owned();
    t2.sort_unstable();
    for w in t2.windows(3) {
        if w[0] != w[1] && w[1] != w[2] {
            ret.push(w[1]);
        }
    }
    let mut rev = t2.iter().rev();
    let (z, y) = (rev.next(), rev.next());
    if y != z {
        ret.push(*z.unwrap());
    }
    ret
}
//2 sposób
fn unique(v: &Vec<i32>) -> Vec<&i32> {
    let mut result = Vec::new();
    for i in 0..v.len() {
        let mut unique_val = true;
        for j in 0..v.len() {
            if v.get(i).unwrap() == v.get(j).unwrap() && i != j {
                unique_val = false;
                break;
            }
        }
        if unique_val {
            result.push(v.get(i).unwrap());
        }
    }
    result
}

fn zlicz_wiele(s1: &[i32], s2: &[i32]) -> i32 {
    let mut ret = 0;
    for i in s1.iter() {
        for j in s2.iter() {
            if i == j {
                ret += 1;
            }
        }
    }
    ret
}

fn unique2(v: &Vec<i32>) -> Vec<&i32> {
    let mut result = Vec::new();
    for i in v {
        if zlicz_wiele(v, &vec![*i]) == 1 {
            result.push(i);
        }
    }
    result
}

fn indeksy(wektor: &[i32], element: i32) -> Vec<usize> {
    let mut ret: Vec<usize> = Vec::new();
    for (i, j) in wektor.iter().enumerate() {
        if *j == element {
            ret.push(i);
        }
    }
    ret
}

fn indeksy2(wektor: &Vec<i32>, element: i32) -> Vec<i32> {
    let mut result = Vec::new();
    wektor
        .iter()
        .enumerate()
        .filter(|x| *(x.1) == element)
        .for_each(|y| result.push(y.0 as i32));

    result
}

fn main() {
    println!("{:?}", powtorki(&[1, 3, 4, 3, 3, 3, 3, 4, 1, 1, 6]));
    println!("{:?}", unikalne(&[1, 3, 4, 3, 3, 5, 3, 4, 1, 1, 6]));
    println!("{:?}", zlicz_wiele(&[1, 2, 1, 3], &[1, 2]));
    println!("{:?}", zlicz_wiele(&[1, 2, 1, 3], &[12]));
    println!("{:?}", zlicz_wiele(&[1, 2, 1, 3], &[1, 2, 2]));
    println!("{:?}", zlicz_wiele(&[1, 2, 1, 3], &[1, 2, 2, 1]));
    println!("{:?}", indeksy(&[1, 7, 7, 1, 3, 1, 3, 5, 6, 7], 7));
    println!("{:?}", ('a'..='z').collect::<Vec<_>>());
    println!("{:?}", (1..=10).map(|i| i * i).collect::<Vec<_>>());
    println!("{:?}", (1..=10).map(|i| i32::pow(2, i)).collect::<Vec<_>>());
    println!(
        "{:?}",
        (1..=20).map(|i| 1.0 / (i as f64)).collect::<Vec<_>>()
    );
    println!(
        "{:?}",
        (1..=100)
            .filter(|i| i % 3 == 0 && i % 4 != 0)
            .collect::<Vec<_>>()
    );
}
