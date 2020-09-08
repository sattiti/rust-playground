// The `derive` attribute automatically creates the implementation
// required to make this `struct` printable with `fmt::Debug`.
// #[derive(Debug)] attribute をつけたことによって、
// fmt::Debug {:?} marker で出力することができるようになった。
#[derive(Debug)]
struct DebugPrintable(i32);

#[derive(Debug)]
struct Person<'a>{
    name: &'a str,
    age: u8
}

// main {{{
fn main(){
    // DebugPrintable is printable.
    println!("{:?}", DebugPrintable(1));

    let name  = "Peter";
    let age   = 27;
    let peter = Person { name, age };

    // Pretty print
    println!("{:#?}", peter);
}
// }}}
