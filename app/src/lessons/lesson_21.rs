use std::{fs::{File, OpenOptions}, io::{stdin, Read, Write}};

/*
1 Методы открытия и создания файлов
2 запись данных в файл
3 чтение данных из файла
4 режим read-write при открытии файла
5 переименование файла
6 удаление файла
*/
pub fn lesson21(){
    //1 Методы открытия и создания файлов
    //read
    //write
    File::create("text.txt").expect("Error creating file!");
    //create создает пустой файл, если файл был создан ранее - данные стираются
    //create write-only

    //File::open("text.txt").expect("Error opening file!");
    //open передает ошибки, если файл не существует


    //2 запись данных в файл
    // let path = "data.txt";

    // let mut f = File::create(path).expect("Error crearing file");

    // f.write_all(b"Hello from fogerion").expect("Error writing to file");

    
    //3 чтение данных из файла
    // let path = "data.txt";

    // let mut f = File::open(path).expect("Error crearing file");

    // let mut file_data = String::new();

    // f.read_to_string(&mut file_data).expect("Error reading file!");

    // println!("{}", file_data);


    //4 режим read-write при открытии файла
    // let path = "data.txt";

    // let mut f = OpenOptions::new()
    //     .read(true)
    //     .write(true)
    //     .create(true)
    //     .open(path)
    //     .expect("Error opening creating file");

    // let mut file_data = String::new();
    // f.read_to_string(&mut file_data).expect("Error reading file");
    // println!("File data: \n{}",file_data);
    
    // println!("Enter something: ");

    // let mut input = String::new();
    // stdin().read_line(&mut input).expect("Error getting user input");

    // f.write_all(input.as_bytes()).expect("Error writing to file");


    //5переименование файла

    // let path = "data.txt";
    // std::fs::rename(path, "text.txt").expect("Error!");

    //6 удаление файла
    // let path = "text.txt";
    // std::fs::remove_file(path).expect("Error!");

}