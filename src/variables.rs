// const   An unchangeable safe value.
// static  A possibly mutable variable with static lifetime.
//         Accessing or modifying a mutable static variable is unsafe.

static LANG:&str = "ja";
const LEN:i32 = 10;

// main {{{
fn main(){
    // int
    let an_int = 1u32;

    // copy an_int into `cp_int`
    let cp_int = an_int;

    // bool
    let a_bool = true;

    // tuple
    let unit = ();

    // unused value.
    // 変数名の先頭に _ をつけると、コンパイル時の警告を消すことができる。
    let _unused_val = 2u32;

    // mutable var
    let mut mut_num:f64 = 1.0;
    mut_num += 1.0;

    println!("{:?}", LANG);
    println!("{:?}", LEN);
    println!("{:?}", an_int);
    println!("{:?}", cp_int);
    println!("{:?}", a_bool);
    println!("{:?}", unit);

    // returns the size of a variable in bytes.
    println!("Bytes of mut_num is {:?}", std::mem::size_of_val(&mut_num))
}
// }}}
