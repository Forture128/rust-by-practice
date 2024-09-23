pub fn reverse(input: &str) -> String {
    let reversed_string = input.chars().rev().collect();
    println!("Reversed string will be {}", reversed_string);
    reversed_string
}
