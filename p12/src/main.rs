use std::fs::File;
use std::io::ErrorKind;

fn main () {
    let greeting_file_result = File::open("hello.txt");
    #[warn(unused_variables)]
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fs) => fs,
                Err(e) => panic!("Problem creating file {:?}", e),
            },
            other_error => {
                panic!("Problem opening file: {:?}", other_error);
            }
        }
    };
}
