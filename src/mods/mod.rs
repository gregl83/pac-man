mod chunks;
mod secrets;
mod uuid;

use regex::Regex;
use futures::executor::block_on;
use serde_json::{
    Map,
    Value,
};

use crate::adapters::secrets::get_secret;

type Mod = Box<dyn Modifier + Send>;
type Mods = Vec<Mod>;

/// Modifier is able to modify a target of type T
#[async_trait::async_trait]
pub trait Modifier {
    fn key(&self) -> &'static str;

    fn option(&self, _: &str) -> Option<String> { None }

    async fn modify(&mut self, params: Vec<&str>) -> Option<String>;

    fn advance(&mut self) { }
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

    pub fn find(&self, key: &str) -> Option<&Mod> {
        self.mods.iter().find(|m| { key == m.key() })
    }

    pub async fn reduce(&mut self, target: String) -> String {
        if self.mods.is_empty() { return target; }

        let mut res = target.clone();
        // iterate over modifiers applying modify on res (clone of target string)
        for m in self.mods.iter_mut() {
            let mut modified = String::new();

            // find modifier matches {:name:key:sub-key}
            let pattern = format!("\\{{:{}[^}}]*}}", m.key());
            let re = Regex::new(&pattern).unwrap();
            let mut capture_matches = re.captures_iter(&res);

            // get first capture; if empty continue or try next modifier
            let mut next_capture = capture_matches.next();
            if next_capture.is_none() { continue; }

            // set capture start and end char position tracking
            let mut capture_params = vec![];
            let mut capture_start = 0;
            let mut capture_end = 0;

            // iterate over target characters
            for (i, c) in res.chars().enumerate() {
                // check if current character position is end of a capture position (get next capture)
                if i == capture_end {
                    // after first pass, next_capture is refreshed
                    if i > 0 { next_capture = capture_matches.next(); }

                    // if next capture has value, collect capture params
                    if let Some(next_captures) = &next_capture {
                        // collect modifier parameters
                        let captured_pattern = next_captures.get(0).unwrap().as_str();
                        let param_matches = Regex::new(r"([^:^{^}]+)").unwrap();
                        let mut params = vec![];
                        for (i, param) in param_matches.captures_iter(captured_pattern).enumerate() {
                            if i > 0 { params.push(param.get(0).unwrap().as_str().clone()); }
                        }
                        capture_params = params;

                        // update capture boundaries (start and end positions)
                        let captures_match = next_captures.get(0).unwrap().range();
                        capture_start = captures_match.start;
                        capture_end = captures_match.end;
                    }
                }

                // check if current character position is start of a capture position (modify + push)
                if i == capture_start {
                    if let Some(result) = block_on(m.modify(capture_params.clone())) {
                        modified.push_str(result.as_str());
                    }
                    continue;
                }

                // if not inside capture boundary (start and end position) push character
                if i < capture_start || i >= capture_end { modified.push(c); }
            }

            res = modified;
        }
        res
    }

    pub fn advance(&mut self) {
        for m in self.mods.iter_mut() { m.advance(); }
    }
}

fn load(config: &Map<String, Value>) -> Box<dyn Modifier + Send> {
    let name = config.get("name").unwrap();
    match name.as_str().unwrap() {
        chunks::NAME => {
            let start = config.get("start").unwrap().as_u64().unwrap();
            let chunk_length = config
                .get("chunk")
                .unwrap()
                .get("length")
                .unwrap()
                .as_u64()
                .unwrap();
            let end = match config.get("end") {
                Some(end) => end.as_u64(),
                _ => None
            };
            let bytes = format!("{}", config.get("bytes").unwrap());
            Box::new(chunks::Chunks::new(start, chunk_length, end, bytes.as_str()))
        },
        secrets::NAME => {
            let region = config.get("region").unwrap().as_str().unwrap();
            Box::new(secrets::Secrets::new(region, get_secret))
        },
        uuid::NAME => Box::new(uuid::Uuid::new()),
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
    async fn modifiers_reduce_no_matches() {
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

        let target = String::from("no matches in this string");

        let expected = target.clone();
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
    async fn modifiers_reduce_single_mod_sans_params() {
        struct ModifierMock {}
        #[async_trait::async_trait]
        impl Modifier for ModifierMock {
            fn key(&self) -> &'static str { "modifier-mock" }
            async fn modify(&mut self, _: Vec<&str>) -> Option<String> {
                Some(String::from("modified"))
            }
        }

        let config: Mods = vec![Box::new(ModifierMock {})];
        let mut mods = Modifiers::new(config);

        let target = String::from("result: {:modifier-mock}");

        let expected = String::from("result: modified");
        let actual = mods.reduce(target).await;

        assert_eq!(actual, expected);
    }

    #[tokio::test]
    async fn modifiers_reduce_single_mod_multimatch() {
        struct ModifierMock {}
        #[async_trait::async_trait]
        impl Modifier for ModifierMock {
            fn key(&self) -> &'static str { "modifier-mock" }
            async fn modify(&mut self, params: Vec<&str>) -> Option<String> {
                Some(format!("{}|{}", params[0], params[1]))
            }
        }

        let config: Mods = vec![Box::new(ModifierMock {})];
        let mut mods = Modifiers::new(config);

        let target = String::from("?a={:modifier-mock:key:alpha}&b={:modifier-mock:key:bravo}");

        let expected = String::from("?a=key|alpha&b=key|bravo");
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