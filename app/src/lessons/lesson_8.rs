//lesson 8 rust
//match
pub fn lesson8(){
    // let num = 43;
    
    // match num{
    //     10 => println!("Num is 10"),
    //     45 =>{
    //         println!("num is 45");
    //         println!("ur old man");
    //     },
    //     10..=50 => {
    //         println!("число 10< <50")
    //     },
    //     _ => {
    //         println!("wtf");
    //     }
    // }

    // let numx = 8;
    // let num = match numx {
    //     2=>1,
    //     3=>10,
    //     8=>8*3,
    //     _=>888
    // };

    // println!("{}", num);

    let is_old: i8 = 18;
    let mut ready_num: String = String::new();

    match is_old{
        0..=18=>ready_num = String::from("Заходи"),
        _=> ready_num = String::from("get away")
    }
    println!("{}", ready_num);
}