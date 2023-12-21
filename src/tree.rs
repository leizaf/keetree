use crate::map::Map;

enum RemoveResult<T> {
    Success,
    Found(T),
    NotFound,
}

pub struct Node<V> {
    value: Option<V>,
    children: Map<Node<V>>,
    is_catchall: bool,
}

impl<V> Node<V> {
    pub fn is_empty(&self) -> bool {
        self.value.is_none() && self.children.is_empty()
    }

    pub fn get<'a>(&self, mut path: impl Iterator<Item = &'a str>) -> Option<&V> {
        path.try_fold(self, |curr, key| curr.children.get(key).ok_or_else(|| ()))
            .ok()
            .and_then(|n| n.value.as_ref())
    }

    pub fn get_mut<'a>(&mut self, mut path: impl Iterator<Item = &'a str>) -> Option<&mut V> {
        path.try_fold(self, |curr, key| {
            curr.children.get_mut(key).ok_or_else(|| ())
        })
        .ok()
        .and_then(|n| n.value.as_mut())
    }

    pub fn insert<'a>(&mut self, path: impl Iterator<Item = &'a str>, value: V) {
        let end = path.fold(self, |curr, key| {
            let next = curr.children.insert(key);
            if key.starts_with("*") {
                next.is_catchall = true;
            }
            next
        });
        end.value = Some(value);
    }

    fn remove_impl<'a>(&mut self, mut path: impl Iterator<Item = &'a str>) -> RemoveResult<Option<V>> {
        if let Some(child_key) = path.next() {
            if let Some(child) = self.children.get_mut(child_key) {
                match child.remove_impl(path) {
                    RemoveResult::Success => {
                        RemoveResult::Found(
                            self.children.remove(child_key).unwrap().value
                        )
                    }
                    result => result,
                }
            } else {
                RemoveResult::NotFound
            }
        } else {
            self.value = None;
            self.children = Map::default();
            RemoveResult::Success
        }
    }

    pub fn remove<'a>(&mut self, path: impl Iterator<Item = &'a str>) -> Option<V> {
        match self.remove_impl(path) {
            RemoveResult::Found(v) => v,
            _ => None,
        }
    }

    fn at_impl<'a>(&self, path: &mut impl Iterator<Item = &'a str>) -> Option<&V> {
        if self.is_catchall {
            self.value.as_ref()
        } else if let Some(child_key) = path.next() {
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
            is_catchall: false,
        }
    }
}
