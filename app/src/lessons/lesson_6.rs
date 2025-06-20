
//lesson 6 rust
//условнеы операторы

pub fn lesson6(){
    //let age: i8 = 10;

    // >, <, >=, <=, ==, !=
    // if age >= 18 {
    //     println!("Заходи бро");
    // }
    // else{
    //     println!("Вон отсюда, малыш");
    // }
    //let num = 33;
    // if num > 10 {
    //     println!("num>10");
    // }
    // else if num < 55{
    //     println!("num<55");
    // }

    // || &&
    // if num > 10 && num < 50 {
    //     println!("cool");
    // }
    // let name: String = String::from("Kate");
    // if name != "Jane" {
    //     println!("Good bye");
    // }

    let is_true: bool = true;

    let num = if is_true {
        1
    }
    else {
        0
    };
    println!("{}", num);
}
