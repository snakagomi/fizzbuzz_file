use std::fs::File;
use std::io::{BufWriter, Write};

fn fizzbuzz(i: i32) -> String {
    match (i % 3, i % 5) {
        (0, 0) => "FizzBuzz".to_string(),
        (0, _) => "Fizz".to_string(),
        (_, 0) => "Buzz".to_string(),
        _ => i.to_string(),
    }
}

fn main() {
    let filename = "fizzbuzz_file_results.txt";

    {
        let fp = File::create(filename).unwrap();
        let mut writer = BufWriter::new(fp);
        for i in 1..=100 {
            let line = format!("{}\n", fizzbuzz(i));
            let bytes = line.as_bytes();
            writer.write(bytes).unwrap();
        }
    }

    let s = std::fs::read_to_string(filename).unwrap();
    println!("{}", s);
}
