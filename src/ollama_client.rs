use ollama_rs::generation::chat::{request::ChatMessageRequest, ChatMessage};
use ollama_rs::Ollama;
use tokio_stream::StreamExt;
use ansi_term::Colour::Red;

pub struct OllamaClient {
    inner: Ollama,
}

impl OllamaClient {
    pub fn new(host: String, port: u16) -> Self {
        let url = format!("http://{host}");
        Self {
            inner: Ollama::new(url, port),
        }
    }

    /// Send a prompt and stream the model response to stdout.
    pub async fn stream_prompt(&self, prompt: String) {
        let req = ChatMessageRequest::new(
            "llama3".to_string(),
            vec![ChatMessage::user(prompt)],
        );

        match self.inner.send_chat_messages_stream(req).await {
            Ok(mut stream) => {
                while let Some(chunk) = stream.next().await {
                    if let Ok(resp) = chunk {
                        print!("{}", resp.message.content);
                    }
                }
                println!();
            }
            Err(e) => {
                eprintln!("{}", Red.paint(format!("Failed to communicate with Ollama: {e}")));
            }
        }
    }
}
