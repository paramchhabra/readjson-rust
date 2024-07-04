use std::fs;

use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
struct Paragraph{
    data : String,
}

#[derive(Serialize, Deserialize)]
struct Article{
    article:String,
    author:String,
    paragraph:Vec<Paragraph>,
}


fn main(){
    std::process::exit(realmain());
}

fn read_json_type(json: &str) -> Option<Article>{
    return serde_json::from_str(json).ok();
}

fn realmain()->i32{
    let json = fs::read_to_string("main.json").unwrap();
    let new = json.as_str();

    let parsed_article: Article = read_json_type(new).unwrap();

    println!("The name of the first paragraph is: {}", parsed_article.paragraph[0].data);
    0
}