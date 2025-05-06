// collectors library test
pub mod vec_01 {
    pub fn vec_fn() {
        // Create a new collectors
        let mut vec_1 = Vec::new();
        vec_1.push(0);
        vec_1.push(1);
        vec_1.push(2);
        vec_1.push(3);
    
        for i in &vec_1 {
            println!("{i}");
        }
    }
}