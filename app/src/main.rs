// mod lessons{
//     pub mod lesson_3;
// }
mod lessons{
    pub mod lesson_10;
}


// fn main(){
//     // let str1 = String::from("TEXT");
//     // let str2 = str1.clone();
//     // println!("{}, {}", str1, str2);

//     // // стеки и куча 
//     // // переменные из стека могут спокойно клонироваться
//     // // стек переменные: интовые, чар, флатовые и кортежи с этими переменными

//     // // String и т.д. кучи


//     // let str = String::from("TEXT");


//     //текст перешел во владение в функции
//     // let str = String::from("TEXT");

//     // print_value(str);

//     // println!("{}", str);
//     let num = 9;
//     print_number(num);
//     println!("{}", num);
// }

// fn print_value(s: String){
//     println!("{}", s);
// }

// fn print_number(n: i32){
//     println!("{}", n);
// }

// fn main(){
//     let s = give();

//     let s1 = String::from("TEXT1");
//     let s2 = take_and_give(s1);

//     println!("{}",s1);
// }

// fn give() -> String{
//     String::from("Hello")
// }

// fn take_and_give(s: String) -> String{
//     s
//  }

fn main(){
    let s1 = String::from("TEXT");

}

fn calc(s: String) -> usize {
    s.len()
}