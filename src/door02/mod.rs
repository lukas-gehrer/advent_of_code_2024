pub mod solution {
    use crate::read_lines::read_lines::read_lines;
    use colored::Colorize;
    use std::env;

    pub fn part_one() {
        let path = env::current_dir().unwrap();
        let file01 = path.join("./src/door02/input.txt");

        let mut current_sum: i32 = 0;

        if let Ok(lines) = read_lines(file01) {
            for line in lines {
                let line = line.unwrap();
                let mut numbers = line.split_whitespace();
                let mut number_memory = 0;
                let mut overall_increasing = false;
                let mut save = true;

                for (i, number) in numbers.enumerate() {
                    let current_number = number.parse::<i32>().unwrap();
                    if current_number == number_memory {
                        save = false;
                        break;
                    }
                    let current_increasing = current_number > number_memory;
                    let difference = (number_memory - current_number).abs();

                    // first round
                    if number_memory == 0 {
                        number_memory = current_number;
                        overall_increasing = current_increasing;
                        continue;
                    }

                    if (i > 1 && current_increasing != overall_increasing) || difference > 3 {
                        save = false;
                        break;
                    }

                    overall_increasing = current_increasing;
                    number_memory = current_number;
                }

                if save {
                    println!("{}", line);
                    current_sum = current_sum + 1;
                }
            }
        }

        println!("Door 02 part one result {}", current_sum);
    }

    pub fn part_two() {
        let path = env::current_dir().unwrap();
        let file01 = path.join("./src/door02/input.txt");

        let mut current_sum: i32 = 0;

        if let Ok(lines) = read_lines(file01) {
            for line in lines {

                let line = line.unwrap();
                let mut numbers: Vec<&str> = line.split_whitespace().collect();
                let numbers_reversed = {
                    let mut clone = numbers.clone();
                    clone.reverse();
                    clone
                };

                if check_numbers_with_joker(numbers) || check_numbers_with_joker(numbers_reversed) {
                    current_sum = current_sum + 1;
                }
            }
        }
        println!("Door 02 part two result {}", current_sum);
    }

    fn check_numbers_with_joker(numbers: Vec<&str>) -> bool {
        let mut joker_used = false;
        let mut number_memory = 0;
        let mut overall_increasing = false;

        for (i, number) in numbers.iter().enumerate() {
            let current_number = number.parse::<i32>().unwrap();
            // check for equal rule
            if current_number == number_memory {
                if !joker_used {
                    joker_used = true;
                    continue;
                }
                return false;
            }
            let current_increasing = current_number > number_memory;
            let difference = (number_memory - current_number).abs();

            // first round
            if number_memory == 0 {
                number_memory = current_number;
                overall_increasing = current_increasing;
                continue;
            }

            // check for bigger one and smaller three rule
            if (i > 1 && current_increasing != overall_increasing) || difference > 3 {
                if !joker_used {
                    joker_used = true;
                    continue;
                }
                return false;
            }

            overall_increasing = current_increasing;
            number_memory = current_number;
        }

        true
    }
}