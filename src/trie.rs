//! Struct and functions for the `Trie` data structure

use crate::error::TrieError;
use crate::trie_node::TrieNode;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::clone::Clone;
use std::cmp::{Eq, Ord};

/// Prefix tree object, contains 1 field for the `root` node of the tree
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct Trie<K: Eq + Ord + Clone, V> {
    /// Root of the prefix tree
    root: TrieNode<K, V>,
}

impl<K: Eq + Ord + Clone, V: Clone> Trie<K, V> {
    /// Creates a new `Trie` object
    ///
    /// # Example
    ///
    /// ```rust
    /// use ptrie::Trie;
    ///
    /// let t = Trie::<char, String>::new();
    /// ```
    pub fn new() -> Self {
        Trie {
            root: TrieNode::default(),
        }
    }

    /// Looks for the key in trie
    ///
    /// # Example
    ///
    /// ```rust
    /// use ptrie::Trie;
    ///
    /// let mut t = Trie::new();
    /// let data = "test".bytes();
    /// let another_data = "notintest".bytes();
    /// assert!(!t.contains_key(data.clone()));
    /// t.insert(data.clone(), 42);
    ///
    /// assert!(!t.is_empty());
    /// assert!(t.contains_key(data));
    /// assert!(!t.contains_key(another_data));
    /// ```
    pub fn contains_key<I: Iterator<Item = K>>(&self, key: I) -> bool {
        if self.is_empty() {
            return false;
        }
        // self.root.find_node(key).is_some()
        match self.find_node(key) {
            Some(node) => node.may_be_leaf(),
            None => false,
        }
    }

    /// Gets the value from the tree by key
    ///
    /// # Example
    ///
    /// ```rust
    /// use ptrie::Trie;
    ///
    /// let mut t = Trie::new();
    /// let data = "test".bytes();
    /// let another_data = "notintest".bytes();
    /// assert_eq!(t.get(data.clone()), None);
    /// t.insert(data.clone(), 42);
    ///
    /// assert_eq!(t.get(data), Some(42).as_ref());
    /// assert_eq!(t.get(another_data), None);
    /// ```
    pub fn get<I: Iterator<Item = K>>(&self, key: I) -> Option<&V> {
        self.find_node(key).and_then(|node| node.get_value())
    }

    /// Gets the mutable value from the tree by key
    ///
    /// # Example
    ///
    /// ```rust
    /// use ptrie::Trie;
    ///
    /// let mut t = Trie::new();
    /// let data = "test".bytes();
    /// let another_data = "notintest".bytes();
    /// assert_eq!(t.get_mut(data.clone()), None);
    /// t.insert(data.clone(), 42);
    ///
    /// assert_eq!(t.get_mut(data), Some(42).as_mut());
    /// assert_eq!(t.get(another_data), None);
    /// ```
    pub fn get_mut<I: Iterator<Item = K>>(&mut self, key: I) -> Option<&mut V> {
        self.find_node_mut(key).and_then(|node| node.get_value_mut())
    }

    /// Sets the value pointed by a key
    ///
    /// # Example
    ///
    /// ```rust
    /// use ptrie::Trie;
    ///
    /// let mut t = Trie::new();
    /// let data = "test".bytes();
    /// let another_data = "notintest".bytes();
    ///
    /// t.insert(data.clone(), 42);
    ///
    /// assert_eq!(t.get(data.clone()), Some(42).as_ref());
    /// assert!(t.set_value(data.clone(), 43).is_ok());
    /// assert_eq!(t.get(data), Some(43).as_ref());
    /// assert!(t.set_value(another_data, 39)
    ///     .map_err(|e| assert!(e.to_string().starts_with("Key not found")))
    ///     .is_err());
    /// ```
    pub fn set_value<I: Iterator<Item = K>>(&mut self, key: I, value: V) -> Result<(), TrieError> {
        self.find_node_mut(key)
            .ok_or_else(|| TrieError::NotFound("Key not found".to_string()))
            .map(|node| node.set_value(value))
    }

    /// Returns a list of all prefixes in the trie for a given string, ordered from smaller to longer.
    ///
    /// # Example
    ///
    /// ```rust
    /// use ptrie::Trie;
    ///
    /// let mut trie = Trie::new();
    /// trie.insert("abc".bytes(), "ABC");
    /// trie.insert("abcd".bytes(), "ABCD");
    /// trie.insert("abcde".bytes(), "ABCDE");
    ///
    /// let prefixes = trie.find_prefixes("abcd".bytes());
    /// assert_eq!(prefixes, vec![&"ABC", &"ABCD"]);
    /// assert_eq!(trie.find_prefixes("efghij".bytes()), Vec::<&&str>::new());
    /// assert_eq!(trie.find_prefixes("abz".bytes()), Vec::<&&str>::new());
    /// ```
    pub fn find_prefixes<I: Iterator<Item = K>>(&self, key: I) -> Vec<&V> {
        let mut node = &self.root;
        let mut prefixes = Vec::new();
        for k in key {
            if let Some(next) = node.children.iter().find(|(ckey, _)| ckey == &k).map(|(_, n)| n) {
                if let Some(value) = &next.value {
                    prefixes.push(value);
                }
                node = next;
            } else {
                break;
            }
        }
        prefixes
    }

    /// Finds the longest prefix in the `Trie` for a given string.
    ///
    /// # Example
    ///
    /// ```rust
    /// use ptrie::Trie;
    ///
    /// let mut trie = Trie::default();
    /// assert_eq!(trie.find_longest_prefix("http://purl.obolibrary.org/obo/DOID_1234".bytes()), None);
    /// trie.insert("http://purl.obolibrary.org/obo/DOID_".bytes(), "doid");
    /// trie.insert("http://purl.obolibrary.org/obo/".bytes(), "obo");
    ///
    /// assert_eq!(trie.find_longest_prefix("http://purl.obolibrary.org/obo/DOID_1234".bytes()), Some("doid").as_ref());
    /// assert_eq!(trie.find_longest_prefix("http://purl.obolibrary.org/obo/1234".bytes()), Some("obo").as_ref());
    /// assert_eq!(trie.find_longest_prefix("notthere".bytes()), None.as_ref());
    /// assert_eq!(trie.find_longest_prefix("httno".bytes()), None.as_ref());
    /// ```
    pub fn find_longest_prefix<I: Iterator<Item = K>>(&self, key: I) -> Option<&V> {
        {
            let mut current = &self.root;
            let mut last_value: Option<&V> = None.as_ref();
            for k in key {
                if let Some((_, next_node)) = current.children.iter().find(|(key, _)| key == &k) {
                    if next_node.value.is_some() {
                        last_value = next_node.value.as_ref();
                    }
                    current = next_node;
                } else {
                    break;
                }
            }
            last_value
        }
    }

    /// Finds the longest prefix and it's length in the `Trie` for a given string.
    ///
    /// # Example
    ///
    /// ```rust
    /// use ptrie::Trie;
    ///
    /// let mut trie = Trie::default();
    /// assert_eq!(trie.find_longest_prefix_len("http://purl.obolibrary.org/obo/DOID_1234".bytes()), None);
    /// trie.insert("http://purl.obolibrary.org/obo/DOID_".bytes(), "doid");
    /// trie.insert("http://purl.obolibrary.org/obo/".bytes(), "obo");
    ///
    /// assert_eq!(trie.find_longest_prefix_len("http://purl.obolibrary.org/obo/DOID_1234".bytes()), Some((36, &"doid")));
    /// assert_eq!(trie.find_longest_prefix_len("http://purl.obolibrary.org/obo/1234".bytes()), Some((31, &"obo")));
    /// assert_eq!(trie.find_longest_prefix_len("notthere".bytes()), None);
    /// assert_eq!(trie.find_longest_prefix_len("httno".bytes()), None);
    /// ```
    pub fn find_longest_prefix_len<I: Iterator<Item = K>>(&self, key: I) -> Option<(usize, &V)> {
        {
            let mut current = &self.root;
            let mut len = 0;
            let mut last_value: Option<(usize, &V)> = None;
            for k in key {
                if let Some((_, next_node)) = current.children.iter().find(|(key, _)| key == &k) {
                    len += 1;
                    if next_node.value.is_some() {
                        last_value = next_node.value.as_ref().map(|v| (len, v));
                    }
                    current = next_node;
                } else {
                    break;
                }
            }
            last_value
        }
    }

    /// Returns a list of all strings in the `Trie` that start with the given prefix.
    ///
    /// # Example
    ///
    /// ```rust
    /// use ptrie::Trie;
    ///
    /// let mut trie = Trie::new();
    /// trie.insert("app".bytes(), "App");
    /// trie.insert("apple".bytes(), "Apple");
    /// trie.insert("applet".bytes(), "Applet");
    /// trie.insert("apricot".bytes(), "Apricot");
    ///
    /// let strings = trie.find_postfixes("app".bytes());
    /// assert_eq!(strings, vec![&"App", &"Apple", &"Applet"]);
    /// assert_eq!(trie.find_postfixes("bpp".bytes()), Vec::<&&str>::new());
    /// assert_eq!(trie.find_postfixes("apzz".bytes()), Vec::<&&str>::new());
    /// ```
    pub fn find_postfixes<I: Iterator<Item = K>>(&self, prefix: I) -> Vec<&V> {
        let mut postfixes = Vec::new();
        if let Some(node) = self.find_node(prefix) {
            self.collect_values(node, &mut postfixes);
        }
        postfixes
    }

    #[allow(clippy::only_used_in_recursion)]
    fn collect_values<'a>(&self, node: &'a TrieNode<K, V>, values: &mut Vec<&'a V>) {
        if let Some(ref value) = node.value {
            values.push(value);
        }
        for (_, child) in &node.children {
            self.collect_values(child, values);
        }
    }

    /// Checks if the `Trie` is empty
    ///
    /// # Example
    ///
    /// ```rust
    /// use ptrie::Trie;
    ///
    /// let t = Trie::<char, f64>::new();
    /// assert!(t.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.root.children.is_empty()
    }

    /// Clears the trie
    ///
    /// # Example
    ///
    /// ```rust
    /// use ptrie::Trie;
    ///
    /// let mut t = Trie::new();
    /// let data = "test".bytes();
    ///
    /// t.insert(data, String::from("test"));
    /// t.clear();
    /// assert!(t.is_empty());
    /// ```
    pub fn clear(&mut self) {
        self.root = TrieNode::default();
    }

    /// Adds a new key to the `Trie`
    ///
    /// # Example
    ///
    /// ```rust
    /// use ptrie::Trie;
    ///
    /// let mut t = Trie::new();
    /// let data = "test".bytes();
    /// t.insert(data.clone(), 42);
    /// t.insert(data, 42);
    /// t.insert("test2".bytes(), 43);
    /// assert!(!t.is_empty());
    /// ```
    pub fn insert<I: Iterator<Item = K>>(&mut self, key: I, value: V) {
        self.root.insert(key, value);
    }

    /// Removes a key from the trie, if it exists.
    ///
    /// # Example
    ///
    /// ```rust
    /// use ptrie::Trie;
    ///
    /// let mut t = Trie::new();
    /// let data = "test".bytes();
    /// t.insert(data.clone(), 42);
    /// assert!(t.contains_key(data.clone()));
    ///
    /// t.remove(data.clone());
    /// assert!(!t.contains_key(data));
    /// t.remove("toto".bytes());
    /// ```
    pub fn remove<I: Iterator<Item = K>>(&mut self, key: I) -> Option<V> {
        Self::remove_recursive(&mut self.root, key)
    }

    fn remove_recursive<I: Iterator<Item = K>>(node: &mut TrieNode<K, V>, mut key: I) -> Option<V> {
        if let Some(k) = key.next() {
            if let Some(index) = node.children.iter().position(|(key_part, _)| key_part == &k) {
                let child = &mut node.children[index];
                let result = Self::remove_recursive(&mut child.1, key);

                // If the child node is now empty, remove it
                if child.1.value.is_none() && child.1.children.is_empty() {
                    node.children.remove(index);
                }

                return result;
            } else {
                // Key part not found
                return None;
            }
        }

        // Reached the node corresponding to the full key
        node.value.take()
    }

    // pub fn remove<I: Iterator<Item = K>>(&mut self, key: I) -> Option<V> {
    //     let root = &mut self.root;
    //     self.remove_recursive(root, key)
    // }

    // fn remove_recursive<I: Iterator<Item = K>>(
    //     &mut self,
    //     node: &mut TrieNode<K, V>,
    //     mut key: I,
    // ) -> Option<V> {
    //     if let Some(k) = key.next() {
    //         // If the next part of the key exists in the children, recurse deeper
    //         if let Some((_, child)) = node.children.iter_mut().find(|(key_part, _)| key_part == &k) {
    //             let result = self.remove_recursive(child, key);

    //             // If the child is now empty (no value and no children), remove it
    //             if child.value.is_none() && child.children.is_empty() {
    //                 node.children.retain(|(key_part, _)| key_part != &k);
    //             }
    //             return result;
    //         } else {
    //             // If the key is not found, return None
    //             return None;
    //         }
    //     }

    //     // We've reached the node corresponding to the full key
    //     node.value.take()
    // }

    // /// Removes a key from the trie
    // ///
    // /// # Example
    // ///
    // /// ```rust
    // /// use ptrie::Trie;
    // ///
    // /// let mut t = Trie::new();
    // /// let data = "test".bytes();
    // /// t.insert(data.clone(), 42);
    // /// assert!(t.contains_key(data.clone()));
    // /// t.remove(data.clone());
    // /// assert!(!t.contains_key(data));
    // /// ```
    // pub fn remove<I: Iterator<Item = K>>(&mut self, key: I) -> Option<V> {
    //     let mut current = &mut self.root;
    //     let mut path = Vec::new();

    //     // Traverse the trie to find the node
    //     for k in key {
    //         if let Some(index) = current.children.iter().position(|(ckey, _)| ckey == &k) {
    //             path.push((current, index));
    //             current = &mut current.children[index].1;
    //         } else {
    //             return None; // Key not found
    //         }
    //     }

    //     // Remove the value from the leaf node
    //     let value = current.value.take();

    //     // Remove unnecessary nodes
    //     while let Some((parent, child_index)) = path.pop() {
    //         if current.children.is_empty() && current.value.is_none() {
    //             parent.children.remove(child_index);
    //         } else {
    //             break;
    //         }
    //         current = parent;
    //     }

    //     value
    // }

    /// Finds the node in the `Trie` for a given key
    ///
    /// Internal API
    fn find_node<I: Iterator<Item = K>>(&self, key: I) -> Option<&TrieNode<K, V>> {
        self.root.find_node(key)
    }

    fn find_node_mut<I: Iterator<Item = K>>(&mut self, key: I) -> Option<&mut TrieNode<K, V>> {
        self.root.find_node_mut(key)
    }

    /// Iterate the nodes in the `Trie`
    ///
    /// # Example
    ///
    /// ```
    /// use ptrie::Trie;
    ///
    /// let mut t = Trie::new();
    /// let test = "test".bytes();
    /// let tes = "tes".bytes();
    ///
    /// t.insert(test.clone(), String::from("test"));
    /// t.insert(tes.clone(), String::from("tes"));
    /// for (k, v) in t.iter() {
    ///     assert!(std::str::from_utf8(&k).unwrap().starts_with("tes"));
    ///     assert!(v.starts_with("tes"));
    /// }
    /// ```
    pub fn iter(&self) -> TrieIterator<K, V> {
        TrieIterator::new(self)
    }
}

impl<'a, K: Clone + Ord, V: Clone> IntoIterator for &'a Trie<K, V> {
    type IntoIter = TrieIterator<'a, K, V>;
    type Item = (std::vec::Vec<K>, V);
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

/// Implement the `Default` trait for `Trie` since we have a constructor that does not need arguments
impl<T: Eq + Ord + Clone, U: Clone> Default for Trie<T, U> {
    fn default() -> Self {
        Self::new()
    }
}

/// Iterator for the `Trie` struct
pub struct TrieIterator<'a, K: Eq + Ord + Clone, V> {
    // Stack with node reference and current path
    stack: Vec<(&'a TrieNode<K, V>, Vec<K>)>,
}

impl<'a, K: Eq + Ord + Clone, V: Clone> TrieIterator<'a, K, V> {
    fn new(trie: &'a Trie<K, V>) -> Self {
        TrieIterator {
            // Start with root node and empty path
            stack: vec![(&trie.root, Vec::new())],
        }
    }
}

impl<'a, K: Eq + Ord + Clone, V: Clone> Iterator for TrieIterator<'a, K, V> {
    // Yield key-value pairs
    type Item = (Vec<K>, V);
    fn next(&mut self) -> Option<Self::Item> {
        while let Some((node, path)) = self.stack.pop() {
            // Push children to the stack with updated path
            for (key_part, child) in &node.children {
                let mut new_path = path.clone();
                new_path.push(key_part.clone());
                self.stack.push((child, new_path));
            }
            // Return value if it exists
            if let Some(ref value) = node.value {
                return Some((path, value.clone()));
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    #[test]

    fn longest_prefix() {
        use super::Trie;

        let mut trie = Trie::default();
        trie.insert("hello".bytes(), 1u8);
        trie.insert("h".bytes(), 2u8);

        assert_eq!(trie.find_longest_prefix_len("hello".bytes()), Some((5, &1)));
        assert_eq!(trie.find_longest_prefix_len("h".bytes()), Some((1, &2)));
        assert_eq!(trie.find_longest_prefix_len("he".bytes()), Some((1, &2)));
    }
}
