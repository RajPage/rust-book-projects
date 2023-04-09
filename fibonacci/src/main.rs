use std::io;

fn main() {
    println!("Enter a positive integer below 20 and I will give you the fibonacci numbers");
    let mut limit = String::new();
    io::stdin()
        .read_line(&mut limit)
        .expect("Failed to read line");
    let limit: u8 = limit.trim().parse().expect("Enter a positive integer");
    if limit > 20 {
        println!("The entered number is greater than 20");
        return;
    }

    let mut fn_old = 1;
    let mut fn_curr = 1;

    print!("{fn_old} ");
    if limit > 1 {
        print!("{fn_curr} ");
    }
    for _i in 2..limit {
        let temp = fn_curr;
        fn_curr = fn_old + fn_curr;
        fn_old = temp;
        print!("{fn_curr} ");
    }
}
