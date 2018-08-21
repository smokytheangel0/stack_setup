#[allow(non_snake_case)]
fn main() -> String {
    let thisBOX = String::from("Hello, world!");
    println!("{}", &thisBOX);
    thisBOX
}

/// this tests the main function to see if it poops out a string
/// that says hello world!
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn main_works(){
        assert_eq!(main(), "Hello, world!");
    }
}