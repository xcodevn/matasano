// check if the number is prime or not
fn is_prime_number(num: uint) -> bool {
    if num < 2 { return false; }
    
    let mut count : uint = 2;
    while count * count < num {
        if num % count == 0 { return false; }
        count = count + 1;
    }

    return true;
}

fn main() {
    println!("Hello, world!")
}
