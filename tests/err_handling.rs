use serde::{Serialize, Deserialize, de::Error};

#[derive(Serialize, Deserialize, Debug)]
pub struct Dog {
    pub message: String,
    pub status: String,
}

#[tokio::test]
async fn error_handling() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://dog.ceo/api/breeds/image/rando")
        .await?
        .json::<Dog>()
        .await?;

    // panicked at equal
    // TODO: Error is unreachable pattern
    match &resp {
        result => {
            println!("dog image url: {:#?}", result.message);
        },
        Error => {
            println!("err: {:#?}", &resp);
        }
    }
    Ok(())
}
