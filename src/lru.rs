// Least Recently Used Implementation for Caching

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[allow(dead_code)]

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

struct Node<T> {
    value: T,
    prev: Link<T>,
    next: Link<T>,
}

struct LRUCache<K, V> {
    map: HashMap<K, Link<V>>,
    head: Link<V>,
    tail: Link<V>,
    capacity: usize,
}

impl<K: Eq + std::hash::Hash, V> LRUCache<K, V> {
    fn new(capacity: usize) -> LRUCache<K, V> {
        LRUCache {
            map: HashMap::new(),
            head: None,
            tail: None,
            capacity,
        }
    }

    fn get(&mut self, key: K) {
        // TODO: return the value if exists in cache, and update the position of the key in DLL
    }

    fn put(&mut self, key: K, value: V) {
        // TODO: insert key-value into the cache, if already exists the update the value and position in DLL
        // TODO: if cache is full, it will remove the least recently used item before inserting new item
    }
}
