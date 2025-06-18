//lesson 3 rust
// fn main() {
//     let mut num: f64 = 21.321321;
//     let mut age: i64 = 21;
//     //i64 it is integer and f64 it is float
//     num = 25.123213;
//     println!("Number is {}", num);
// }

// i8 i16 i32 i64 i128 isize
// u8 u16 u32 u64 u128 usize - its plus integers not minus

// fn main(){
//     //let mut name = "Ivan";
//     //let mut name: &str = "Ivan";
//     let mut name = String::from("Denis");
//     println!("Your name is {}", name);
// }


// lesson 4 rust

// fn main(){
//     //char
//     //bool

//     // let symbol: char = 'q';
//     // println!("{}",symbol);
//     //использование чар

//     // let logic: bool = true;
//     // let logic_1: bool = false;

//     // println!("{}, {}", logic, logic_1);
// }


//lesson 5 rust

// fn main(){
//     //однострочный комментарий

//     /*
//     многострочный
//     комментарий
//     база
//     */
//     //println!("Hello rust");
// }


//lesson 6 rust
//условнеы операторы

// fn main(){
//     //let age: i8 = 10;

//     // >, <, >=, <=, ==, !=
//     // if age >= 18 {
//     //     println!("Заходи бро");
//     // }
//     // else{
//     //     println!("Вон отсюда, малыш");
//     // }
//     //let num = 33;
//     // if num > 10 {
//     //     println!("num>10");
//     // }
//     // else if num < 55{
//     //     println!("num<55");
//     // }

//     // || &&
//     // if num > 10 && num < 50 {
//     //     println!("cool");
//     // }
//     // let name: String = String::from("Kate");
//     // if name != "Jane" {
//     //     println!("Good bye");
//     // }

//     // let is_true: bool = true;

//     // let num = if is_true {
//     //     1
//     // }
//     // else {
//     //     0
//     // };
//     // println!("{}", num);
// }



//lesson 7 rust
//циклы

// fn main(){
//     //loop
//     /*
//     let mut num: i16 = 0;
//     loop {
//         println!("{}", num);
//         num += 1;
//         if num == 100 {
//             break;
//         }
//     }
//     */

//     //while
//     /*
//     let mut num = 0;
//     while num <=100 {
//         if num % 2 ==0{
//             println!("{}", num);
//         }
//         num+=1;
//     }
//     */

//     //for
//     for i in 0..101 { //>=0 and <101
//         //println!("{}",i);
//         if i % 2 ==0{
//             println!("{}", i);
//         }
//     }
// }

//lesson 8 rust
//match
// fn main(){
//     // let num = 43;
    
//     // match num{
//     //     10 => println!("Num is 10"),
//     //     45 =>{
//     //         println!("num is 45");
//     //         println!("ur old man");
//     //     },
//     //     10..=50 => {
//     //         println!("число 10< <50")
//     //     },
//     //     _ => {
//     //         println!("wtf");
//     //     }
//     // }

//     // let numx = 8;
//     // let num = match numx {
//     //     2=>1,
//     //     3=>10,
//     //     8=>8*3,
//     //     _=>888
//     // };

//     // println!("{}", num);

//     let is_old: i8 = 18;
//     let mut ready_num: String = String::new();

//     match is_old{
//         0..=18=>ready_num = String::from("Заходи"),
//         _=> ready_num = String::from("get away")
//     }
//     println!("{}", ready_num);
// }