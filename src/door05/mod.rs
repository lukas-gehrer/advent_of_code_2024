pub mod solution {
    use std::cmp::Ordering;
    use std::collections::HashMap;
    use std::fs::File;
    use std::io;
    use std::io::Read;

    pub fn part_one() {
        println!("SUM: {}", process_file("./src/door05/input.txt"));
    }

    pub fn part_two() {
        println!("SUM PART TWO: {}", process_file("./src/door05/input.txt"));
    }

    fn file_content_to_string(s: &str) -> io::Result<String> {
        let mut file = File::open(s)?;
        let mut s = String::new();
        file.read_to_string(&mut s)?;
        Ok(s)
    }

    fn process_file(file: &str) -> i32 {
        let mut sum = 0;
        let mut rules_hashmap: HashMap<&str, Vec<&str>> = HashMap::new();
        let content_string = file_content_to_string(file).unwrap();
        let mut seperated_ordering_rules_and_page_numbers = content_string.split("\n\n");
        let ordering_rules = seperated_ordering_rules_and_page_numbers
            .next()
            .unwrap()
            .split("\n");
        let page_numbers_lines = seperated_ordering_rules_and_page_numbers
            .next()
            .unwrap()
            .split("\n");

        for rule in ordering_rules.to_owned() {
            let mut rule_split = rule.split("|");
            let left_number = rule_split.next();
            let right_number = rule_split.next();

            rules_hashmap
                .entry(left_number.unwrap())
                .or_insert_with(Vec::new)
                .push(right_number.unwrap());
        }

        for page_numbers_line in page_numbers_lines {
            println!("PAGE NUMBER LINE {}", page_numbers_line);
            let mut page_numbers_split = page_numbers_line.split(",");
            let page_number_array: Vec<&str> = page_numbers_split.clone().collect();
            let middle_number = page_number_array[page_number_array.len() / 2];

            let numbers_are_in_valid_order = check_rules(&rules_hashmap, &page_number_array);

            if numbers_are_in_valid_order {
                println!("LINE IS IN VALID ORDER {}", page_numbers_line);
                sum += middle_number.parse::<i32>().unwrap();
            }
        }
        sum
    }

    fn check_rules(
        rules_hashmap: &HashMap<&str, Vec<&str>>,
        page_numbers_split: &Vec<&str>,
    ) -> bool {
        check_rules_recursive(rules_hashmap, page_numbers_split, 0, true)
    }

    fn check_rules_recursive(
        rules_hashmap: &HashMap<&str, Vec<&str>>,
        page_numbers_split: &Vec<&str>,
        page_numbers_split_index: usize,
        mut in_correct_order: bool,
    ) -> bool {
        let current_number = page_numbers_split.get(page_numbers_split_index);
        if in_correct_order && current_number.is_some() {
            let mut rule_set = rules_hashmap.get(current_number.unwrap());
            if rule_set.is_some() {
                let mut rule_set_iter = rule_set.unwrap().iter();
                let next_number = page_numbers_split.get(page_numbers_split_index + 1);
                if next_number.is_none() {
                    return in_correct_order;
                }
                for rule in rule_set_iter {
                    if rule == next_number.unwrap() {
                        in_correct_order = true;
                        println!("IN CORRECT ORDER WITH NUMBER {}", current_number.unwrap());
                        break;
                    }
                    in_correct_order = false;
                }
                return check_rules_recursive(
                    rules_hashmap,
                    page_numbers_split,
                    page_numbers_split_index + 1,
                    in_correct_order,
                );
            }
            // there is no entry in the hashmap indicating that it might be the last number
            // check edge case if the current number is the last one in the pages
            else if page_numbers_split.len() - 1 == page_numbers_split_index {
                return in_correct_order;
            }
            return false;
        }
        in_correct_order
    }

    fn process_file_part2(file: &str) -> i32 {
        let mut sum = 0;
        let mut rules_hashmap: HashMap<&str, Vec<&str>> = HashMap::new();
        let content_string = file_content_to_string(file).unwrap();
        let mut seperated_ordering_rules_and_page_numbers = content_string.split("\n\n");
        let ordering_rules = seperated_ordering_rules_and_page_numbers
            .next()
            .unwrap()
            .split("\n");
        let page_numbers_lines = seperated_ordering_rules_and_page_numbers
            .next()
            .unwrap()
            .split("\n");

        for rule in ordering_rules.to_owned() {
            let mut rule_split = rule.split("|");
            let left_number = rule_split.next();
            let right_number = rule_split.next();

            rules_hashmap
                .entry(left_number.unwrap())
                .or_insert_with(Vec::new)
                .push(right_number.unwrap());
        }

        for page_numbers_line in page_numbers_lines {
            println!("PAGE NUMBER LINE {}", page_numbers_line);
            let mut page_numbers_split = page_numbers_line.split(",");
            let page_number_array: Vec<&str> = page_numbers_split.clone().collect();

            let numbers_are_in_valid_order = check_rules(&rules_hashmap, &page_number_array);

            if !numbers_are_in_valid_order {
                println!("LINE IS NOT IN VALID ORDER {}", page_numbers_line);
                // Sort and return middle number
                sum += get_sorted_middle_number(&rules_hashmap, &page_number_array);
            }
        }
        sum
    }

    fn get_sorted_middle_number(
        rules_hashmap: &HashMap<&str, Vec<&str>>,
        page_numbers_split: &Vec<&str>,
    ) -> i32 {
        let mut page_number_array: Vec<&str> = page_numbers_split.clone();

        page_number_array.sort_by(|a, b| {
            let mut rule_set = rules_hashmap.get(a);
            if rule_set.is_none() {
                return Ordering::Greater; // no ruleset found means it is the least number and needs to be swapped
            };
            let mut rule_set_iter = rule_set.unwrap().iter();
            for rule in rule_set_iter {
                if rule == b {
                    return Ordering::Less; // a is already before b
                }
            }
            Ordering::Greater // not in the right order and needs swap
        });

        let middle_number = page_number_array[page_number_array.len() / 2];
        middle_number.parse::<i32>().unwrap()
    }
}
