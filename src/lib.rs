use reqwest::blocking;
use roxmltree;
use std::collections::HashMap;
use chrono::{DateTime,NaiveDate};

extern crate markdown;

pub struct Article {
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
    pub fn print(&self) {
        println!("
        name: {title},\n
        authors: {authors},\n
        published: {published},\n
        summary: {summary},\n
        url: {url}\n",
        title=self.title,
        authors=self.authors.join(", "),
        published=self.published,
        summary=markdown::to_html(&self.summary),
        url=self.url
    );
    }
    pub fn print_html(&self) {
        println!("
        <!DOCTYPE html>
<html>
<head>
  <title>Articles from Arxiv</title>
</head>
<body>
  <h1 style='font-size:40px;'>{title}</h1>
  <p style='color:gray;font-size:30px;'><i>By {authors}</i></p>
  <p style='color:gray;font-size:30px;'>Published: {published}</p>
    <p style='font-size:30px;'>{summary}</p>
    <ul>
        <li><a href='{url}'>Link to the article</a></li>
    </ul>
</body>
</html>
",
        title=self.title,
        authors=self.authors.join(", "),
        published=self.published,
        summary=self.summary,
        url=self.url
    );
    }
}


// const SEARCH_QUERY: &str = "abs:state_driving OR abs:qubit OR abs:quantum_computing OR abs:quantum_computer OR abs:state_manifold";

// const TODAY_ONLY: bool = false;

pub fn arxiv_scrapper(search_query: &str, since_date: &str) -> Vec<Article> {
    // Set up the base URL for the arXiv API
    let base_url = "http://export.arxiv.org/api/query?";

    // Set up the query parameters for the search
    let mut params = HashMap::new();
    params.insert("search_query", search_query);
    params.insert("sortBy", "submittedDate");
    params.insert("start", "0");
    params.insert("max_results", "70");

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

        
        
        let since_date_date = NaiveDate::parse_from_str(since_date, "%Y-%m-%d").unwrap();
        // dbg!(&since_date_date);
        let published_datetime = DateTime::parse_from_rfc3339(published).unwrap();
        // since_date_date is sooner then published_datetime, print
        if since_date_date <= published_datetime.date_naive() {
            articles_all.push(Article::new(
                title.to_string(), 
                authors.iter().map(|s| s.to_string()).collect(), 
                published.to_string(), 
                summary.to_string(),
                url.to_string()
            ));
        }
    }

    articles_all
}