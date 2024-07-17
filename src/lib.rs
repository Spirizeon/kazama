use serde::{Deserialize, Serialize};
/// Represents a message structure used in the chat request.
#[derive(Debug, Serialize, Deserialize)]
struct Message {
    role: String,
    content: String,
}

/// Represents a request to perform a chat completion.
#[derive(Debug, Serialize, Deserialize)]
struct ChatRequest {
    /// Name or identifier of the model to use for chat completion
    model: String,
    messages: Vec<Message>,
    stream: bool,
}

/// Represents a request to pull a model.
#[derive(Debug, Serialize, Deserialize)]
struct PullRequest {
    name: String,
    stream: bool,
}

/// Represents a request to generate embeddings.
#[derive(Debug, Serialize, Deserialize)]
struct EmbRequest {
    model: String,
    prompt: String,
}

/// Represents a request to push a model.
#[derive(Debug, Serialize, Deserialize)]
struct PushRequest {
    name: String,
    stream: bool,
}


/// Performs a chat completion request.
///
/// Sends a chat completion request using the specified model, message content, and role.
/// Returns an error if the request fails.
#[tokio::main]
pub async fn chat_completion(model: &str, content: &str, role: &str) -> Result<(), reqwest::Error> {
    let req = ChatRequest {
        model: String::from(model),
        messages: vec![Message {
            role: role.to_string(),
            content: content.to_string(),
        }],
        stream: false,
    };

    let response = reqwest::Client::new()
        .post("http://localhost:11434/api/chat")
        .json(&req)
        .send()
        .await?;
    let response_json: serde_json::Value = response.json().await?;
    println!("{:#?}", response_json);
    Ok(())
}


/// Performs a model pull request.
///
/// Sends a request to pull a model identified by `name`.
/// Uses streaming mode if `stream_mode` is true.
/// Returns an error if the request fails.
#[tokio::main]
pub async fn pull_model(name: &str, stream_mode: bool) -> Result<(), reqwest::Error> {
    let req = PullRequest {
        name: String::from(name),
        stream: stream_mode,
    };
    let response = reqwest::Client::new()
        .post("http://localhost:11434/api/pull")
        .json(&req)
        .send()
        .await?;
    let response_json: serde_json::Value = response.json().await?;
    println!("{:#?}", response_json);
    Ok(())
}

/// Performs a request to generate embeddings.
///
/// Sends a request to generate embeddings using the specified model and prompt.
/// Returns an error if the request fails.
#[tokio::main]
pub async fn gen_embeddings(model: &str, prompt: &str) -> Result<(), reqwest::Error> {
    let req = EmbRequest {
        model: String::from(model),
        prompt: String::from(prompt),
    };
    let response = reqwest::Client::new()
        .post("http://localhost:11434/api/embeddings")
        .json(&req)
        .send()
        .await?;
    let response_json: serde_json::Value = response.json().await?;
    println!("{:#?}", response_json);
    Ok(())
}
/// Lists available models.
///
/// Retrieves a list of available models from the server.
/// Prints the response to standard output.
/// Returns an error if the request fails.
#[tokio::main]
pub async fn list_models() -> Result<(), reqwest::Error> {
    let response = reqwest::get("http://localhost:11434/api/tags").await?;
    println!("{:#?}", response);
    Ok(())
}

/// Performs a model push request.
///
/// Sends a request to push a model identified by `name`.
/// Uses streaming mode if `stream_mode` is true.
/// Returns an error if the request fails.
#[tokio::main]
pub async fn push_models(name: &str, stream_mode: bool) -> Result<(), reqwest::Error> {
    let req = PushRequest {
        name: String::from(name),
        stream: stream_mode,
    };
    let response = reqwest::Client::new()
        .post("http://localhost:11434/api/push")
        .json(&req)
        .send()
        .await?;
    let response_json: serde_json::Value = response.json().await?;
    println!("{:#?}", response_json);
    Ok(())
}
/// Unit tests for the functions in this module.
#[cfg(test)]
mod tests{

    use super::*;
     /// Test for `chat_completion` function.
    #[test]
    fn chat_test(){
        let _ = chat_completion("model_name", "Hello!", "user");
    }
/// Test for `pull_model` function.
    #[test]
    fn pull_test(){
        // Example: Model Pull
        let _ = pull_model("model_name", false);
    }
/// Test for `gen_embeddings` function.
    #[test]
    fn gen_embed_test(){
        // Example: Generate Embeddings
        let _ = gen_embeddings("model_name", "Generate embeddings from this prompt");
    }
/// Test for `list_models` function.
    #[test]
    fn listing(){
        // Example: List Models
        let _ = list_models();
    }
   /// Test for `push_models` function.
    #[test]
    fn pushing(){
        // Example: Push Models
        let _ = push_models("model_name", true);
    }
}

