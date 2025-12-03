fn main() -> Result<(), String> {
    let num = 100;
    let div = 0;

    if div == 0 {
        return Err("Some divide by zero error".to_string());
    } else {
        println!("{}",num/div);
        Ok(())
    }
}
