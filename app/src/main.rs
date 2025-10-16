use std::{thread, time::Duration};

fn main(){
    let mut thread_handles = Vec::new();
    for n in 1..=100{
        thread_handles.push(thread::spawn(move || foo(n)));
    }

    for handle in thread_handles{
        handle.join().unwrap();
    }
}
 
fn foo(n: u64){
    println!("start {n}");
    thread::sleep(Duration::from_secs(1));
    print!("end {n}, ");
}