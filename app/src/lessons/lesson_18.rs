
pub fn lesson18(){
    //lesson18();
    //Строки 


    //Создание строк

    let mut s: String = String::new(); //пустая строка(объявление)
    let s1: String = String::from("String2");
    let s2: String = "String 3".to_string();
    //без разницы

    //обновление строки
    
    s.push_str("Hello");//добавление строки
    s.push('W'); //добавляет один элемент в конец

    //конкатенация строк

    //let res = s1+&s2+"string4"; //первая строка должна быть владением, то есть без амперсанта

    let res: String = format!("{} {} TEXT {}",s,s1,s2);
    println!("{}", res);

    //индексирование строк
    //не поддерживает индексирование строк

    // let s3: String = "привет".to_string();
    // println!("{}", s3.len()); //кодирование кириллицы 2 байта
    // println!("{}", &s3[..2]); //первая буква &s3[..2])


    let s4: String = "йопи".to_string();

    // for ch in s4.chars(){
    //     println!("{}", ch);
    // }

    for ch in s4.bytes(){
        println!("{}", ch);
    }
}
