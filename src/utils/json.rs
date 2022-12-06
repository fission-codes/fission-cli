use anyhow::{Result, bail};
use serde_json::{Map, Value};

/*
Ipfs often returns json with a single property with the value being an array. 
This function simply takes the array in that property and turns it into a vector that can be used in rust.
It will return an error result if the json is not formatted in this way.
*/
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

pub fn change_json_part<I>(root:&Value, loc:I, to:&Value) -> Result<Value>
    where I: Iterator<Item = String> + Clone{
    if loc.clone().count() == 0 {
        // println!("{}", "found it!".red());
        return Ok(to.clone());
    }else if !root.is_object(){
        return Ok(root.clone());
    }else{
        let root_iter = match root.as_object() {
            Some(o) => o.iter(),
            None => return Ok(Value::Object(serde_json::Map::new()))
        };
        let mut new_root:Map<String, Value> = Map::new();
        for (prop, val) in root_iter {
            let mut new_loc = loc.clone();
            let prop_to_match = new_loc.next().unwrap().to_string() ;
            // println!("{} <=> {} = {}", prop, prop_to_match, prop.to_owned() == prop_to_match);
            let new_val = match prop.to_owned() == prop_to_match{
                true => change_json_part(val, new_loc, to)?,
                false => change_json_part(val, loc.clone(), to)?
            };
            new_root.insert(prop.to_owned(), new_val);
        }
        return Ok(Value::Object(new_root));
    }
}
