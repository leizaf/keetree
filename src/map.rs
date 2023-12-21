use alloc::borrow::ToOwned;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::iter::once;

use hashbrown::HashMap;
use regex::Regex;

#[derive(Debug, Default)]
pub struct Map<V> {
    children: HashMap<String, V>,
    regexes: Vec<Regex>,
    param: Option<String>,
    catchall: Option<String>,
}

impl<V: Default> Map<V> {
    pub fn is_empty(&self) -> bool {
        self.children.is_empty()
    }

    pub fn get(&self, key: &str) -> Option<&V> {
        self.children.get(key)
    }

    pub fn get_mut(&mut self, key: &str) -> Option<&mut V> {
        self.children.get_mut(key)
    }

    pub fn insert(&mut self, key: &str) -> &mut V {
        if key.starts_with("r{") && key.ends_with("}") {
            self.regexes.push(Regex::new(key).unwrap())
        } else if key.starts_with(":") {
            self.param = Some(key.to_string());
        } else if key.starts_with("*") {
            self.catchall = Some(key.to_string());
        }
        self.children.entry(key.to_string()).or_default()
    }

    pub fn remove(&mut self, key: &str) -> Option<V> {
        if key.starts_with("r{") && key.ends_with("}") {
            self.regexes.retain(|r| r.as_str() != key);
        } else if key.starts_with(":") {
            self.param = self.param.clone().filter(|p| p != key)
        } else if key.starts_with("*") {
            self.catchall = self.catchall.clone().filter(|c| c != key)
        }
        self.children.remove(key)
    }

    fn match_regex(&self, key: &str) -> impl Iterator<Item = Option<&V>> {
        let key = key.to_owned();
        self.regexes.iter().map(move |r| {
            r.is_match(&key)
                .then(|| self.children.get(r.as_str()).unwrap())
        })
    }

    fn match_param(&self) -> impl Iterator<Item = Option<&V>> {
        once(self.param.as_ref()).map(|p| p.map(|p| self.children.get(p).unwrap()))
    }

    fn match_catchall(&self) -> impl Iterator<Item = Option<&V>> {
        once(self.catchall.as_ref()).map(|c| c.map(|c| self.children.get(c).unwrap()))
    }

    pub fn matches(&self, key: &str) -> impl Iterator<Item = &V> {
        once(self.get(key))
            .chain(self.match_regex(key))
            .chain(self.match_param())
            .chain(self.match_catchall())
            .filter_map(|v| v)
    }
}
