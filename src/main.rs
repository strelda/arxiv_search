use reqwest::blocking;
use roxmltree;
use std::collections::HashMap;
use chrono::{Utc};

//define constants

// how to construct query: https://arxiv.org/help/api/user-manual#query_details
const SEARCH_QUERY: &str = "abs:state_driving OR abs:qubit OR abs:quantum_computing OR abs:quantum_computer OR abs:state_manifold";

const TODAY_ONLY: bool = false;

struct Article {
    title: String,
    authors: Vec<String>,
    published: String,
    summary: String,
    url: String,
}
impl Article {
    fn new(title: String, 
        authors: Vec<String>, 
        published: String, 
        summary: String,
        url: String) -> Article {
            Article {
                title,
                authors,
                published,
                summary,
                url,
            }
        }
    fn print(&self) {
        println!("
        name: {title},\n
        authors: {authors},\n
        published: {published},\n
        summary: {summary},\n
        url: {url}\n",
        title=self.title,
        authors=self.authors.join(", "),
        published=self.published,
        summary=self.summary,
        url=self.url
    );
    }
}

fn main() {
    //set todays date in a format YYYY-MM-DD
    let today = Utc::now().format("%Y-%m-%d").to_string();
    
    // Set up the base URL for the arXiv API
    let base_url = "http://export.arxiv.org/api/query?";

    // Set up the query parameters for the search
    let mut params = HashMap::new();
    params.insert("search_query", SEARCH_QUERY);
    params.insert("sortBy", "submittedDate");
    params.insert("start", "0");
    params.insert("max_results", "10");

    // Build the URL for the request
    let url = reqwest::Url::parse_with_params(base_url, params).unwrap();

    // Send the GET request and retrieve the response
    let response = blocking::get(url).unwrap();
    // println!("{}", response.text().unwrap());

    // Parse the response body as XML
    let binding = response.text().unwrap();
    let doc = roxmltree::Document::parse(&binding).unwrap();
    
    let mut articles_all: Vec<Article> = Vec::new();
    // Extract the information for each article
    
    for entry in doc.descendants().filter(|n| n.has_tag_name("entry")) {
        //rewrite the below initializations into loop and make the element variable accessible outside the loop
        let title = entry.descendants()
                         .find(|n| n.has_tag_name("title"))
                         .unwrap().text().unwrap();
        let authors = entry.descendants()
                           .filter(|n| n.has_tag_name("author"))
                           .map(|n| n.descendants().find(|n| n.has_tag_name("name")).unwrap().text().unwrap())
                           .collect::<Vec<_>>();
        let published = entry.descendants()
                             .find(|n| n.has_tag_name("published"))
                             .unwrap().text().unwrap();
        let summary = entry.descendants()
                           .find(|n| n.has_tag_name("summary"))
                           .unwrap().text().unwrap();
        let url = entry.descendants()
                        .find(|n| n.has_tag_name("id"))
                        .unwrap().text().unwrap();

        //if published does not start with today's date, then skip. But only is TODAY_ONLY is true
        if !TODAY_ONLY || published.starts_with(&today) {
            articles_all.push(Article::new(
                title.to_string(), 
                authors.iter().map(|s| s.to_string()).collect(), 
                published.to_string(), 
                summary.to_string(),
                url.to_string()
            ));
        }

    }

    // Print the result for each article
    for (i,article) in articles_all.iter().enumerate() {
        println!("article #{}", i);
        article.print();
    }

}