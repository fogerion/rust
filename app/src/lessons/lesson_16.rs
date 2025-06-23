//lesson 16
//перечисления

enum Person{
    Adult,
    Underage
}

enum Say{
    HI(String),
    Bye(String),
    Gm(String),
    Gn(String)
}

enum MathOperations {
    Add(f64, f64),
    Sub(f64, f64),
    Mult(f64, f64),
    Div(f64, f64)
}
impl MathOperations{
    fn math_operations(&self)-> f64{
        match self{
            &MathOperations::Add(a, b) =>  a+b,
            &MathOperations::Sub(a, b) =>  a-b,
            &MathOperations::Mult(a, b) => a*b,
            &MathOperations::Div(a, b) => a/b,
        }
    }
}
pub fn lesson16(){
    
    let person = Person::Adult;

    //match
    // match person {
    //     Person::Adult => {
    //         println!("Тебе можно");
    //         println!("Ты уже вырос");
    //     }
    //     Person::Underage => {
    //         println!("Тебе нельзя");
    //         println!("Ты маленький");
    //     }
    // }

    // let say = Say::Bye("Hello".to_string());


    // match say{
    //     Say::HI(h) => println!("{}", h),
    //     Say::Bye(_) => {
    //         println!("Say something1")
    //     }
    //     _ => {
    //         println!("Say something")
    //     }

    // }
    let mo = MathOperations::Add(18.0, 9.0);
    // match mo{
    //     MathOperations::Add(a, b) => println!("{}", a+b),
    //     MathOperations::Sub(a, b) => println!("{}", a-b),
    //     MathOperations::Mult(a, b) => println!("{}", a*b),
    //     MathOperations::Div(a, b) => println!("{}", a/b),
    // }

    let result = mo.math_operations();

    println!("{}",result)
}
