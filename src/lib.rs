use generator::generate_ran;

mod generator;
pub fn print_random_number() {
    let n = generator::generate_ran();
    println!("Random u8:{}", n);
    
}