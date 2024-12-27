pub mod solution {
    use crate::read_lines::read_lines::read_lines;
    use std::collections::HashMap;
    use std::path::PathBuf;
    use std::{env};
    use std::collections::hash_map::Entry;

    enum DIRECTIONS {
        UP,
        RIGHT,
        DOWN,
        LEFT,
    }

    pub fn part_one() {
        let path = env::current_dir().unwrap();
        let example_input = path.join("./src/door06/example_input.txt");
        let input = path.join("./src/door06/input.txt");
        // assert_eq!(41, process_file(example_input));
        process_file(input);
    }

    pub fn part_two() {
        let path = env::current_dir().unwrap();
        let example_input = path.join("./src/door06/example_input.txt");
        let input = path.join("./src/door06/input.txt");
        // assert_eq!(6, process_file_part_two(example_input));
        process_file_part_two(input);

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

    fn process_file(file: PathBuf) -> i32 {
        let mut count: i32 = 0;

        let mut grid = generate_grid(file);

        count = count_walked_path(&mut grid);

        println!("Door 06 part one result {}", count);
        count
    }

    fn process_file_part_two(file: PathBuf) -> usize {
        let mut count: usize = 0;

        let mut grid = generate_grid(file);

        count = count_possible_loops(&mut grid);

        println!("Door 06 part two result {}", count);
        count
    }

    fn count_possible_loops(grid: &mut Vec<Vec<char>>) -> usize {
        let mut loop_count = 0;
        let (y_start, x_start) = find_start(grid);
        let end_x = grid[0].len();
        let end_y = grid.len();

        for (i, line) in grid.iter().enumerate() {
            for (ii, char) in line.iter().enumerate() {
                if grid[i][ii] != '#' {
                    let mut grid_copy = grid.clone();

                    grid_copy[i][ii] = 'O';

                    let mut lines: HashMap<String, String> = HashMap::new();

                    let result = found_loop_recursive(
                        &mut grid_copy,
                        y_start,
                        x_start,
                        end_y,
                        end_x,
                        DIRECTIONS::UP,
                        false,
                        &mut lines,
                        y_start,
                        x_start,
                    );

                    if result {
                        loop_count = loop_count + 1;
                    }
                }
            }
        }
        loop_count
    }

    fn found_loop_recursive(
        grid: &mut Vec<Vec<char>>,
        y: usize,
        x: usize,
        end_y: usize,
        end_x: usize,
        mut current_direction: DIRECTIONS,
        mut found_obstacle: bool,
        lines: &mut HashMap<String, String>,
        mut line_y_start: usize,
        mut line_x_start: usize,
    ) -> bool {
        let mut next_x = x;
        let mut next_y = y;
        let mut changed_direction = false;
        grid[y][x] = 'X';
        match current_direction {
            DIRECTIONS::UP => {
                if y.checked_sub(1).is_none() {
                    return false;
                }
                if grid[y - 1][x] == '#' || grid[y - 1][x] == 'O' {
                    current_direction = get_next_direction(current_direction);
                    changed_direction = true;
                } else {
                    next_y = y - 1;
                }
            }
            DIRECTIONS::RIGHT => {
                if x + 1 >= end_x {
                    return false;
                }
                if grid[y][x + 1] == '#' || grid[y][x + 1] == 'O' {
                    current_direction = get_next_direction(current_direction);
                    changed_direction = true;
                } else {
                    next_x = x + 1;
                }
            }
            DIRECTIONS::DOWN => {
                if y + 1 >= end_y {
                    return false;
                }
                if grid[y + 1][x] == '#' || grid[y + 1][x] == 'O' {
                    current_direction = get_next_direction(current_direction);
                    changed_direction = true;
                } else {
                    next_y = y + 1;
                }
            }
            DIRECTIONS::LEFT => {
                if x.checked_sub(1).is_none() {
                    return false;
                }
                if grid[y][x - 1] == '#' || grid[y][x - 1] == 'O'{
                    current_direction = get_next_direction(current_direction);
                    changed_direction = true;
                } else {
                    next_x = x - 1;
                }
            }
        }

        if changed_direction {
            let is_loop = check_is_loop(lines, line_y_start, line_x_start, y, x);
            if is_loop {
                return true;
            }
            line_y_start = y;
            line_x_start = x;
        }

        found_loop_recursive(
            grid,
            next_y,
            next_x,
            end_y,
            end_x,
            current_direction,
            found_obstacle,
            lines,
            line_y_start,
            line_x_start,
        )
    }

    /// Check if there is an entry in the hashmap for the coordinates start and end
    /// If there is we have passed this way already, and we can call it a loop
    fn check_is_loop(
        lines: &mut HashMap<String, String>,
        line_y_start: usize,
        line_x_start: usize,
        y: usize,
        x: usize,
    ) -> bool {
        let key = format!("{}{}{}{}", line_y_start, line_x_start, y, x);

        match lines.entry(key) {
            Entry::Occupied(_) => return true,
            Entry::Vacant(v) => v.insert("".parse().unwrap())
        };
        false
    }

    fn count_walked_path(grid: &mut Vec<Vec<char>>) -> i32 {
        print_grid(grid);
        let (y, x) = find_start(grid);
        let end_x = grid[0].len();
        let end_y = grid.len();

        count_walked_path_recursive(grid, x, y, end_x, end_y, DIRECTIONS::UP, 0)
    }

    fn count_walked_path_recursive(
        grid: &mut Vec<Vec<char>>,
        x: usize,
        y: usize,
        end_x: usize,
        end_y: usize,
        mut current_direction: DIRECTIONS,
        count: i32,
    ) -> i32 {
        let mut next_x = x;
        let mut next_y = y;
        match current_direction {
            DIRECTIONS::UP => {
                if y.checked_sub(1).is_none() {
                    return count + 1;
                }
                if grid[y - 1][x] == '#' {
                    current_direction = get_next_direction(current_direction);
                } else {
                    next_y = y - 1;
                }
            }
            DIRECTIONS::RIGHT => {
                if x + 1 >= end_x {
                    return count + 1;
                }
                if grid[y][x + 1] == '#' {
                    current_direction = get_next_direction(current_direction);
                } else {
                    next_x = x + 1;
                }
            }
            DIRECTIONS::DOWN => {
                if y + 1 >= end_y {
                    return count + 1;
                }
                if grid[y + 1][x] == '#' {
                    current_direction = get_next_direction(current_direction);
                } else {
                    next_y = y + 1;
                }
            }
            DIRECTIONS::LEFT => {
                if x.checked_sub(1).is_none() {
                    return count + 1;
                }
                if grid[y][x - 1] == '#' {
                    current_direction = get_next_direction(current_direction);
                } else {
                    next_x = x - 1;
                }
            }
        }
        if grid[y][x] == 'X' {
            return count_walked_path_recursive(
                grid,
                next_x,
                next_y,
                end_x,
                end_y,
                current_direction,
                count, // do not count X double
            );
        }

        grid[y][x] = 'X';

        count_walked_path_recursive(
            grid,
            next_x,
            next_y,
            end_x,
            end_y,
            current_direction,
            count + 1,
        )
    }

    fn get_next_direction(current_direction: DIRECTIONS) -> DIRECTIONS {
        match current_direction {
            DIRECTIONS::UP => DIRECTIONS::RIGHT,
            DIRECTIONS::RIGHT => DIRECTIONS::DOWN,
            DIRECTIONS::DOWN => DIRECTIONS::LEFT,
            _ => DIRECTIONS::UP,
        }
    }

    fn print_grid(grid: &Vec<Vec<char>>) {
        for y in grid {
            for x in y {
                print!("{}", x)
            }
            println!();
        }
        println!()
    }

    fn find_start(grid: &Vec<Vec<char>>) -> (usize, usize) {
        for (y, line) in grid.into_iter().enumerate() {
            for (x, char) in line.into_iter().enumerate() {
                if grid[y][x] == '^' {
                    return (y, x);
                }
            }
        }
        (1000, 1000)
    }
}
