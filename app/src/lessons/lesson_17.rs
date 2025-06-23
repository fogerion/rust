
/* 
fn main(){
    //vectors
    //вектор хранит только один тип данных
    //let mut list = vec![1,2,3];
    //let mut list: Vec<f64> = Vec::new();
    // list.push(1.123);
    // list.push(213.123);
    // list.push(32.4443);

    let list = vec![1,2,3,4,5,6,7];
    
    //срез
    //println!("{:?}", &list[1..5]);

    //Option: Some, None
    // если элемента нету нон если есть сом
    // match list.get(100){
    //     Some(el) => {
    //         println!("Element with index 6 is {}", el);
    //     },
    //     None => {
    //         println!("element doesnt exist");
    //     },
    // }

    //Два метода перебора вектора 
    // for el in &list{
    //     println!("{}", el)
    // }

    // for el in list.iter(){
    //     println!("{}", el);
    // }

    println!("{:?}", list);
    //list.remove(2);  -- удаление элемента

    //println!("{:?}", &list.get(6));

    let list = vec![10,20,30, 80];


    println!("{}", find_avg(&list));
}*/
// fn find_avg(l: &Vec<i32>) -> f64{
//     let mut sum = 0;

//     for el in l{
//         sum += el;
//     }

//     let lenth = (l.len()) as i32;

//     ((sum / lenth)) as f64
// }



//Как хранить разные типы в векторах
// и работать с ними
#[derive(Debug)]
enum Types{
    Int(i32),
    Float(f64),
    Bool(bool),
    Text(String)
}

pub fn lesson17(){
    let list = vec![
        Types::Int(7),
        Types::Float(7.7),
        Types::Bool(true),
        Types::Text("seven".to_string())
    ];
    println!("{:?}", &list);

    match &list[3]{
        Types::Int(i) => {
            println!("Int is {}", i);
        },
        Types::Float(f) => {
            println!("Float is {}", f);
        },
        Types::Bool(b) => {
            println!("Bool is {}", b);
        },
        Types::Text(text) => {
            println!("Text is {}", text);
        },
    }
}