use serde::{Serialize, Deserialize};
use std::fmt::{self, Debug};
use std::error;
use std::error::Error;
use std::any::type_name;

#[derive(Serialize, Deserialize, Debug)]
pub struct Dog {
    pub message: String,
    pub status: String,
}

pub enum JsonResult {
    Dog,
    Error
}

impl fmt::Display for Dog {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "json response has error")
    }
}
// TODO: かきこむ
impl error::Error for Dog {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

pub mod my_crate {
    pub struct Dog;
}

// TODO: type assertion println!("dog image url: {:#?}", type_of(&resp));
fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

#[tokio::test]
async fn error_handling() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://dog.ceo/api/breeds/image/rando")
        .await?
        .json::<Dog>()
        .await?;

    match JsonResult::Dog {
        JsonResult::Dog => println!("dog image url: {:#?}", &resp),
        JsonResult::Error => panic!("json response has error: {:#?}", &resp)
    }
    Ok(())
}
