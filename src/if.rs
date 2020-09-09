// if/else


// main {{{
fn main(){
    let n = 5;
    let size = 10;

    if n < 0 {
        println!("{} is negative", n);
    }
    else if n > 0 {
        println!("{} is positive", n);
    }
    else {
        println!("{} is zero", n);
    }

    let big_n =
        if n < size && n > -size {
            size * n
        }
        else {
            n / 2
        };

    println!("{} < {}", n, big_n);
}
// }}}
