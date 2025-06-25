use std::fs::File;
use std::io::{ErrorKind, Read};
use std::io::Error;

pub fn lesson20(){
    //обработка ошибок в программе

    //использование макрокоманды panic!()
    //panic!("Error");


    //ввыводится паника выхода за границы вектора
    // let list = vec![1,2,3];
    // list[4];
    
    //тип result
    // let f = File::open("lol.html");
    
    // let f = match f{
    //     Ok(file) => file,
    //     Err(e) => panic!("Error openin file {:?}", e),
    // };


    //обработка конкретных ошибок с помощью match
    // let f = File::open("lol.html");
    
    // let path = "lol.html";

    // let f = match f{
    //     Ok(file) => file,
    //     Err(e) => match e.kind(){
    //         ErrorKind::NotFound => {                 //--вот это вот
    //             match File::create(path){
    //                 Ok(file) => file,
    //                 Err(e) => panic!("Error creating file: {:?}", e)
    //             }
    //         },
    //         other => panic!("ERROR: {:?}", other)
    //     },
    // };


    //Методы unwrap() и expect()
    // let path = "lol.html";
    // let f = File::open("lol.html").unwrap();
    // Ok(file) => file,
    // Err(e) => panic!("...{:?}", e)
    
    // let f = File::open("lol.html").expect("Error1");
    // оба паникуют, но экспект выводит наше сообщение


    //РАспространение ошибок
    // let path = "file.txt";

    // match read_file_data(path){
    //     Ok(data) => println!("data is {}", data),
    //     Err(e) => match e.kind(){
    //         ErrorKind::NotFound => match File::create(path){
    //             Ok(_) => println!("FIle created"),
    //             Err(e) => panic!("creating error"),
    //         },
    //         other=> panic!("Error OCCURED: {:?}", other),
    //     }
    // }


    // оператор ? Используется только в result
    let path = "file.txt";

    match read_file_data(path){
        Ok(data) => println!("data is {}", data),
        Err(e) => match e.kind(){
            ErrorKind::NotFound => match File::create(path){
                Ok(_) => println!("FIle created"),
                Err(e) => panic!("creating error"),
            },
            other=> panic!("Error OCCURED: {:?}", other),
        }
    }
}

//для распространении ошибки
// fn read_file_data(path: &str) -> Result<String, Error>{
//     let f = File::open(path);

//     let mut f = match f {
//         Ok(file) => file,
//         Err(e) => return Err(e),
//     };

//     let mut data = String::new();

//     match f.read_to_string(&mut data){
//         Ok(_) => Ok(data),
//         Err(e) => Err(e),
//     }
// }


//для оператора ?
fn read_file_data(path: &str) -> Result<String, Error>{
    let mut f = File::open(path)?;
    // вместо 
    //     let f = File::open(path);

    //     let mut f = match f {
    //         Ok(file) => file,
    //         Err(e) => return Err(e),
    //     };

    let mut data = String::new();
    f.read_to_string(&mut data);
    Ok(data)
    // вместо
    // match f.read_to_string(&mut data){
    //     Ok(_) => Ok(data),
    //     Err(e) => Err(e),
    // }
}