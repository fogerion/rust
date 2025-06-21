//lesson 14
// ссылки, владения, срезы


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


// использование ссылок
// fn main(){
//     let s1 = String::from("TEXT");
//     println!("{}", calc(&s1));

//     println!("{}", s1);
// }

// fn calc(s: &String) -> usize {
//     s.len()
// }


// использование ссылок с изменениями переменной
// fn main(){
//     let mut s = String::from("Text");
//     println!("{}", s);
//     add(&mut s);
//     println!("{}", s);
// }

// fn add(s: &mut String){
//     s.push_str(" IS AWESOME")
// }
// fn main(){
//     let mut name = String::new();

//     std::io::stdin().read_line(&mut name).expect("Err");

//     println!("My name is {}", name);
// }


// ограничение на изменяемые ссылки, не больше одного на переменного
// fn main(){
//     let mut s = String::from("Text");

//     let refe = &mut s;
//     let refe1 = &mut s;

//     println!("{}, {}", refe, refe1);
// }

// из-за владения
// и области видимости
// можно создавать две изменяемые ссылки на переменную
// fn main(){
//     let mut s = String::from("Text");

//     let x = true;

//     if x == true {
//         let ref1=&mut s;
//         println!("{}", ref1);
//     }

//     let ref2 = &mut s;
//     println!("{}", ref2);
//     // а так субобласть владения надо стоят впереди
// }


// висячие ссылки
// если ретурнить ссылку на другое владение
// то ретурниться пустота, а в расте ошибка
// fn main(){
//     let n = 65;
//     let s = ret();
//     println!("{}", s);
// }

// fn ret() -> String{
//     let str = String::from("text");
//     str
// }

//срез
// fn main(){
//     let str = String::from("TEXT HERE");

//     let slice = &str[0..4];
//     // [..4] = [0..4]
//     // [0..8+1] or [..] all
//     println!("{}", slice);

//     let arr = [1,2,3,4,5];
//     let slice = &arr[1..3];

//     println!("{:?}", slice);
// }