mod secrets;

use serde_json::{
    Map,
    Value,
};

fn load(config: &Map<String, Value>) -> Box<dyn Modifier + Send> {
    let name = config.get("name").unwrap();
    match name.as_str().unwrap() {
        "secrets" => {
            let region = config.get("region").unwrap().as_str().unwrap();
            Box::new(secrets::Secrets::new(region))
        },
        _ => panic!(format!("modifier \"{}\" not found", name.as_str().unwrap()))
    }
}

/// Modifier is able to modify a target of type T
#[async_trait::async_trait]
pub trait Modifier {
    async fn modify(&mut self, target: String) -> String;
}

/// Modifiers is a collection of structs that implement the Modifier trait
///
/// Calling `reduce` performs the equivalent of a fold() on modifiers returning
/// the result of modifier.modify() on each fold
pub struct Modifiers {
    mods: Vec<Box<dyn Modifier + Send>>
}

impl Modifiers {
    pub fn new(mods: Option<&Vec<Value>>) -> Self {
        let mut modifiers = vec![];

        if let Some(mods) = mods {
            for modifier in mods {
                modifiers.push(
                    load(modifier.as_object().unwrap())
                )
            }
        }

        Modifiers {
            mods: modifiers
        }
    }

    pub async fn reduce(&mut self, target: String) -> String {
        if self.mods.is_empty() {
            return target;
        }

        let mut res = target;
        for m in self.mods.iter_mut() {
            res = m.modify(res).await;
        }
        res
    }
}