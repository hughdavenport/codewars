#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn returns_expected() {
        assert_eq!(order("is2 Thi1s T4est 3a"), "Thi1s is2 3a T4est");
        assert_eq!(order(""), "");
    }
}

fn get_number_from_string(string: &str) -> u64 {
    let mut num: u64 = 0;
    for c in string.chars() {
        if c.is_digit(10) {
            num = 10*num + c.to_digit(10).unwrap() as u64;
        }
    }
    return num
}

fn order(sentence: &str) -> String {
    // code here
    let mut split: Vec<_> = sentence.split_whitespace().collect();
    split.sort_by_key(|s| get_number_from_string(s));
    return split.join(" ");
}

fn main() {
    println!("Hello, world!");
}
