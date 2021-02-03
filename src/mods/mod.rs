mod secrets;

use std::collections::HashMap;

use regex::Regex;
use futures::executor::block_on;
use serde_json::{
    Map,
    Value,
};

type Mods = Vec<Box<dyn Modifier + Send>>;

/// Modifier is able to modify a target of type T
#[async_trait::async_trait]
pub trait Modifier {
    fn key(&self) -> &'static str;
    async fn modify(&mut self, params: Vec<&str>) -> Option<String>;
}

/// Modifiers is a collection of structs that implement the Modifier trait
///
/// Calling `reduce` performs the equivalent of a fold() on modifiers returning
/// the result of modifier.modify() on each fold
pub struct Modifiers {
    mods: Mods
}

impl Modifiers {
    pub fn new(mods: Mods) -> Self {
        Modifiers { mods }
    }

    pub async fn reduce(&mut self, target: String) -> String {
        if self.mods.is_empty() {
            return target;
        }

        let mut res = target.clone();
        for m in self.mods.iter_mut() {
            let mut modified = String::new();

            let pattern = format!("\\{{:{}(:[^:^}}]+)*}}", m.key());
            let re = Regex::new(&pattern).unwrap();
            let mut capture_matches = re.captures_iter(&res);

            let mut capture_params = vec![];
            let mut capture_start = 0;
            let mut capture_end = 0;
            for (i, c) in target.chars().enumerate() {
                if i == capture_end {
                    if let Some(next_captures) = capture_matches.next() {
                        // collect modifier parameters
                        let captured_pattern = next_captures.get(0).unwrap().as_str();
                        let param_matches = Regex::new(r"([^:^{^}]+)").unwrap();
                        let mut params = vec![];
                        for (i, param) in param_matches.captures_iter(captured_pattern).enumerate() {
                            if i > 0 { params.push(param.get(0).unwrap().as_str().clone()); }
                        }
                        capture_params = params;

                        // get capture boundaries (start, end)
                        let captures_match = next_captures.get(0).unwrap().range();
                        capture_start = captures_match.start;
                        capture_end = captures_match.end;
                    }
                }

                if i == capture_start && !capture_params.is_empty() {
                    if let Some(result) = block_on(m.modify(capture_params.clone())) {
                        modified.push_str(result.as_str());
                    }
                    continue;
                }

                if i < capture_start || i >= capture_end { modified.push(c); }
            }

            res = modified;
        }
        res
    }
}

fn load(config: &Map<String, Value>) -> Box<dyn Modifier + Send> {
    let name = config.get("name").unwrap();
    match name.as_str().unwrap() {
        secrets::NAME => {
            let region = config.get("region").unwrap().as_str().unwrap();
            Box::new(secrets::Secrets::new(region))
        },
        _ => panic!(format!("modifier \"{}\" not found", name.as_str().unwrap()))
    }
}

/// Convert vector of serde json Value to Mods required by Modifiers struct
pub fn to_mods(config: Option<&Vec<Value>>) -> Mods {
    let mut modifiers = vec![];

    if let Some(config) = config {
        for modifier in config {
            modifiers.push(
            load(modifier.as_object().unwrap())
            )
        }
    }

    modifiers
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn modifiers_reduce_no_mods() {
        let config: Mods = vec![];
        let mut mods = Modifiers::new(config);

        let target = String::from("original");

        let expected = String::from("original");
        let actual = mods.reduce(target).await;

        assert_eq!(actual, expected);
    }

    #[tokio::test]
    async fn modifiers_reduce_single_mod() {
        struct ModifierMock {}
        #[async_trait::async_trait]
        impl Modifier for ModifierMock {
            fn key(&self) -> &'static str { "modifier-mock" }
            async fn modify(&mut self, params: Vec<&str>) -> Option<String> {
                Some(format!("{} {}", params[0], params[1]))
            }
        }

        let config: Mods = vec![Box::new(ModifierMock {})];
        let mut mods = Modifiers::new(config);

        let target = String::from("{:modifier-mock:key:value}");

        let expected = String::from("key value");
        let actual = mods.reduce(target).await;

        assert_eq!(actual, expected);
    }

    #[tokio::test]
    async fn modifiers_reduce_chained_mods() {
        struct ModifierMock {}
        struct ChainedModifierMock {}
        #[async_trait::async_trait]
        impl Modifier for ModifierMock {
            fn key(&self) -> &'static str { "modifier-mock" }
            async fn modify(&mut self, params: Vec<&str>) -> Option<String> {
                Some(format!("{{:chained-modifier-mock:{}:{}}}", params[0], params[1]))
            }
        }
        #[async_trait::async_trait]
        impl Modifier for ChainedModifierMock {
            fn key(&self) -> &'static str { "chained-modifier-mock" }
            async fn modify(&mut self, params: Vec<&str>) -> Option<String> {
                Some(format!("{} {}", params[0], params[1]))
            }
        }

        let config: Mods = vec![
            Box::new(ModifierMock {}),
            Box::new(ChainedModifierMock {})
        ];
        let mut mods = Modifiers::new(config);

        let target = String::from("{:modifier-mock:key:value}");

        let expected = String::from("key value");
        let actual = mods.reduce(target).await;

        assert_eq!(actual, expected);
    }
}