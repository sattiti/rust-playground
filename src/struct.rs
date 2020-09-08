// Structures
// 3種類の構造体を作成することができる。
// 1. Tuple Structs
// 2. classic C structs
// 3. Unit Structs, which are field-less, are useful for generics.

#[allow(dead_code)]
struct Person<'a>{
    // The 'a defines a lifetime.
    name: &'a str,
    age: u8
}

// Unit struct
#[allow(dead_code)]
struct Nil;

// tuple struct
#[allow(dead_code)]
struct Pair(f64, f64);

// struct with field.
#[allow(dead_code)]
struct Point{
    x: f64,
    y: f64,
}

#[allow(dead_code)]
struct Rectangle{
    left_top:Point,
    right_bottom:Point,
}

// main{{{
fn main(){

}
// }}}
