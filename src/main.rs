use error_chain::error_chain;
use std::io::Read;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        Reqwest(reqwest::Error);
        ParseIntError(std::num::ParseIntError);
    }
    errors { RandomResponseError(t: String) }
}

fn parse_number(response: reqwest::blocking::Response) -> Result<u32> {
    let mut response_body = response.text()?;
    response_body.pop();
    response_body
        .parse()
        .chain_err(|| ErrorKind::RandomResponseError(response_body))
}
fn main() -> Result<()> {
    let url =
        format!("https://www.random.org/integers/?num=1&min=0&max=10&col=1&base=10&format=plain");
    let mut response = reqwest::blocking::get(&url)?;
    let random_number = parse_number(response);
    if let Err(error) = random_number {
        match *error.kind() {
            ErrorKind::Io(_) => println!("Standard IO error: {:?}", error),
            ErrorKind::Reqwest(_) => println!("Reqwest error: {:?}", error),
            ErrorKind::ParseIntError(_) => println!("Standard parse int error: {:?}", error),
            ErrorKind::RandomResponseError(_) => println!("User defined error: {:?}", error),
            _ => println!("Other error: {:?}", error),
        }
    }
    Ok(())
}
