#[derive(Debug)]

// struct Person{
//     name: String,
//     surname: String,
//     age: i32,
//     balance: f64
// }



// // Структуры
// fn main(){
//     // let mut person1 = Person{
//     //     name: String::from("Kate"),
//     //     surname: String::from("Sofronova"),
//     //     age: 25,
//     //     balance: 99999.321
//     // };

//     //person1.name = String::from("Evgeny");

//     //println!("{:#?}", person1);
//     // println!(
//     //     "Person name is {} age is {}",
//     //     person1.name,
//     //     person1.age
//     // );


//     //вместо surname: surname можно surname просто
//     // let name = String::from("John");
//     // let surname = String::from("Smith");

//     // let person2 = Person{
//     //     name: name,
//     //     surname,
//     //     age: 0,
//     //     balance: 0.0
//     // };


//     let age = 34;
//     let balance = 4135.1231;

//     let person3 = Person{
//         name: "Andrei".to_string(),
//         surname: "Ivanov".to_string(),
//         age,
//         balance
//     };

//     let person4 = Person{
//         name: "Ivan".to_string(),
//         surname: "Ivanov".to_string(),
//         ..person3
//     };

//     println!("{:#?}", person4);
// }

// struct Str(i32, String, f64);

// fn main(){
//     let object = Str(5, "text".to_string(), 1231.23);

//     println!("{:?}", object);
// }


struct Triangle{
    cat1: f64,
    cat2: f64
}

impl Triangle{
    fn find_hyp(&self) -> f64{
        (self.cat1*self.cat1+self.cat2*self.cat2).sqrt()
    }

    fn find_s(&self) -> f64{
        self.cat1*self.cat2*(1.0/2.0)
    }

    fn is_eq(&self, ar: f64) -> bool {
        self.find_s() < ar
    }

    fn create_isc(cat: f64) -> Triangle{
        Triangle {
            cat1: cat, 
            cat2: cat 
        }
    }
}
pub fn lesson15(){
    /* 
    let tr1 = Triangle{
        cat1: 8.0,
        cat2: 10.0
    };

    //let hyp = find_hyp(tr1.cat1, tr1.cat2);
    let hyp = tr1.find_hyp();
    println!("{}", hyp);
    let s = tr1.find_s();
    println!("{}", s);
    */

    /* 
    let isc_tr = Triangle::create_isc(5.0);

    let hyp = isc_tr.find_hyp();
    let area = isc_tr.find_s();

    println!("{:#?}", isc_tr);
    println!("{}, {}", hyp, area);
    */

    let tr1 = Triangle{
        cat1: 6.0,
        cat2: 8.0
    };

    let tr2 = Triangle{
        cat1: 32.0,
        cat2: 43.0
    };

    let res = tr2.is_eq(tr1.find_s());

    if res{
        println!("tr2 can input into tr1");
    }
    else {
        println!("Oops");
    }

}

// fn find_hyp(c1: f64, c2: f64) -> f64{
//     (c1*c1+c2*c2).sqrt()