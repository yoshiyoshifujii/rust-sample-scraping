use scraper::{Selector, Html};

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let response = reqwest::blocking::get("https://blog.rust-lang.org")?;
    let body = response.text()?;
    let document = Html::parse_document(&body);

    let selector = Selector::parse(".post-list tr > td.tr + td > a").unwrap();
    let elements = document.select(&selector);

    let f: fn(()) = |e| println!("{}", e.text().next().unwrap());
    elements.for_each(|e| println!("{}", e.text().next().unwrap()));
    elements.map(|e| "");

    Ok(())
}
