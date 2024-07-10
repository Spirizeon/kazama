use serde::{Serialize,Deserialize};

#[derive(Debug,Serialize,Deserialize)]
struct Message { 
    role: String,
    content: String,
}
#[derive(Debug,Serialize,Deserialize)]
struct ApiRequest {
    model: String,
    messages: Vec<Message>,
    stream: bool,
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error>{
    let req = ApiRequest { 
        model: String::from("gemma:2b"),
        messages: vec![
            Message {role: "user".to_string(), content: "why is the moon white".to_string()}
        ],
        stream: false,
    };

    println!("{:#?}",req);
    let response = reqwest::Client::new()
        .post("http://localhost:11434/api/chat")
        .json(&req)
        .send()
        .await?;
    let response_json: serde_json::Value = response.json().await?;
    println!("{:#?}",response_json);
    println!("{:#?}",req);
    Ok(())
}
