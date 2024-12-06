pub mod solution {
    use crate::read_lines::read_lines::read_lines;
    use std::env;
    use std::path::PathBuf;

    pub fn part_one() {
        let path = env::current_dir().unwrap();
        let test_input = path.join("./src/door04/test_input.txt");
        let example_input = path.join("./src/door04/example_input.txt");
        let up_test = path.join("./src/door04/up_test.txt");
        let left_test = path.join("./src/door04/left_test.txt");
        let right_test = path.join("./src/door04/right_test.txt");
        let down_test = path.join("./src/door04/down_test.txt");
        let input = path.join("./src/door04/input.txt");

        assert_eq!(1, process_file(up_test), "UP TEST");
        assert_eq!(1, process_file(left_test), "LEFT TEST");
        assert_eq!(1, process_file(right_test), "RIGHT TEST");
        assert_eq!(1, process_file(down_test), "DOWN TEST");

        assert_eq!(4, process_file(test_input));
        assert_eq!(18, process_file(example_input));
        process_file(input);
    }

    pub fn part_two() {
        let path = env::current_dir().unwrap();
        let example_input = path.join("./src/door04/example_input_2.txt");
        let input = path.join("./src/door04/input_2.txt");

        assert_eq!(9, process_file_part_2(example_input));
        process_file_part_2(input);
    }

    pub fn generate_grid(file: PathBuf) -> Vec<Vec<char>> {
        let mut grid = Vec::new();
        if let Ok(lines) = read_lines(file) {
            for line in lines {
                let grid_line = line.unwrap().chars().collect();
                grid.push(grid_line);
            }
        }
        grid
    }

    fn process_file_part_2(file: PathBuf) -> i32 {
        let mut count: i32 = 0;

        let mut grid = generate_grid(file);

        for (i, grid_line) in grid.iter().enumerate() {
            // not necessary to check edges
            if i == 0 || i == grid.len() - 1 {
                continue;
            }
            println!("{:?}", grid_line.to_owned().into_iter());
            for (ii, grid_char) in grid_line.iter().enumerate() {
                // not necessary to check edges
                if ii == 0 || ii == grid[0].len() - 1 {
                    continue;
                }
                if grid_char.to_ascii_uppercase() == 'A' {
                    println!(
                        "CHECK AROUND {} AT Y: {} X: {}",
                        grid_char.to_ascii_uppercase(),
                        i,
                        ii
                    );
                    count = count + get_x_mas_count_around_a(&grid.to_owned(), i, ii);
                }
            }
        }

        println!("Door 04 part one result {}", count);
        count
    }

    fn process_file(file: PathBuf) -> i32 {
        let mut count: i32 = 0;

        let mut grid = generate_grid(file);

        for (i, grid_line) in grid.iter().enumerate() {
            println!("{:?}", grid_line.to_owned().into_iter());
            for (ii, grid_char) in grid_line.iter().enumerate() {
                if grid_char.to_ascii_uppercase() == 'X' {
                    println!(
                        "CHECK AROUND {} AT Y: {} X: {}",
                        grid_char.to_ascii_uppercase(),
                        i,
                        ii
                    );
                    count = count + get_xmas_count_around_x(&grid.to_owned(), i, ii);
                }
            }
        }

        println!("Door 04 part one result {}", count);
        count
    }

    fn get_xmas_count_around_x(
        grid: &Vec<Vec<char>>,
        current_position_y: usize,
        current_position_x: usize,
    ) -> i32 {
        let mut xmas_count = 0;
        let mut xmas = ['X', 'M', 'A', 'S'];
        let directions = ["U", "UR", "UL", "R", "RD", "D", "LD", "L"];

        for direction in directions {
            let mut found_xmas = false;
            println!("CHECK DIRECTION {}", direction);
            if direction == "U" && xmas_fits_top(current_position_y as i32) {
                let y = current_position_y;
                let mut wrong = false;

                for (i, char) in xmas.into_iter().enumerate() {
                    let grid_index_y = y - i;

                    if char != grid[grid_index_y][current_position_x] {
                        println!(
                            "CHECKED FOR CHAR {} FOUND {}",
                            char, grid[grid_index_y][current_position_x]
                        );
                        wrong = true;
                        break;
                    } else {
                        println!("FOUND CHAR {}", char);
                    }
                }
                if !wrong {
                    found_xmas = true;
                }
            } else if direction == "UR"
                && xmas_fits_top(current_position_y as i32)
                && xmas_fits_right(current_position_x as i32, grid[0].len() as i32)
            {
                let x = current_position_x;
                let y = current_position_y;
                let mut wrong = false;

                for (i, char) in xmas.into_iter().enumerate() {
                    let grid_index_y = y - i;
                    let mut grid_index_x = x + i;

                    if char != grid[grid_index_y][grid_index_x] {
                        println!(
                            "CHECKED FOR CHAR {} FOUND {}",
                            char, grid[grid_index_y][grid_index_x]
                        );
                        wrong = true;
                        break;
                    } else {
                        println!("FOUND CHAR {}", char);
                    }
                }
                if !wrong {
                    found_xmas = true;
                }
            } else if direction == "UL"
                && xmas_fits_top(current_position_y as i32)
                && xmas_fits_left(current_position_x as i32)
            {
                let x = current_position_x;
                let y = current_position_y;
                let mut wrong = false;

                for (i, char) in xmas.into_iter().enumerate() {
                    let grid_index_y = y - i;
                    let grid_index_x = x - i;

                    if char != grid[grid_index_y][grid_index_x] {
                        println!(
                            "CHECKED FOR CHAR {} FOUND {}",
                            char, grid[grid_index_y][grid_index_x]
                        );
                        wrong = true;
                        break;
                    } else {
                        println!("FOUND CHAR {}", char);
                    }
                }
                if !wrong {
                    found_xmas = true;
                }
            } else if direction == "D"
                && xmas_fits_bottom(current_position_y as i32, grid.len() as i32)
            {
                let y = current_position_y;
                let mut wrong = false;

                for (i, char) in xmas.into_iter().enumerate() {
                    let grid_index_y = y + i;

                    if char != grid[grid_index_y][current_position_x] {
                        println!(
                            "CHECKED FOR CHAR {} FOUND {}",
                            char, grid[grid_index_y][current_position_x]
                        );
                        wrong = true;
                        break;
                    }
                }
                if !wrong {
                    found_xmas = true;
                }
            } else if direction == "RD"
                && xmas_fits_bottom(current_position_y as i32, grid.len() as i32)
                && xmas_fits_right(current_position_x as i32, grid[0].len() as i32)
            {
                let x = current_position_x;
                let y = current_position_y;
                let mut wrong = false;

                for (i, char) in xmas.into_iter().enumerate() {
                    let grid_index_y = y + i;
                    let grid_index_x = x + i;

                    if char != grid[grid_index_y][grid_index_x] {
                        println!(
                            "CHECKED FOR CHAR {} FOUND {}",
                            char, grid[grid_index_y][grid_index_x]
                        );
                        wrong = true;
                        break;
                    }
                }
                if !wrong {
                    found_xmas = true;
                }
            } else if direction == "LD"
                && xmas_fits_bottom(current_position_y as i32, grid.len() as i32)
                && xmas_fits_left(current_position_x as i32)
            {
                let x = current_position_x;
                let y = current_position_y;
                let mut wrong = false;

                for (i, char) in xmas.into_iter().enumerate() {
                    let grid_index_y = y + i;
                    let grid_index_x = x - i;

                    if char != grid[grid_index_y][grid_index_x] {
                        println!(
                            "CHECKED FOR CHAR {} FOUND {}",
                            char, grid[grid_index_y][grid_index_x]
                        );
                        wrong = true;
                        break;
                    }
                }
                if !wrong {
                    found_xmas = true;
                }
            } else if direction == "R"
                && xmas_fits_right(current_position_x as i32, grid[0].len() as i32)
            {
                let x = current_position_x;
                let mut wrong = false;

                for (i, char) in xmas.into_iter().enumerate() {
                    let mut grid_index_x = x + i;

                    if char != grid[current_position_y][grid_index_x] {
                        println!(
                            "CHECKED FOR CHAR {} FOUND {}",
                            char, grid[current_position_y][grid_index_x]
                        );
                        wrong = true;
                        break;
                    }
                }
                if !wrong {
                    found_xmas = true;
                }
            } else if direction == "L" && xmas_fits_left(current_position_x as i32) {
                let x = current_position_x;
                let mut wrong = false;

                for (i, char) in xmas.into_iter().enumerate() {
                    let mut grid_index_x = x - i;

                    if char != grid[current_position_y][grid_index_x] {
                        println!(
                            "CHECKED FOR CHAR {} FOUND {}",
                            char, grid[current_position_y][grid_index_x]
                        );
                        wrong = true;
                        break;
                    }
                }
                if !wrong {
                    found_xmas = true;
                }
            }
            if found_xmas {
                println!("ALL LETTERS MATCH XMAS IN DIRECTION {}", direction);
                xmas_count = xmas_count + 1;
            }
        }
        xmas_count
    }

    fn xmas_fits_left(x: i32) -> bool {
        x - 3 >= 0
    }

    fn xmas_fits_right(x: i32, grid_len: i32) -> bool {
        x + 3 <= grid_len - 1
    }

    fn xmas_fits_top(y: i32) -> bool {
        y - 3 >= 0
    }

    fn xmas_fits_bottom(y: i32, grid_len: i32) -> bool {
        y + 3 <= grid_len - 1
    }

    fn get_x_mas_count_around_a(
        grid: &Vec<Vec<char>>,
        current_position_y: usize,
        current_position_x: usize,
    ) -> i32 {
        let mut xmas_count = 0;
        let positions = ["MMSS", "MSMS", "SMSM", "SSMM"];

        for position in positions {
            println!("CHECK POSITION {:?}", position);

            let up_left_y = current_position_y - 1;
            let up_left_x = current_position_x - 1;

            let up_right_y = current_position_y - 1;
            let up_right_x = current_position_x + 1;

            let down_left_y = current_position_y + 1;
            let down_left_x = current_position_x - 1;

            let down_right_y = current_position_y + 1;
            let down_right_x = current_position_x + 1;

            let edges = format!(
                "{}{}{}{}",
                grid[up_left_y][up_left_x],
                grid[up_right_y][up_right_x],
                grid[down_left_y][down_left_x],
                grid[down_right_y][down_right_x]
            );

            if edges == position {
                println!("ALL LETTERS MATCH X-MAS IN DIRECTION {:?}", position);
                xmas_count = xmas_count + 1;
            }
        }
        xmas_count
    }
}
