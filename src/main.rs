use scraper::{Html, Selector};
use regex::Regex;
use error_chain::error_chain;
use select::document::Document;
use select::predicate::Name;
use std::thread;
use futures::executor;
use std::borrow::Borrow;
use std::collections::HashMap;


fn main() {
    let mut baseUrl = "/wiki/Paper";
    let mut previousUrl = "";
    // let mut count = 0;
    let mut visited = HashMap::new();

    recursiveCall(baseUrl, previousUrl.to_string(), visited).unwrap();
}

fn recursiveCall(url: &str, previousUrl: String, mut visited: HashMap<String, bool>) -> Result<String, Box<dyn std::error::Error>> {
    visited.insert(url.to_string(), true);
    let owned_string: String = "https://en.wikipedia.org".to_owned();
    let new_owned_string = owned_string + url;
    let res = reqwest::blocking::get(new_owned_string.as_str()).unwrap();
    // println!("Status: {}", res.status());
    // println!("Headers:\n{:#?}", res.headers());
    let body = res.text().unwrap();
    let firsLink = parseFirstUrl(body);
    println!("{} --> {}", previousUrl.replace("/wiki/","").replace("_"," "), url.replace("/wiki/","").replace("_"," "));
    if (firsLink != previousUrl && !visited.contains_key(&*firsLink)) {
        let firstLink = recursiveCall(firsLink.as_str(), url.to_string(), visited).unwrap();
    }
    return Ok(firsLink);
}

fn parseFirstUrl(html: String) -> String {
    let document = Html::parse_document(html.as_str());
    let main_content = Selector::parse("div.mw-parser-output").unwrap();
    let p = document.select(&main_content).next().unwrap();
    let p_selector = Selector::parse("p").unwrap();
    let firstLink = "";

    for element in p.select(&p_selector) {
        let mut p_inside = element.inner_html().replace("\n", "");
        if (p_inside != "" || !p_inside.is_empty()) {
            let doc = Document::from(p_inside.as_str());
            let links = doc
                .find(Name("a"))
                .filter_map(|n| n.attr("href"));

            for link in links {
                if (!link.contains("#") && !link.contains("Latin") &&!link.contains("wikimedia") &&!link.contains("wiktionary")&&!link.contains("Greek")&&!link.contains("English")&&!link.contains("File")) {
                    return link.to_string();
                }
            }
        }
    }
    firstLink.to_string()
}