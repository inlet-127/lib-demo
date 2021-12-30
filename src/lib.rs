#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

mod generator;

pub fn print_random_number() {
    let n = generator::gen_ran();
    println!("Random u8: {}", n);
}
