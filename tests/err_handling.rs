use std::{collections::HashMap, error::Error};

# [derive(Clone, Debug)]
pub struct Dog {
  pub message: String,
  pub status: String,
}

#[tokio::test]
async fn error_handling() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://dog.ceo/api/breeds/image/random")
        .await?
        .json::<HashMap<String, String>>()
        .await?;
    // panicked at equal
    // TODO: Error is unreachable pattern
    match &resp {
        Result => {
            println!("{:#?}", &resp);
        },
        Error => {
            println!("{:#?}", "err!");
        }
    }
    Ok(())
}
