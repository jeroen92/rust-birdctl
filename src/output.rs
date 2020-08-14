pub fn parse(data: String) -> String {
    let mut results = String::new();
    for line in data.lines() {
        match line.chars().next() {
            Some('\n') => continue,
            Some(' ') => {
                results.push_str(&line);
                results.push('\n');
                continue
            },
            _ => (),
        }

        let preamble: String = line.chars().take(4).collect();
        match preamble.as_str() {
            "0001" => println!("Welcome to Bird!"),
            "0000" => return results,
            _ => {
                results.push_str(line.chars().skip(5).collect::<String>().as_str());
                results.push('\n');
            }
        }
    }
    results
}
