pub mod calculator {
    pub struct Calculator {}

    impl Calculator {
        pub fn set_values(data_string: &str) -> Vec<f32> {

            let numbers_str: Vec<&str> = data_string.split(",").map(|c| c.trim()).collect();

            let numbers_i32: Vec<f32> = numbers_str.iter().flat_map(|x| x.parse()).collect();

            println!("{:?}", numbers_i32);
            println!("{}", "-".repeat(40));

            numbers_i32
        }
    }
}
