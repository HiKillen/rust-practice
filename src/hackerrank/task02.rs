use std::io;

fn main() {
    let mut count = String::new();
    io::stdin().read_line(&mut count).unwrap();

    let mut num = String::new();
    io::stdin().read_line(&mut num).unwrap();
    let v: Vec<&str> = num.trim().split(' ').collect();
    
    let mut total: i32 = 0;
    for num in &v {
        let num: i32 = num.parse().unwrap();    
        total += num;
    }
     
    println!("{}", total);
}
