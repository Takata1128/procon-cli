use std::println;

use clap::Parser;
use scraper::Html;
use scraper::Selector;

#[derive(Debug, Parser)]
struct Cli {
    #[arg(
        short,
        long,
        default_value = "atcoder",
        help = "the name of procon services (atcoder / codeforces)."
    )]
    service: String,
    #[arg(
        short,
        long,
        help = "contest type. ( ex.| 'abc', 'arc', 'agc' (atcoder); 'div1', 'div2', 'div3', 'div4' (codeforces) )"
    )]
    types: String,
    #[arg(short, long, help = "contest index.")]
    index: String,
}

struct Problem {
    name: String,
    url: String,
}

impl Problem {
    fn new(name: String, url: String) -> Self {
        Self { name, url }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    let url = String::from("https://atcoder.jp/contests/@#/tasks")
        .replace("@", args.types.as_str())
        .replace("#", args.index.as_str());
    let result = get_reqwest(url).await?;
    let problems = get_problem_info(&result);

    Ok(())
}

async fn get_reqwest(url: String) -> Result<String, Box<dyn std::error::Error>> {
    let body = reqwest::get(url).await?.text().await?;
    Ok(body)
}

fn get_problem_info(html: &str) -> Vec<Problem> {
    let document = Html::parse_document(html);
    let selector_str = "#main-container > div.row > div:nth-child(2) > div > table > tbody > tr";
    let selector = Selector::parse(selector_str).unwrap();

    let mut ret: Vec<Problem> = Vec::new();

    for element in document.select(&selector) {
        let inner_elem = element
            .select(&Selector::parse("td:nth-child(1) a").unwrap())
            .nth(0);

        match inner_elem {
            None => (),
            Some(x) => {
                let prob_url = x.value().attr("href").unwrap().to_string();
                let prob_name = x.text().collect::<Vec<_>>()[0].to_string();
                ret.push(Problem::new(prob_url, prob_name));
            }
        }
    }
    ret
}
