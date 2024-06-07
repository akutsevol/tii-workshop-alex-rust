
//use std::env;

mod bmod;

pub fn foo(mut a: i32, lim: i32) {
    while a < lim {
        a += 1;
        if a % 2 != 0 {
            continue;
        }
        println!("0x{a:04X?}");   // ? - for debugging
    }
}

fn main() {
    //env::set_var("RUST_BACKTRACE", "1"); // use for backtrace!

    println!("Hello, world!");
    foo(1, 10);
    print_num("8");
    print_num("n");
    println!("{}", bmod::add1(5));
}

pub fn bar(a: u64) -> u64 { a as u64 }

fn print_num(string_number: &str) {
    match string_number.parse::<i32>() {
        Ok(number)  => println!("Ваше число – {}", number),
        Err(e) => println!("Ошибка: {}", e)
    }
    bar(5);
    //string_number.parse::<i32>().unwrap();
}
