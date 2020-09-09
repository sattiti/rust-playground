// Functions
fn is_divisible(a:u32, b:u32) -> bool {
    if b == 0 {
        return false;
    }
    a % b == 0
}

fn fizzbuzz(n: u32) -> (){
    if is_divisible(n, 15) {
        println!("fizzbuzz");
    }
    else if is_divisible(n, 3) {
        println!("fizz");
    }
    else if is_divisible(n, 5) {
        println!("buzz");
    }
    else {
        println!("{}", n);
    }
}

// main
fn main(){
    for n in 1..20 {
        fizzbuzz(n);
    }

}
