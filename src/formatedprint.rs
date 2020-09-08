// print 関連の macro は std::fmt で定義されている。
// それらの macro は以下のものが含まれている。

// format!   Write formatted text to String.
// print!    Same as format!, but the text is printed to console(io::stdout).
// println!  Same as print!, but a newline is appended.
// eprint!   Same as format!, but the text is printed to the standard err(io::stderr).
// eprintln! Same as eprintln!, but a newline is appended.

// Marker
// fmt::Debug   {:?} marker. Format text for debugging purposes.
// fmt::Display {} marker. Format text in a more elegant, user friendly fashion.

// main {{{
fn main(){
    // 引数自動置換。
    println!("{} days.", 31);

    // 引数の位置から場所指定することができる。
    println!("{0}, this is {1}. {1}, here is {0}", "May", "Alice");

    // 名前指定することができる。
    println!("{subject} {verb} {object}",
        subject="He",
        verb="looks",
        object="tired");

    // Special formatting can be specified after a ":".
    // {:b} cast to binary.
    println!("i32: {} Binary: {:b}", 1, 2);

    // Right align text with specified width.
    // This will output "1     !"
    println!("{number:<width$}!", number=1, width=6);

    // This will output "1------!"
    println!("{number:-<width$}!", number=1, width=6);

    // This will output "     1!"
    println!("{number:>width$}!", number=1, width=6);

    // This will output "  1  !"
    println!("{number:^width$}!", number=1, width=6);

    // Pad numbers with extra zeroes.
    // This will output 000001
    println!("{number:>0width$}", number=1, width=6);

    // 出力時の桁数操作。
    let pi = 3.141592;
    println!("Pi is rougly {:.3}", pi);
}
// }}}
