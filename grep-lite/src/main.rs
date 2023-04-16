fn main() {
    let search_term = "picture";
    let quote = "\
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, 
        sed do eiusmod tempor incididunt ut labore picture et dolore magna aliqua.
    ";

    for line in quote.lines() {
        if line.contains(search_term) {
            println!("Found: {}", line);
        }
    }
}
