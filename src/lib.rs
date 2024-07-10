/// This module provides functions to interact with the Kazama API, facilitating various operations such as
/// chat completions, model pulls, embeddings generation, model listing, and model pushes.
///
/// # Examples
///
/// ## Chat Completion
///
/// ```
/// use kazama::{chat_completion};
///
/// #[tokio::main]
/// async fn main() {
///     chat_completion("model_name", "Hello!", "user").await.expect("Failed to complete chat");
/// }
/// ```
///
/// ## Model Pull
///
/// ```
/// use kazama::{pull_model};
///
/// #[tokio::main]
/// async fn main() {
///     pull_model("model_name", false).await.expect("Failed to pull model");
/// }
/// ```
///
/// ## Generate Embeddings
///
/// ```
/// use kazama::{gen_embeddings};
///
/// #[tokio::main]
/// async fn main() {
///     gen_embeddings("model_name", "Generate embeddings from this prompt").await.expect("Failed to generate embeddings");
/// }
/// ```
///
/// ## List Models
///
/// ```
/// use kazama::{list_models};
///
/// #[tokio::main]
/// async fn main() {
///     list_models().await.expect("Failed to list models");
/// }
/// ```
///
/// ## Push Models
///
/// ```
/// use kazama::{push_models};
///
/// #[tokio::main]
/// async fn main() {
///     push_models("model_name", true).await.expect("Failed to push model");
/// }
/// ```
use serde::{Serialize,Deserialize};


/// Represents a message structure used in the chat request.
#[derive(Debug,Serialize,Deserialize)]
struct Message { 
    role: String,
    content: String,
}

/// Represents a request to perform a chat completion.
#[derive(Debug,Serialize,Deserialize)]
struct chat_request {
    model: String,
    messages: Vec<Message>,
    stream: bool,
}

/// Represents a request to pull a model.
#[derive(Debug,Serialize,Deserialize)]
struct pull_request {
    name: String,
    stream: bool,
}

/// Represents a request to generate embeddings.
#[derive(Debug,Serialize,Deserialize)]
struct emb_request {
    model: String,
    prompt: String,
}

/// Represents a request to push a model.
#[derive(Debug,Serialize,Deserialize)]
 struct push_request {
    name: String,
    stream: bool,
}


/// Sends a request to complete a chat interaction with the specified model, content, and role.
    ///
    /// # Errors
    ///
    /// Returns a `reqwest::Error` if the request fails.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use kazama::chat_completion;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     chat_completion("model_name", "Hello!", "user").await.expect("Failed to complete chat");
    /// }
    /// ```
#[tokio::main]
pub async fn chat_completion(model: &str,content: &str,role: &str) -> Result<(), reqwest::Error>{
    let req = chat_request { 
        model: String::from(model),
        messages: vec![
            Message {role: role.to_string(), content: content.to_string()}
        ],
        stream: false,
    };

    let response = reqwest::Client::new()
        .post("http://localhost:11434/api/chat")
        .json(&req)
        .send()
        .await?;
    let response_json: serde_json::Value = response.json().await?;
    println!("{:#?}",response_json);
    Ok(())
}

 /// Sends a request to pull a model with the specified name and stream mode.
    ///
    /// # Errors
    ///
    /// Returns a `reqwest::Error` if the request fails.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use kazama::pull_model;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     pull_model("model_name", false).await.expect("Failed to pull model");
    /// }
    /// ```
#[tokio::main]
pub async fn pull_model(name:&str,stream_mode: bool) -> Result<(), reqwest::Error>{
    let req = pull_request { 
        name: String::from(name),
        stream: stream_mode,
    };
    let response = reqwest::Client::new()
        .post("http://localhost:11434/api/pull")
        .json(&req)
        .send()
        .await?;
    let response_json: serde_json::Value = response.json().await?;
    println!("{:#?}",response_json);
    Ok(())
}


    /// Sends a request to generate embeddings with the specified model and prompt.
    ///
    /// # Errors
    ///
    /// Returns a `reqwest::Error` if the request fails.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use kazama::gen_embeddings;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     gen_embeddings("model_name", "Generate embeddings from this prompt").await.expect("Failed to generate embeddings");
    /// }
    /// ```
#[tokio::main]
pub async fn gen_embeddings(model:&str,prompt: &str) -> Result<(), reqwest::Error>{
    let req = emb_request { 
        model: String::from(model),
        prompt: String::from(prompt),
    };
    let response = reqwest::Client::new()
        .post("http://localhost:11434/api/embeddings")
        .json(&req)
        .send()
        .await?;
    let response_json: serde_json::Value = response.json().await?;
    println!("{:#?}",response_json);
    Ok(())
}

 /// Sends a request to list available models.
    ///
    /// # Errors
    ///
    /// Returns a `reqwest::Error` if the request fails.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use kazama::list_models;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     list_models().await.expect("Failed to list models");
    /// }
    /// ```
#[tokio::main]
pub async fn list_models() -> Result<(), reqwest::Error>{
    let response = reqwest::get("http://localhost:11434/api/ps")
        .await?;
    println!("{:#?}",response);
    Ok(())
}

   /// Sends a request to push a model with the specified name and stream mode.
    ///
    /// # Errors
    ///
    /// Returns a `reqwest::Error` if the request fails.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use kazama::push_models;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     push_models("model_name", true).await.expect("Failed to push model");
    /// }
    /// ```
#[tokio::main]
pub async fn push_models(name: &str,stream_mode:bool) -> Result<(), reqwest::Error>{
    let req = push_request { 
        name: String::from(name),
        stream: stream_mode,
    };
    let response = reqwest::Client::new()
        .post("http://localhost:11434/api/push")
        .json(&req)
        .send()
        .await?;
    let response_json: serde_json::Value = response.json().await?;
    println!("{:#?}",response_json);
    Ok(())
}


