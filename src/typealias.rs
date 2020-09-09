// Type Alias

type Inch = u64;

#[allow(non_camel_case_types)]
type u64_t = u64;


// main{{{
fn main(){
    let inches:Inch = 2 as u64_t;

    println!("{}", inches);
}
// }}}
