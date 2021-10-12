use regex::Regex;

fn song_decoder(song: &str) -> String {
    Regex::new("  *").unwrap().replace_all(song.replace("WUB", " ").trim(), " ").to_string()
}

#[cfg(test)]
mod tests {
    use super::song_decoder;
    
    #[test]
    fn returns_expected() {
        assert_eq!(song_decoder("WUBAWUBWUBC"), "A C");
        assert_eq!(song_decoder("AWUBWUBWUBBWUBWUBWUBC"), "A B C");
        assert_eq!(song_decoder("WUBAWUBBWUBCWUB"), "A B C");
        assert_eq!(song_decoder("AWUBBWUBC"), "A B C");
    }
}

fn main() {
    println!("Hello, world!");
}
