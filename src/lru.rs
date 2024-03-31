// Least Recently Used Implementation for Caching

use std::collections::HashMap;
use std::fmt::Display;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

pub type Link<K, V> = Option<Arc<Mutex<Node<K, V>>>>;

pub struct Node<K, V> {
    key: K,
    value: V,
    expires_at: Instant,
    prev: Link<K, V>,
    next: Link<K, V>,
}

pub struct LRUCache<K, V> {
    map: Arc<Mutex<HashMap<K, Link<K, V>>>>,
    expires: Duration,
    head: Link<K, V>,
    tail: Link<K, V>,
    capacity: usize,
}

impl<K: Eq + std::hash::Hash + Clone + Display, V: Clone + Display> LRUCache<K, V> {
    pub fn new(capacity: usize, expires: Option<Duration>) -> LRUCache<K, V> {
        let expires = expires.unwrap_or(Duration::from_secs(3600)); // default of duration is 3600 seconds (1 hour)
        LRUCache {
            map: Arc::new(Mutex::new(HashMap::new())),
            expires,
            head: None,
            tail: None,
            capacity,
        }
    }

    pub fn get(&mut self, key: K) -> Option<V> {
        let map = self.map.lock().unwrap();
        if let Some(node_link) = map.get(&key).cloned() {
            drop(map);
            if let Some(node_ref) = node_link {
                let node = node_ref.lock().unwrap();
                if node.expires_at < Instant::now() {
                    drop(node);
                    self.remove(key);
                    return None;
                }
                let value = node.value.clone();
                drop(node);
                self.move_to_head(node_ref.clone()); // Clone node_ref
                Some(value)
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn put(&mut self, key: K, value: V) {
        let mut map = self.map.lock().unwrap();
        let node = match map.get(&key) {
            Some(Some(node_link)) => {
                let mut node = node_link.lock().unwrap();
                node.value = value.clone();
                Arc::clone(node_link)
            }
            Some(None) => panic!("Logic error: Node is None for existing key..."),
            None => {
                let new_node = Arc::new(Mutex::new(Node {
                    key: key.clone(),
                    value: value.clone(),
                    expires_at: Instant::now() + self.expires,
                    prev: None,
                    next: self.head.clone(),
                }));
                if let Some(head) = &self.head {
                    let mut head = head.lock().unwrap();
                    head.prev = Some(Arc::clone(&new_node));
                } else {
                    self.tail = Some(Arc::clone(&new_node));
                }
                self.head = Some(Arc::clone(&new_node));

                if map.len() >= self.capacity {
                    if let Some(tail) = self.tail.clone() {
                        let tail = tail.lock().unwrap();
                        let prev = tail.prev.clone();
                        match prev {
                            Some(ref prev) => {
                                let mut prev = prev.lock().unwrap();
                                prev.next = None;
                            }
                            None => self.head = None,
                        }
                        let key_to_remove = tail.key.clone();
                        map.remove(&key_to_remove);
                        self.tail = prev;
                    }
                }
                map.insert(key.clone(), Some(Arc::clone(&new_node)));
                new_node
            }
        };
        drop(map); // Release lock before mutable borrow
        self.move_to_head(node);
    }

    pub fn print(&mut self) -> Vec<(K, V)> {
        let map = self.map.lock().unwrap();
        let mut current = self.head.clone();
        let mut get_all = Vec::new();
        drop(map); // Release lock before mutable borrow
        while let Some(node) = current {
            let node = node.lock().unwrap();
            let key = node.key.clone();
            if node.expires_at < Instant::now() {
                self.remove(key);
            } else {
                get_all.push((key, node.value.clone()));
            }
            current = node.next.clone();
        }
        get_all
    }

    fn detach_node(&mut self, node_ref: Arc<Mutex<Node<K, V>>>) {
        let prev;
        {
            // let map = self.map.lock().unwrap();
            let node = node_ref.lock().unwrap();
            prev = node.prev.clone();
        }
        if let Some(prev_node_ref) = &prev {
            let mut prev_node = prev_node_ref.lock().unwrap();
            prev_node.next = node_ref.lock().unwrap().next.clone();
        } else {
            // node is head of LRUCache DLL, update the head
            self.head = node_ref.lock().unwrap().next.clone();
        }

        if let Some(next_node_ref) = &node_ref.lock().unwrap().next {
            let mut next_node = next_node_ref.lock().unwrap();
            next_node.prev = prev.clone();
        } else {
            // node is in tail, update tail
            self.tail = prev.clone();
        }
    }

    fn remove(&mut self, key: K) -> Option<(K, V)> {
        let node_link = {
            let mut map = self.map.lock().unwrap();
            map.remove(&key)
        };

        if let Some(node_link) = node_link {
            if let Some(node_ref) = node_link.clone() {
                // unlink/detaching node from DLL
                self.detach_node(node_ref.clone());
                let node = node_ref.lock().unwrap();
                return Some((node.key.clone(), node.value.clone()));
            }
        }
        None
    }

    fn move_to_head(&mut self, node_ref: Arc<Mutex<Node<K, V>>>) {
        // unlinking/detaching node from the DLL
        self.detach_node(node_ref.clone());

        // inserting at head
        if let Some(head_ref) = &self.head {
            let mut head = head_ref.lock().unwrap();
            head.prev = Some(node_ref.clone());
            let mut node = node_ref.lock().unwrap();
            node.next = Some(head_ref.clone());
        } else {
            // DLL is empty, both head and tail to the node
            self.tail = Some(node_ref.clone());
        }

        self.head = Some(node_ref);
    }
}
