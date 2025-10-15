use csv;
use std::error::Error;
use std::{fs::File, io:: Write, io};

fn main(){
    //  цикл для бесконечной работы программы
    loop{
        //  интерфейс cli
        println!("Здравствуйте, вы открыли приложение парсер для парсинга .csv в .json");
        println!("Пожалуйства выберите действие:\n1)парсинг\n2)выход");
        
        //  выбор действия, 
        //  создается переменная
        let mut input = String::new();

        //ввод данных для переменной
        io::stdin().read_line(&mut input).expect("Error input_1");
        
        //  условия действия программы при выборе
        match input.trim(){
            //  действие когда выбрано 1)
            "1" => {
                println!("Вы выбрали парсинг, а теперь выберите следующее действие\n1)Указать путь к файлу .csv\n2)Сделать шаблон парсинга в этой папке\n3)выход");
                
                //  дополнительный выбор для парсинга
                //  переменная ввода
                let mut input_2 = String::new();
                
                //  ввод переменной
                io::stdin().read_line(&mut input_2).expect("Error input_2");
                match input_2.trim(){
                    "1" => {
                        println!("А теперь введите путь к исполняемому файлу");
                        
                        //  создается переменная для ввода пути к файлу с расширением .csv
                        let mut path = String::new();
                        
                        //  ввод пути к файлу
                        io::stdin().read_line(&mut path).expect("Error typing path");
                        
                        //  для корректной работы пользуемся .trim()
                        let path = path.trim();

                        //  создаем переменную и запускаем функцию конвертации
                        let text = csv_to_json(&path).expect("Error converting");

                        // создаем .json файл и записываем туда наши данные
                        let mut json_file = File::create("file.json").expect("Error crearing file");
                        json_file.write_all(text.as_bytes()).expect("error writing json file");
                        println!("Успешно. \n\n");
                        break;
                    }
                    "2" => {
                        println!("doing with according to the program template ");
                        //  создаем данные для .csv файл шаблона
                        let csv_data = r#"name,age,salary,active
"John Doe",30,50000.50,true
"Jane Smith",25,45000.75,false
"Bob Johnson",35,,true
"Alice Brown",28,48000.0,true"#;
                        
                        // путь к файлу пусть будет просто file.csv создадим файл тамже где запускается программа
                        let path = "file.csv";
                        let mut f = File::create(path).expect("Error crearing file");
                        
                        // записываем шаблонные данные в .csv файл
                        f.write_all(csv_data.as_bytes()).expect("Error writing to file");
                        
                        // Конвертируем нашим методом
                        let text = csv_to_json("file.csv").expect("Error converting");

                        // создаем .json файл и записываем туда наши данные
                        let mut json_file = File::create("file.json").expect("Error crearing file");
                        json_file.write_all(text.as_bytes()).expect("error writing json file");
                        println!("Успешно. \n\n");
                        break;
                    }
                    "3" => {
                        println!("Подождите, идет закрытие программы");
                        break;
                    }
                    _ => println!("Вы ввели неправильно, пожалуйста, введите только цифру без пробелов.\n\n")
                }
            }
            "2" => {
                break;
            }
            _=>{
                println!("Неверный выбор, Пожалуйста, введите 1 или 2 без пробелов.\n\n");
                continue;
            }
        }
    }    
}


//  метод конвертации .csv в .json 
fn csv_to_json(file_path: &str) -> Result<String, Box<dyn Error>>{

    //  считаем .csv файл методом для чтения именно таких файлов
    let mut rdr = csv::Reader::from_path(file_path)?;
    
    //  первую строку считаем за ключами
    let headers = rdr.headers()?.clone();
    
    //  создаем пустой вектор для записи данных
    let mut all = Vec::new();

    //  смотрим каждую строку файла
    for record in rdr.records(){
        //  записываем данные строки в переменную row
        let row = record?;

        //  строка записи ключ-значение
        let mut json_vector = Vec::new();

        //  записываем ключ-значение для одной строки
        for i in 0..headers.len(){
            let json_row = format!("{}: {}", &headers[i].trim(), &row[i].trim());

            //  пушим ключ-значение в вектор
            json_vector.push(json_row);
        }
        
        //  пушим строку в вектор all
        all.push(json_vector);
    }
    
    println!("{:?}", all);
    
    // записываем в формате стринг и ретурн 
    let result = format!("{:?}", all);
    Ok(result)
}

