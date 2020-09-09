// Loop

// main {{{
fn main(){
    let end = 10;
    let mut n = 1;

    // loop {{{
    // loop は while 1 {} に相当。
    println!("loop");

    loop {
        println!("{}", n);
        n += 1;

        if n == 3 {
            println!("continue.");
            continue;
        }
        else if n == 6 {
            println!("{}", n);
        }
        else if n >= end {
            println!("DONE: {}", n);
            break;
        }
        else{
            println!("hello: {}", n);
        }
    }

    // ラベル付き。
    'outer: loop {
        println!("outer loop");

        #[allow(unused_labels)]
        'inner: loop {
            println!("inner loop");
            break 'outer;
        }

        // 以下は永遠に実行されない。
    }

    println!("Exited");
    // }}}

    // while {{{
    let mut m = 0;

    println!("while");

    while m < end{
        println!("{}", m);
        m += 1;
    }
    // }}}

    // for .. in {{{
    println!("for..in");

    println!("0 to 9");
    for a in 0..10{
        println!("{}", a);
    }

    println!("0 to 10");
    for a in 0..=10{
        println!("{}", a);
    }
    // }}}

    // for iter{{{
    let names = vec!["hello", "world", "jess"];
    for name in names.iter(){
        println!("{}", name);
    }
    // }}}
    // for into_iter{{{
    // into_iter は所有権を奪い、イテレータを作成する？
    for name in names.into_iter(){
        println!("{}", name);
    }
    // println!("{:?}", names);
    // }}}
    // for iter_mut {{{
    // 可変の iter. map() みたいなもの？
    let mut member_names = vec!["June", "Helen", "Alan", "Clay"];

    for name in member_names.iter_mut() {
        *name = match name {
            &mut "Alan" => "He is Alan.",
            _ => "Hello",
        }
    }
    println!("names: {:?}", member_names);
    // }}}
}
// }}}
