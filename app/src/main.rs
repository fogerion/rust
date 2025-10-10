use csv::{Reader, StringRecord};
use serde_json::{Value, Map};
use std::error::Error;
use std::{fs::{File, OpenOptions}, io::{stdin, Read, Write}};

fn main(){
    // Создаем тестовый CSV
    let csv_data = r#"name,age,salary,active
"John Doe",30,50000.50,true
"Jane Smith",25,45000.75,false
"Bob Johnson",35,,true
"Alice Brown",28,48000.0,true"#;

    let path = "file.csv";
    let mut f = File::create(path).expect("Error crearing file");
    
    f.write_all(csv_data.as_bytes()).expect("Error writing to file");
    // Конвертируем
    let text = csv_to_json("file.csv").expect("Error converting");

    let mut json_file = File::create("file.json").expect("Error crearing file");
    json_file.write_all(text.as_bytes()).expect("error writing json file");

}

fn csv_to_json(file_path: &str) -> Result<String, Box<dyn Error>>{
    let mut rdr = csv::Reader::from_path(file_path)?;
    
    let mut headers = rdr.headers()?.clone();
    
    let mut all = Vec::new();
    for record in rdr.records(){
        let row = record?;
        let mut json_vector = Vec::new();
        for i in 0..headers.len(){
            let json_row = format!(" {}: {} ", &headers[i], &row[i]);
            json_vector.push(json_row);
        }
        //let mut json_obj = format!("{{{:?}}}", json_vector);
        //println!("information: {:?}", json_obj);
        all.push(json_vector);
    }
    
    println!("{:?}", all);
    //println!("headers is :{}", &headers.len());
    let result = format!("{:?}", all);
    Ok(result)

}


    // let mut file_data = String::new();
    // f.read_to_string(&mut file_data).expect("Error reading file");