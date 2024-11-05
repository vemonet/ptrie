//! Struct and functions for the `Trie` nodes

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::clone::Clone;
use std::cmp::{Eq, Ord};

/// A node in the `Trie`, it holds a value, and a list of children nodes
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct TrieNode<K: Eq + Ord + Clone, V> {
    pub value: Option<V>,
    pub children: Vec<(K, TrieNode<K, V>)>,
}

impl<K: Eq + Ord + Clone, V: Clone> TrieNode<K, V> {
    pub fn new() -> Self {
        TrieNode {
            value: None,
            children: Vec::new(),
        }
    }

    /// Insert a node in the trie
    pub fn insert<I: Iterator<Item = K>>(&mut self, mut key: I, value: V) {
        if let Some(part) = key.next() {
            if let Some(child) = self.children.iter_mut().find(|child| child.0 == part) {
                child.1.insert(key, value);
            } else {
                let mut new_node = TrieNode::new();
                new_node.insert(key, value);
                self.children.push((part, new_node));
            }
        } else {
            self.value = Some(value);
        }
    }

    /// Recursively find a node searching through children
    pub fn find_node<I: Iterator<Item = K>>(&self, mut key: I) -> Option<&Self> {
        if let Some(p) = key.next() {
            self.children.iter().find(|c| c.0 == p)?.1.find_node(key)
        } else {
            Some(self)
        }
    }

    pub fn find_node_mut<I: Iterator<Item = K>>(&mut self, mut key: I) -> Option<&mut Self> {
        if let Some(p) = key.next() {
            self.children.iter_mut().find(|c| c.0 == p)?.1.find_node_mut(key)
        } else {
            Some(self)
        }
    }

    pub fn set_value(&mut self, value: V) {
        self.value = Some(value);
    }

    pub fn get_value(&self) -> Option<&V> {
        self.value.as_ref()
    }

    pub fn get_value_mut(&mut self) -> Option<&mut V> {
        self.value.as_mut()
    }

    pub fn may_be_leaf(&self) -> bool {
        self.value.is_some()
    }
}

impl<T: Eq + Ord + Clone, U: Clone> Default for TrieNode<T, U> {
    fn default() -> Self {
        Self::new()
    }
}
