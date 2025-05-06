/* Given a list of integers, use a vector and return the median
 * (when sorted, the value in the middle position) and mode (the value that occurs most often;
 * a hash map will be helpful here) of the list.
 */
pub mod q1_list {
    pub fn q1_fn() {
        use rand::Rng;

        let numbers: Vec<i32> = (0..10)
            .map(|_| rand::thread_rng().gen_range(0..=100))
            .collect();

        println!("Numbers: {:?}", numbers);
        
        let median = find_median(&numbers);
        let mode = find_mode(&numbers);
        println!("Median: {median}");
        println!("Mode: key={}, value={}", mode.0, mode.1);
    }

    // This function finds the median of a list of integers.
    fn find_median(numbers: &[i32]) -> i32 {
        let mut sorted_numbers = numbers.to_vec();
        sorted_numbers.sort();
        let len = sorted_numbers.len();
        if len % 2 == 0 {
            (sorted_numbers[len / 2 - 1] + sorted_numbers[len / 2]) / 2
        } else {
            sorted_numbers[len / 2]
        }
    }

    // This function finds the mode of a list of integers.
    fn find_mode(numbers: &[i32]) -> (i32, i32) {
        use std::collections::HashMap;
    
        let mut map = HashMap::new();
    
        for num in numbers {
            let count = map.entry(*num).or_insert(0);
            *count += 1;
        }
    
        map.iter()
            .max_by_key(|&(_, count)| *count)
            .map(|(&num, &count)| (num, count))
            .unwrap_or((-1, 0)) // 모드가 없을 때 (0, 0) 반환
    }
}