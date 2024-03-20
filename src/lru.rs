// Least Recently Used Implementation for Caching

use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Display;
use std::rc::Rc;
use std::time::{Duration, Instant};

pub type Link<K, V> = Option<Rc<RefCell<Node<K, V>>>>;

pub struct Node<K, V> {
    key: K,
    value: V,
    expires_at: Instant,
    prev: Link<K, V>,
    next: Link<K, V>,
}

pub struct LRUCache<K, V> {
    map: HashMap<K, Link<K, V>>,
    expires: Duration,
    head: Link<K, V>,
    tail: Link<K, V>,
    capacity: usize,
}

impl<K: Eq + std::hash::Hash + Clone + Display, V: Clone + Display> LRUCache<K, V> {
    pub fn new(capacity: usize, expires: Option<Duration>) -> LRUCache<K, V> {
        let expires = expires.unwrap_or(Duration::from_secs(3600)); // default of duration is 3600 seconds (1 hour)
        LRUCache {
            map: HashMap::new(),
            expires,
            head: None,
            tail: None,
            capacity,
        }
    }

    pub fn get(&mut self, key: K) -> Option<V> {
        match self.map.get(&key) {
            Some(node_link) => {
                if node_link.clone()?.borrow().expires_at < Instant::now() {
                    // self.map.remove(&key);
                    self.remove(key);
                    None
                } else {
                    let value = node_link.clone().unwrap().borrow().value.clone();
                    self.move_to_head(node_link.clone()?);
                    Some(value)
                }
            }
            None => None,
        }
    }

    pub fn put(&mut self, key: K, value: V) {
        let node = match self.map.get_mut(&key) {
            Some(Some(node)) => {
                // if key exists, update the value
                node.borrow_mut().value = value.clone();
                node.clone()
            }
            Some(None) => {
                // if the key is present, but no value is associated with it (should not happen in well-behaved LRU)
                panic!("Logic error: Node is None for existing key...");
            }
            None => {
                // if key does not exist, create a new node
                let new_node = Rc::new(RefCell::new(Node {
                    key: key.clone(),
                    value: value.clone(),
                    expires_at: Instant::now() + self.expires,
                    prev: None,
                    next: self.head.clone(),
                }));
                if let Some(head) = &self.head {
                    head.borrow_mut().prev = Some(new_node.clone());
                } else {
                    self.tail = Some(new_node.clone());
                }

                self.head = Some(new_node.clone());

                // if capacity is reached, remove the tail
                if self.map.len() >= self.capacity {
                    if let Some(tail) = self.tail.clone() {
                        let prev = tail.borrow().prev.clone();
                        match prev {
                            Some(ref prev) => prev.borrow_mut().next = None,
                            None => self.head = None,
                        }
                        // remove the tail(key) from map
                        let key_to_remove = tail.borrow().key.clone();
                        self.map.remove(&key_to_remove);
                        self.tail = prev;
                    }
                }

                // insert the new node into map
                self.map.insert(key.clone(), Some(new_node.clone()));
                new_node
            }
        };

        // finally the node is moved to head
        self.move_to_head(node);
    }

    pub fn print(&mut self) -> Vec<(K, V)> {
        let mut current = self.head.clone();
        let mut get_all = Vec::new();
        while let Some(node) = current {
            let key = node.borrow().key.clone();
            if node.borrow().expires_at < Instant::now() {
                // self.map.remove(&key);
                self.remove(key);
            } else {
                get_all.push((key, node.borrow().value.clone()));
            }
            current = node.borrow().next.clone();
        }
        get_all
    }

    fn detach_node(&mut self, node_ref: Rc<RefCell<Node<K, V>>>) {
        if let Some(prev_node_ref) = &node_ref.borrow().prev {
            prev_node_ref.borrow_mut().next = node_ref.borrow().next.clone();
        } else {
            // node is head of LRUCache DLL, update the head
            self.head = node_ref.borrow().next.clone();
        }

        if let Some(next_node_ref) = &node_ref.borrow().next {
            next_node_ref.borrow_mut().prev = node_ref.borrow().prev.clone();
        } else {
            // node is in tail, update tail
            self.tail = node_ref.borrow().prev.clone();
        }
    }

    fn remove(&mut self, key: K) {
        if let Some(node_link) = self.map.get(&key) {
            if let Some(node_ref) = node_link.clone() {
                // unlinking/detaching node from DLL
                self.detach_node(node_ref.clone());

                // remove the node from the map
                self.map.remove(&key);
            }
        }
    }

    fn move_to_head(&mut self, node_ref: Rc<RefCell<Node<K, V>>>) {

        // unlinking/detaching node from the DLL
        self.detach_node(node_ref.clone());

        // inserting at head
        if let Some(head_ref) = &self.head {
            head_ref.borrow_mut().prev = Some(node_ref.clone());
            node_ref.borrow_mut().next = Some(head_ref.clone());
        } else {
            // DLL is empty, both head and tail to the node
            self.tail = Some(node_ref.clone());
        }

        self.head = Some(node_ref);
    }
}
