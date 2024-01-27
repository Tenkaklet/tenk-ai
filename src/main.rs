
use cli_prompts::{
    prompts::Input,
    DisplayPrompt
};

use google_generative_ai_rs::v1::api::PostResult::Rest;
use google_generative_ai_rs::v1::{
    api::Client,
    gemini::{request::Request, Content, Part, Role},
};
use serde::Serialize;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let input_prompt = Input::new("Enter your prompt", |s| Ok(s.to_string()))
        .default_value("")
        .help_message("");

    let name = input_prompt.display();

    dotenv::dotenv().ok();
    let _client = Client::new(dotenv::var("API_KEY").unwrap().to_string());

    let txt_request = Request {
        contents: vec![Content {
            role: Role::User,
            parts: vec![Part {
                text: Some(name.unwrap().to_string()),
                inline_data: None,
                file_data: None,
                video_metadata: None,
            }],
        }],
        tools: vec![],
        safety_settings: vec![],
        generation_config: None,
    };

    #[derive(Serialize)]
    struct PostResult {
        text: String,
    }

    let gemresults = _client.post(30, &txt_request).await?;

    // Pattern match to extract and display the text property
    if let Rest(gemini_response) = gemresults {
        if let Some(candidate) = gemini_response.candidates.get(0) {
            if let Some(part) = candidate.content.parts.get(0) {
                if let Some(text) = &part.text {
                    println!("{}", text);
                }
            }
        }
    }

    Ok(())
}
