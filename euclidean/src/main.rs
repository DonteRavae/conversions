// A program to demonstrate basic and extended Euclidean Algorithm

pub fn find_gcd_using_basic_euclidean(num1: i32, num2: i32) -> i32 {
    if num2 == 0 {
        return num1;
    }

    find_gcd_using_basic_euclidean(num2, num1 % num2)
}

pub fn find_gcd_using_extended_euclidean(_num1: i32, _num2: i32) -> i32 {
    // find_gcd_using_extended_euclidean(num1, num2)
    todo!()
}

fn main() {
    // BASIC EUCLIDEAN

    let num1 = 10;
    let num2 = 15;
    let gcd = find_gcd_using_basic_euclidean(num1, num2);
    println!("GCD({num1},{num2}) = {gcd}");
}
