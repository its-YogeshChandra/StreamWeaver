// create function that returns the parsed (deserialized data )

//import serde json
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]

pub struct ResponseBody<T> {
    pub data: T,
}

impl<T> ResponseBody<T> {
    pub fn create(data: T) -> Self {
        Self { data }
    }
}

// function to desirealize the data - returns Result for proper error handling
pub fn json_deserializer<'de, T>(datastring: &'de str) -> Result<ResponseBody<T>, String>
where
    T: Deserialize<'de>,
{
    // take the input and make the function
    let clean_data = datastring.trim_matches('\0').trim();
    

    
    match serde_json::from_str(clean_data) {
        Ok(json_data) => {
            let return_val = ResponseBody::create(json_data);
            Ok(return_val)
        }
        Err(e) => {
            Err(format!("JSON parse error: {}", e))
        }
    }
}

