use std::io;

use line_reader;

fn main() -> io::Result<()> {
    let numbers: Vec<u32> = line_reader::convert("input")?;

    for (i1, n1) in numbers.iter().enumerate() {
        for (i2, n2) in numbers.iter().enumerate().skip(i1 + 1) {
            if n1 + n2 == 2020 {
                println!("n1: {}", n1);
                println!("n2: {}", n2);
                println!("result: {}", n1 * n2);
            }
            for n3 in numbers.iter().skip(i1 + i2 + 1) {
                if n1 + n2 + n3 == 2020 {
                    println!("n1: {}", n1);
                    println!("n2: {}", n2);
                    println!("n3: {}", n3);
                    println!("result: {}", n1 * n2 * n3);
                }
            }
        }
    }

    Ok(())
}
