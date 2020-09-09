// Match
// Rust provides pattern matching via the match keyword,
// which can be used like a C switch.

// main {{{
fn main(){
    let num:i32 = 9;

    match num{
        1 => println!("One!"),
        2 | 3 | 4 | 5 | 6 | 7 => println!("Great then 1"),
        8..=19 => println!("Great then 8 and Less then 19"),
        _ => println!("More then 20"),
    }

    match num {
        0 => println!("Zero"),

        // マッチした値を n にバインディングする。
        n @ 1..=12 => println!("{}", n),

        _ => println!("Other"),
    }

}
// }}}
