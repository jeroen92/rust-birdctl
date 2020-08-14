pub fn parse(data: String) -> String {
    let mut results = String::new();
    for line in data.lines() {
        match line.chars().next() {
            Some('\n') => continue,
            Some(' ') => {
                results.push_str(line.chars().skip(1).collect::<String>().as_str());
                results.push('\n');
                continue
            },
            _ => (),
        }

        let preamble: u16 = line.chars().take(4).collect::<String>().parse::<u16>().expect("Received non-numeric preamble from server");

        match preamble {
            0001 => println!("Welcome to Bird!"),
            0000 => return results,
            _ => {
                results.push_str(line.chars().skip(5).collect::<String>().as_str());
                results.push('\n');
            }
        }
    }
    panic!("Received invalid response from server. Expected message closure")
}
