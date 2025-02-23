use std:: collections::HashSet;
pub fn new_count_distinct(input_str: &str) -> usize {
    let unique_set  :HashSet<&str>= input_str.split(',').collect();
    unique_set.len()
}
