fn met_newt_port(f: fn(f64) -> f64, fp: fn(f64) -> f64, a: f64, e: f64, n: u128) -> f64 {
    let mut x = a;
    let mut xn = 0.0;
    for _i in 0..n {
        xn = x - f(x) / fp(x);
        if f(xn) <= e && xn - x <= e {
            break;
        }
        x = xn;
    }
    xn
}

fn met_newt_for(a: f64, e: f64, n: u128) -> f64 {
    //for
    let mut x = a;
    let mut xn = 0.0;
    for _i in 0..n {
        xn = x - f(x) / fp(x);
        if f(xn) <= e && xn - x <= e {
            break;
        }
        x = xn;
    }
    xn
}

fn met_newt_whl(a: f64, e: f64, n: u128) -> f64 {
    //while
    let mut x = a;
    let mut xn;
    let mut i = 0;
    xn = x - f(x) / fp(x);
    while (f(xn) > e || xn - x > e) && i < n {
        x = xn;
        i += 1;
        xn = x - f(x) / fp(x);
    }
    xn
}

fn met_newt_rek(a: f64, e: f64, n: u128) -> f64 {
    //rekur
    let x = a;
    let xn = x - f(x) / fp(x);
    if (f(xn) <= e && xn - x <= e) || n == 0 {
        return xn;
    }
    met_newt_rek(xn, e, n - 1)
}

fn met_newt(a: f64, e: f64, n: u128) -> f64 {
    //loop
    let mut x = a;
    let mut xn;
    let mut i = 0;
    loop {
        i += 1;
        xn = x - f(x) / fp(x);
        if (f(xn) <= e && xn - x <= e) || i > n {
            break;
        }
        x = xn;
    }
    xn
}

fn f(x: f64) -> f64 {
    x * x - 4.0
}

fn fp(x: f64) -> f64 {
    2.0 * x
}

fn f2(x: f64) -> f64 {
    x * x * x - 8.0
}

fn f2p(x: f64) -> f64 {
    3.0 * x * x
}

fn main() {
    println!("loop: {}", met_newt(1.0, 0.0001, 1000));
    println!("while: {}", met_newt_whl(1.0, 0.0001, 1000));
    println!("rek: {}", met_newt_rek(1.0, 0.0001, 1000));
    println!("for: {}", met_newt_for(1.0, 0.0001, 1000));
    println!("port: {}", met_newt_port(f2, f2p, 1.0, 0.0001, 1000));
}
