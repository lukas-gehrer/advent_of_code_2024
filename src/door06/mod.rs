pub mod solution {
    use crate::read_lines::read_lines::read_lines;
    use std::env;
    use std::path::PathBuf;

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

    pub fn part_two() {}

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
