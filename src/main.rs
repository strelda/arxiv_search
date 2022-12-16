use arxiv_search::{Article,arxiv_scrapper};
use std::env;

fn main(){
    // how to construct query: https://arxiv.org/help/api/user-manual#query_details
    // const search_query: &str = "abs:state_driving OR abs:qubit OR abs:quantum_computing OR abs:quantum_computer OR abs:state_manifold";
    // const since_date: &str = "2022-12-15"; //"%Y-%m-%d"
    //input these parameters when calling the program
    let args: Vec<String> = env::args().collect();
    let search_query = &args[1];
    let since_date = &args[2];
    // scrape arxiv and get a vector of articles
    let articles_all: Vec<Article> = arxiv_scrapper(search_query,since_date);
    
    // print articles to the console
    for article in articles_all.iter() {
        article.print_html();
    }
    

}