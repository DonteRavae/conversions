// A program to demonstrate basic and extended Euclidean Algorithm

/// Returns the Greatest Common Divisor (GCD)
pub fn basic_euclidean(num1: i32, num2: i32) -> i32 {
    if num2 == 0 {
        return num1;
    }

    basic_euclidean(num2, num1 % num2)
}

/// Returns the Greatest Common Divisor (GCD), `num1` coefficient, and `num2` coefficient
pub fn extended_euclidean(num1: i32, num2: i32, x1: &mut i32, y1: &mut i32) -> i32 {
    if num2 == 0 {
        *x1 = 1;
        *y1 = 0;
        return num1;
    }

    println!("{num1}/{num2}");

    let mut x2 = 0;
    let mut y2 = 0;

    let gcd = extended_euclidean(num2, num1 % num2, &mut x2, &mut y2);

    *y1 = x2 - y2 * (num1 / num2);
    *x1 = y2;

    println!("x1 = {x1}, y1 = {y1}");
    gcd
}

fn main() {
    // BASIC EUCLIDEAN

    let num1 = 10;
    let num2 = 15;
    let gcd = basic_euclidean(num1, num2);
    println!("GCD({num1},{num2}) = {gcd}");

    // EXTENDED EUCLIDEAN

    let num1 = 1080;
    let num2 = 400;
    let mut x = 0;
    let mut y = 0;
    let gcd = extended_euclidean(num1, num2, &mut x, &mut y);
    println!("GCD({num1},{num2}) = {gcd}; {num1}({x}) + {num2}({y})");
}
