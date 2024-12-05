pub mod solution {
    use std::env;
    use regex::{ Regex};
    use std::path::PathBuf;
    use crate::read_lines::read_lines::read_lines;

    pub fn part_one() {
        let path = env::current_dir().unwrap();
        let input = path.join("./src/door03/input.txt");
        let example_input = path.join("./src/door03/example_input.txt");
        assert_eq!(161, process_file(example_input));
        process_file(input);
    }

    pub fn part_two() {
        let path = env::current_dir().unwrap();
        let input = path.join("./src/door03/input.txt");
        let example_input = path.join("./src/door03/example_input.txt");
        assert_eq!(48, process_file_part_2(example_input));
        process_file_part_2(input);
    }

    fn process_file(file: PathBuf) -> i32 {
        let mut current_sum: i32 = 0;

        if let Ok(lines) = read_lines(file) {
            for line in lines {
                let line = line.unwrap();
                let re: Regex = Regex::new(r"mul\(\d*,\d*\)").unwrap();
                let digit_re: Regex = Regex::new(r"(\d+)").unwrap();

                let results: Vec<&str> = re.find_iter(line.as_str()).map(|m| m.as_str()).collect();

                for result in results {
                    let numbers: Vec<&str> = digit_re.find_iter(result).map(
                        |m| m.as_str()
                    ).collect();
                    let mut numbers = numbers.iter();
                    let first_number = numbers.next().unwrap().parse::<i32>().unwrap();
                    let second_number = numbers.next().unwrap().parse::<i32>().unwrap();
                    current_sum = current_sum + (first_number * second_number);
                }
            }

        }

        println!("Door 03 part one result {}", current_sum);
        current_sum
    }

    fn process_file_part_2(file: PathBuf) -> i32 {
        let mut current_sum: i32 = 0;

        if let Ok(lines) = read_lines(file) {
            for line in lines {
                let line = line.unwrap();
                // let re: Regex = Regex::new(r"don't\(\)|do\(\)").unwrap();

                let re: Regex = Regex::new(r"(don't\(\))|(do\(\))|(mul\((\d+),(\d+)\))").unwrap();
                let digit_re: Regex = Regex::new(r"(\d+)").unwrap();

                let results: Vec<&str> = re.find_iter(line.as_str()).map(|m| m.as_str()).collect();

                let mut skip = false;

                for result in results {
                    if result == "do()" {
                        skip = false;
                        continue;
                    }
                    if result == "don't()" {
                        skip = true;
                        continue;
                    } else if skip {
                        continue;
                    }
                    let numbers: Vec<&str> = digit_re.find_iter(result).map(
                        |m| m.as_str()
                    ).collect();
                    let mut numbers = numbers.iter();
                    let first_number = numbers.next().unwrap().parse::<i32>().unwrap();
                    let second_number = numbers.next().unwrap().parse::<i32>().unwrap();

                    current_sum = current_sum + (first_number * second_number);
                }
            }

        }

        println!("Door 03 part two result {}", current_sum);
        current_sum
    }
}