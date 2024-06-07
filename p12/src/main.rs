use std::time::Duration;
use std::fs::File;
use std::env;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use std::collections::HashMap;
use rayon::prelude::*;

fn computing_the_dot_product_of_two_large_vectors () {
    let vec1 = vec![1, 2, 3, 4, 5];
    let vec2 = vec![6, 7, 8, 9, 10];

    let dot_product: i32 = vec1.par_iter()
        .zip(vec2.par_iter())
        .map(|(&x, &y)| x * y)
        .sum();

    println!("{}", dot_product); // Prints 130
}

fn processing_large_files () {
    // Function to read a file into a Vec<String>
    fn read_lines<P>(filename: P) -> io::Result<Vec<String>>
    where P: AsRef<Path>, {
        let file = File::open(filename)?;
        let reader = BufReader::new(file);
        reader.lines().collect()
    }

    let path = env::current_dir().unwrap().into_os_string().into_string().unwrap();
    let log_file_name = format!("{path}/p12/src/{}", "large_log_file.log");
    let lines = read_lines(log_file_name).expect("Could not read file");

    let error_count: usize = lines.par_iter()
        .filter(|line| line.contains("LQM-WIFI"))
        .count();

    println!("Number of \"LQM-WIFI\" logs: {}", error_count);
}

fn test_scope() {
    fn factorial_sequential(n: u128) -> u128 { 
        (1..=n).reduce(|multiple, next| multiple * next).unwrap() 
    } 

    let mut map: Option<HashMap<String, usize>> = None; 
    let mut factorial = 1; 
    let mut other = None; 
    rayon::scope(|s| { 
        s.spawn(|_s| { 
            let iter = 
                (0..10000).enumerate().map(|(a, b)| (format!("index {}", a), b)); 
            map = Some(HashMap::from_iter(iter)); 
        }); 
        s.spawn(|_s| { 
            factorial = factorial_sequential(30); 
        }); 
        s.spawn(|_s| { 
            other = Some("value") 
        }) 
    }); 

    // println!("map {:?}", map.unwrap()); 
    println!("factorial {:?}", factorial); 
    // println!("other {:?}", other); 
} 

fn main () {
    let buf = vec!(1u32, 2, 3);

    let res = std::thread::scope(|s| {
        let h1 = s.spawn(|| {
            let a: u32  = buf.iter().sum();
            std::thread::sleep(Duration::from_secs(1));
            println!("Hi from thread {}!", a);
            2u32
        });
        let h2 = s.spawn(|| {
            let b: u32 = buf.iter().map(|&x| x * x).sum();
            std::thread::sleep(Duration::from_secs(3));
            println!("Hi from thread {}!", b);
            42u32
        });
        h1.join().unwrap() + h2.join().unwrap()
    });

    println!("{res}");

    computing_the_dot_product_of_two_large_vectors();
    processing_large_files();
    test_scope();
}
