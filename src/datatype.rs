// Data type
// main {{{
fn main(){
// Singed Integers {{{
// i8
// i16
// i32
// i64
// i128
// isize   ポインタサイズと同じ符号付き整数
// }}}
// Unsinged Integers {{{
// u8
// u16
// u32
// u64
// u128
// usize    ポインタサイズと同じ符号無し整数
// }}}
// Floating Point {{{
// f32
// f64
// }}}
// char {{{
// U+0000～U+D7FF, U+E000～U+10FFFF
// 4 bytes each.
// }}}
// bool {{{
// true
// false
// }}}
// unit {{{
// ()   Whose only possible value is an empty tuple: ().
// }}}
// tuple{{{
// (T1, T2, ...)
// }}}
// Array{{{
// [T; size]
let xs:[i32; 5] = [0, 1, 2, 3, 4];
// }}}
// Slice {{{
// スライスは配列に似ているが、コンパイル時にサイズが決定されていない。
// &[T]
println!("{:?}", &xs[1..4]);
// }}}
}
// }}}
