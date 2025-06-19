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

//lesson 9 
//enter

// use std::io;
// // io, i-input, o-output

// fn main(){
//     let mut name = String::new();

//     println!("Whats ur name?");
//     match io::stdin().read_line(&mut name){
//         Ok(_)=> {
//             println!("Hello, {}", name);
//         },
//         Err(e) => {
//             println!("Program error -{}",e);
//         }
//     }
// }


// проект квадратных уравнений
/*
use std::io;

fn main(){
    //ax^2+bx+c=0
    //D=b^2-4(ac)
    

    loop{
        let mut a_str = String::new();
        let mut b_str = String::new();
        let mut c_str = String::new(); 
        let mut quit_programm = String::new(); 
        println!("решить квадратное уравнение");
        println!("введите а");
        match io::stdin().read_line(&mut a_str){
            Ok(_) => {},
            Err(e) => {
                println!("Program error -{}",e);
            }
        }

        let mut a: f64 = 0.0;
        match a_str.trim().parse::<f64>() {  
            Ok(num) => a = num,  
            Err(e) => {
                println!("Ошибка ввода коэффициента 'a': {}", e);
                println!("'a': {}", a_str);
                break;
            },
        };  

        println!("введите b");
        match io::stdin().read_line(&mut b_str){
            Ok(_) => {},
            Err(e) => {
                println!("Program error -{}",e);
            }
        }
        let mut b: f64 = 0.0;
        match b_str.trim().parse::<f64>() {  
            Ok(num) => b = num,  
            Err(e) => {
                println!("Ошибка ввода коэффициента 'b': {}", e);
                println!("'b': {}", b_str);
                break;  
            },
        };  

        println!("введите c");
        match io::stdin().read_line(&mut c_str){
            Ok(_) => {},
            Err(e) => {
                println!("Program error -{}",e);
            }
        }
        let mut c: f64 = 0.0;
        match c_str.trim().parse::<f64>() {  
            Ok(num) => c = num,  
            Err(e) => {
                println!("Ошибка ввода коэффициента 'c': {}", e);
                println!("'c': {}", c_str);
                break;  
            },
        };  
      

        let d: f64 = b*b - 4.0*(a*c);

        if d>0.0 {
            let x1 = ((-b)+ d.sqrt())/(2.0*a);
            let x2 = ((-b)- d.sqrt())/(2.0*a);

            println!("x1={}, x2={}", x1, x2);
        }
        else if d==0.0{
            let x = -b/(2.0*a);

            println!("x={}",x);
        }
        else {
            println!("net kornei");
        }

        println!("if u want to quit enter quit:");
        match io::stdin().read_line(&mut quit_programm){
            Ok(_) => {},
            Err(e) => {
                println!("Program error -{}",e);
            }
        }

        
        if quit_programm.trim() == "quit"{
            break;
        }
    }

}

*/


//lesson 10
//Массивы
/*
fn main(){
    
    // let mut array:[i64; 4] = [1, 2, 3, 4];

    // println!("{:?}", array[1]);
    // array[1] = 5;
    
    // println!("{:?}", array[1]);

    // let arr = [2; 50];
    // println!("{:?}", arr);
    let array = [1,2,3,4,5,8,67,7];

    // for i in array.iter(){
    //     println!("{}",i);
    // }

    println!("{}", array.len());

    // for i in 0..array.len(){
    //     println!("{}", array[i]);
    // }

    // let mut i = 0;
    // while i < array.len(){
    //     println!("{}", array[i]);
    //     i+=1;
    // }

    for i in array.iter(){
        if i%2==0{
            println!("{}",i);
        }

    }


}*/


//lesson 11 rust
// const
// const C: i64 = 5;
// fn main(){
//     let c = 5;
//     println!("{}", C)
// }
// fn sum(a: i8, b: i8) -> i8{
//     a+b
// }
// fn main(){
//     let c = sum(2,3);
//     println!("{}", c);
// }



//lesson 12 rust
//кортежи
// fn main(){
//     // let tuple = (12,34.6, "Lol");
//     // println!("{:?}", tuple);

//     let pupil = (String::from("Denis"), 11);

    

//     let name_ = pupil.0;
//     let grade_ = pupil.1;
//     println!("{}", name_);
//     println!("{}", grade_);

//     // let (name, grade) = pupil;
//     // println!("{}", name);
//     // println!("{}", grade);
// }


//lesson 13 rust
//functions

fn main(){
    //info(String::from("asdas"),1231,2131.0);
    let (sum,min,mul) = math(12,12);
    println!("{} {} {}",sum,min,mul);
}

// fn sum(a: i32, b: i32){
//     println!("Sum is {}", a + b);
// }

// fn info(name: String, age: i32, wallet: f64){
//     println!("{}, {}, {}", name, age, wallet)
// }

// fn multi(a: i32, b: i32) -> i32 {
//     return a * b;
// }
fn math(a: i32, b: i32) -> (i32,i64,i32) {
    (a+b,(a-b).into(),a*b)
}