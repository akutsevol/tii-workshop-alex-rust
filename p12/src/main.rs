use std::time::Duration;

fn main () {
    let h = std::thread::spawn(|| {
        println!("Hi from thread!");
        std::thread::sleep(Duration::from_secs(3));
        42u32
    });
    let res = h.join().unwrap();
    println!("{:?}", res);
}
