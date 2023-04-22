use regex::Regex;
use clap::{Arg, App};

fn main() {
    let args = App::new("grep-lite")
        .version("1.0")
        .about("Searches for patterns in files")
        .arg(Arg::with_name("pattern")
            .help("The pattern to search for")
            .takes_value(true)
            .required(true))
        // .arg(Arg::with_name("input")
        //     .help("File to search")
        //     .takes_value(true)
        //     .required(true))
        .get_matches();

    let pattern = args.value_of("pattern").unwrap();
    let re = Regex::new(pattern).unwrap();

    let quote = "Every face, every shop, bedroom window, public-house, and
dark square is a picture feverishly turned--in search of what?
It is the same with books. What do we seek through millions of pages?";

  for line in quote.lines() {
    match re.find(line) {
      Some(_) => println!("{}", line),
      None => (),
    }
  }
}
