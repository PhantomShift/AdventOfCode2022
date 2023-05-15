pub mod utils {
    pub fn split_lines_group(s: &str, n: usize) -> Vec<String> {
        let lines = s.lines().collect::<Vec<&str>>();
        let mut result = Vec::new();
        for chunk in lines.chunks(n) {
            let to_add = chunk.iter().fold(String::new(), |a, e| a + e + "\n");
            result.push(to_add);
        }
    
        result
    }
}