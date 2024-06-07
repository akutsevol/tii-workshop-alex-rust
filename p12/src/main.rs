use std::time::Duration;

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
}
