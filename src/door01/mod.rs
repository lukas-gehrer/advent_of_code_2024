pub mod solution {
    use std::env;
    use crate::read_lines::read_lines::read_lines;

    pub fn part_one() {
        let path = env::current_dir().unwrap();
        let file01 = path.join("./src/door01/input.txt");

        let mut current_sum: i32 = 0;
        let mut  left = Vec::new();
        let mut  right = Vec::new();

        if let Ok(lines) = read_lines(file01) {
            for line in lines {
                let line = line.unwrap();
                let mut left_right = line.split_whitespace();
                left.push(left_right.next().unwrap().to_string());
                right.push(left_right.next().unwrap().to_string());
            }
        }

        left.sort();
        right.sort();

        for i in 0..left.len() {
            let left_int = left.get(i).unwrap().parse::<i32>().unwrap();
            let right_int = right.get(i).unwrap().parse::<i32>().unwrap();
            current_sum = current_sum + (left_int - right_int).abs();
        }

        println!("Door 01 part one result {}", current_sum);
    }

    pub fn part_two() {
        let path = env::current_dir().unwrap();
        let file01 = path.join("./src/door01/input.txt");

        let mut current_sum: i32 = 0;
        let mut  left = Vec::new();
        let mut  right = Vec::new();

        if let Ok(lines) = read_lines(file01) {
            for line in lines {
                let line = line.unwrap();
                let mut left_right = line.split_whitespace();
                left.push(left_right.next().unwrap().to_string());
                right.push(left_right.next().unwrap().to_string());
            }
        }

        for left_item in left {
            let mut multiplier = 0;
            for right_item in right.to_owned() {
                if left_item == right_item {
                    multiplier = multiplier + 1;
                }
            }
            let left_int = left_item.parse::<i32>().unwrap();
            current_sum = current_sum + (left_int * multiplier);
        }

        println!("Door 01 part two result {}", current_sum);
    }
}
