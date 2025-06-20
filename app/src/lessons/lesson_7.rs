//lesson 7 rust
//циклы

pub fn lesson7(){
    //loop
    /*
    let mut num: i16 = 0;
    loop {
        println!("{}", num);
        num += 1;
        if num == 100 {
            break;
        }
    }
    */

    //while
    /*
    let mut num = 0;
    while num <=100 {
        if num % 2 ==0{
            println!("{}", num);
        }
        num+=1;
    }
    */

    //for
    for i in 0..101 { //>=0 and <101
        //println!("{}",i);
        if i % 2 ==0{
            println!("{}", i);
        }
    }
}
