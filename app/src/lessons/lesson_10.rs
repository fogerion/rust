

//lesson 10
//Массивы

pub fn lesson10(){
    
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
}
