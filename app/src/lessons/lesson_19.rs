use std::collections::HashMap;
pub fn lesson19(){
    //HASHMAP
    // кэш отображение
    // пример {"Я":12}
    

    //создание hashmap
    // let mut map = HashMap::new();

    // map.insert("Denis".to_string(), 10);
    // map.insert("Kate".to_string(), 11);
    // map.insert("Evgeny".to_string(), 8);

    // println!("{:?}", map);    //сохраняется в произвольном порядке



    // владение hashmap
    // let mut map = HashMap::new();

    // let n1 ="Denis".to_string();
    // let n2="Kate".to_string();
    // let n3 = "Evgeny".to_string();

    // map.insert(n1, 10);
    // map.insert(n2, 11);
    // map.insert(n3, 8);    
    // println!("{:?}", map); 
    //println!("{}",n1);    //n1,n2,n3 все пеместились в map



    //получение элементов hashmap
    // let mut map = HashMap::new();

    // map.insert("Denis".to_string(), 10);
    // map.insert("Kate".to_string(), 11);
    // map.insert("Evgeny".to_string(), 8);

    //1 способ
    //println!("{}", map["Denis"]); 

    //2 способ
    // match map.get("Kate"){
    //     Some(mark) => {
    //         println!("Mark is {}", mark);
    //     },
    //     None => {
    //         println!("Element doesnt exist");
    //     },
    // }



    // перебор hashmap циклом
    // let mut map = HashMap::new();

    // map.insert("Denis".to_string(), 10);
    // map.insert("Kate".to_string(), 11);
    // map.insert("Evgeny".to_string(), 8);

    // for (name, mark) in &map{
    //     println!("{} has {}!", name, mark);
    // }



    //изменение hashmap
    // let mut map = HashMap::new();

    // let n1 ="Denis".to_string();
    // let n2="Kate".to_string();
    // let n3 = "Evgeny".to_string();

    // map.insert(&n1, 10);
    // map.insert(&n2, 11);
    // map.insert(&n3, 8); 

    // let n4 = "Any".to_string();
    // map.entry(&n4).or_insert(9);    //entry проверяет есть ли индекс &n4, если нет то создается новый индекс &n4 со значением 9

    // println!("{:?}", map);



    let str =  "learn Rust with me me ME".to_lowercase();

    let mut count_map = HashMap::new();

    for w in str.split_whitespace(){
        let count = count_map.entry(w).or_insert(0);
        *count += 1;
    }

    println!("{:?}", count_map);
}