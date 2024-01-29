
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
    
    // Prompt for user input and set the default value to an empty string
    let input_prompt = Input::new("Enter your prompt", |s| Ok(s.to_string()))
        .default_value("")
        .help_message("");
    // Get the user input and store it in a variable
    let user_input = input_prompt.display();

    dotenv::dotenv().ok();
    let _client = Client::new(dotenv::var("API_KEY").unwrap().to_string());

    //  Create a request object with the user input to send to the API
    let txt_request = Request {
        contents: vec![Content {
            role: Role::User,
            parts: vec![Part {
                text: Some(user_input.unwrap().to_string()),
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


    // Post the request to the API & get a response in a variable called gemresults
    // Result of type PostResult is a struct that contains a text property of type String called text.
    let gemresults = _client.post(30, &txt_request).await?;

    // Pattern match to extract and display the text property
    if let Rest(gemini_response) = gemresults {
        if let Some(candidate) = gemini_response.candidates.first() {
            if let Some(part) = candidate.content.parts.first() {
                if let Some(text) = &part.text {
                    println!("{}", text);
                }
            }
        }
    }

    Ok(())
}
