use std::f64;
//use std::fmt;

// Функція y(x)
fn y(x: f64) -> f64 {
    (1.0 + 2.0 * x * x) * f64::exp(x * x)
}

// Ряд із n членів
fn s_n(x: f64, n: usize) -> f64 {
    let mut sum = 0.0;
    for i in 0..=n {
        let term = (2.0 * i as f64 + 1.0) / factorial(i as u64) as f64 * x.powi(2 * i as i32);
        sum += term;
    }
    sum
}

// Ряд із точністю eps
fn s_eps(x: f64, eps: f64) -> f64 {
    let mut sum = 0.0;
    let mut i = 0;
    loop {
        let term = (2.0 * i as f64 + 1.0) / factorial(i as u64) as f64 * x.powi(2 * i as i32);
        sum += term;
        if term.abs() < eps {
            break;
        }
        i += 1;
    }
    sum
}

// Обчислення факторіалу
fn factorial(n: u64) -> u64 {
    (1..=n).product::<u64>().max(1)
}

fn main() {
    let k = 10;
    let n = 5;
    let a = 0.0;
    let b = 1.0;
    let eps = 0.0001;
    let h = (b - a) / ((k - 1) as f64);

    println!("--------------------------------------------------------------");
    println!("{:>8} {:>15} {:>15} {:>15}", "x", "S-n=5(x)", "S-eps(x)", "y(x)");

    for i in 0..k {
        let x = a + (i as f64) * h;
        let s5 = s_n(x, n);
        let se = s_eps(x, eps);
        let exact = y(x);

        println!("{:>8.4} {:>15.6} {:>15.6} {:>15.6}", x, s5, se, exact);
    }
}
