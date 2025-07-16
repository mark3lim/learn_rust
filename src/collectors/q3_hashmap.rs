/* Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company;
 * for example, “Add Sally to Engineering” or “Add Amir to Sales.”
 * Then let the user retrieve a list of all people in a department or all people in the company by department, sorted alphabetically.
 */
pub mod q3_hashmap {
    use std::collections::HashMap;
    use std::io;

    pub fn test() {
        println!("-----------------[ Start q3_hashmap ]-----------------");
        let mut map: HashMap<String, String> = HashMap::new();
        loop {
            
            let input_string = get_input("input text to and employee to a department (or 'end' to finish): \
                \nfor example, “Add Sally to Engineering” or “Add Amir to Sales.” ");
            if input_string == "end" {
                print_info(&map);
                break;
            }
            
            let words: Vec<&str> = input_string.split_whitespace().collect();
            
            if let (Some(&name), Some(&dept)) = (words.get(1), words.get(3)) {
                map.insert(name.parse().unwrap(), dept.parse().unwrap());
                println!("{:?}", map);
                
            } else {
                println!("Invalid input format");
                continue;
            }
        }
    }

    fn get_input(prompt: &str) -> String {
        let mut input = String::new();
        println!("---------------------------------\n{}", prompt);
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        input.trim().to_string()
    }

    fn print_info(data: &HashMap<String, String>) {
        let mut items: Vec<(&String, &String)> = data.iter().collect();
        items.sort_by(|a, b| a.1.cmp(b.1));
        
        println!("[FINISHED]\n---------------------------------");
        println!("Department\tEmployee");
        println!("---------------------------------");
        for (dept, name) in items {
            println!("{dept}\t\t{name}");
        }
        println!("---------------------------------");
    }
}
