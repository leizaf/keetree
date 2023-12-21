use std::fmt::Debug;
use std::hash::Hash;

use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;

use crate::map::Map;

#[derive(Debug, PartialEq)]
pub enum RemoveResult {
    Success,
    NotFound,
}

#[derive(Debug)]
pub struct Node<V> {
    value: Option<V>,
    children: Map<Node<V>>,
}

impl<V> Node<V> {
    pub fn is_empty(&self) -> bool {
        self.value.is_none() && self.children.is_empty()
    }

    pub fn get<'a>(&self, mut path: impl Iterator<Item = &'a str>) -> Option<&V> {
        path.fold_while(Some(self), |curr, key| match curr {
            Some(curr) => Continue(curr.children.get(key)),
            None => Done(None),
        })
        .into_inner()
        .and_then(|n| n.value.as_ref())
    }

    pub fn get_mut<'a>(&mut self, mut path: impl Iterator<Item = &'a str>) -> Option<&mut V> {
        path.fold_while(Some(self), |curr, key| match curr {
            Some(curr) => Continue(curr.children.get_mut(key)),
            None => Done(None),
        })
        .into_inner()
        .and_then(|n| n.value.as_mut())
    }

    pub fn insert<'a>(&mut self, path: impl Iterator<Item = &'a str>, value: V) {
        let end = path.fold(self, |curr, key| curr.children.insert(key));
        end.value = Some(value);
    }

    pub fn remove<'a>(&mut self, mut path: impl Iterator<Item = &'a str>) -> RemoveResult {
        if let Some(child_key) = path.next() {
            if let Some(child) = self.children.get_mut(child_key) {
                let result = child.remove(path);
                if result == RemoveResult::Success {
                    self.children.remove(child_key);
                }
                result
            } else {
                RemoveResult::NotFound
            }
        } else {
            self.value = None;
            self.children = Map::default();
            RemoveResult::Success
        }
    }

    fn at_impl<'a>(&self, path: &mut impl Iterator<Item = &'a str>) -> Option<&V> {
        if let Some(child_key) = path.next() {
            self.children
                .matches(child_key)
                .find_map(|child| child.at_impl(path))
        } else {
            self.value.as_ref()
        }
    }

    pub fn at<'a>(&self, mut path: impl Iterator<Item = &'a str>) -> Option<&V> {
        self.at_impl(&mut path)
    }
}

impl<V> Default for Node<V> {
    fn default() -> Self {
        Self {
            value: None,
            children: Map::default(),
        }
    }
}
