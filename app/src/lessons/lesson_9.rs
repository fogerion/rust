

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

use std::io;

pub fn lesson9(){
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



