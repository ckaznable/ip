fn main() -> Result<(), Box<dyn std::error::Error>> {
    let res = reqwest::blocking::get("https://api64.ipify.org")?.text()?;
    println!("{res}");
    Ok(())
}
