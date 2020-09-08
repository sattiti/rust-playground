// 値を明示していない場合、0 から整数が順に入る。
#[allow(dead_code)]
enum MyColor {
    Red,
    Green,
    Blue
}

#[allow(dead_code)]
enum Status {
    Stop,
    Pause,
    Play,
    Rec
}

#[allow(dead_code)]
enum Color{
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}

// main {{{
fn main(){
    // use を使用すれば、絶対名で指定する必要がなくなる。
    use crate::Status::*;
    use crate::MyColor::{Red};

    println!("{}", Red as i32);

    let current_status = Play;
    match current_status {
        Play => println!("playing."),
        Pause => println!("paused."),
        Stop => println!("Stoped."),
        Rec => println!("Rec."),
    }

    println!("#{:06x}", Color::Red as i32);
    println!("#{:06x}", Color::Green as i32);
    println!("#{:06x}", Color::Blue as i32);
}
// }}}
