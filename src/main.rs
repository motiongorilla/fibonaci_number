use std::io;

fn main() {
    let mut current_number = 0;
    let mut previous_number = 0;
    let mut index = 0;

    loop {
        let mut target = String::new();
        println!("Type in desired index of Fibonaci sequence");

        io::stdin()
            .read_line(&mut target)
            .expect("Failed to read line");
        let target: i32 = match target.trim().parse() {
            Ok(value) => value,
            Err(_) => {
                println!("Incorrect value!");
                continue;
            }
        };

        while index < target + 1 {
            if index == 1 {
                previous_number = 0;
                current_number = 1;
                index += 1;
                continue;
            }
            let next_number = current_number + previous_number;
            previous_number = current_number;
            current_number = next_number;

            index += 1;
        }

        println!("Fibonaci number #{} is {current_number}", index-1);
        break;
    }
}
