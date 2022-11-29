/*
Ipfs often returns json with a single property with the value being an array. 
This function simply takes the array in that property and turns it into a vector that can be used in rust.
It will return an error result if the json is not formatted in this way.
*/

use serde_json::Value;
use anyhow::{Result, bail};

pub fn value_to_vec<A>(json:&Value, index:&str) -> Result<Vec<A>> 
    where A:Clone + for<'de> serde::Deserialize<'de>{
    anyhow::Ok( match json.get(index) {
        Some(list_json) => { 
            match list_json.as_array() {
                Some(list) => {
                    let mut result_list:Vec<A> = vec![];
                    for item_json in list {
                        result_list.push(match serde_json::from_value(item_json.clone()){
                            Ok(x) => x,
                            Err(_) => bail!("Improperly formatted Json: cannot format value {} in index {} to specified type", item_json, index)
                        });
                    }
                    result_list
                },
                None => bail!("Improperly formatted Json: Value at index {} is not an Array", index)
            }
        },
        None => bail!("Improperly formatted Json: Cant locate index {}", index)
    })   
}