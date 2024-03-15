// Least Recently Used Implementation for Caching

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

type Link<K, V> = Option<Rc<RefCell<Node<K, V>>>>;

struct Node<K, V> {
    key: K,
    value: V,
    prev: Link<K, V>,
    next: Link<K, V>,
}

struct LRUCache<K, V> {
    map: HashMap<K, Link<K, V>>,
    head: Link<K, V>,
    tail: Link<K, V>,
    capacity: usize,
}

impl<K: Eq + std::hash::Hash + Clone, V: Clone> LRUCache<K, V> {
    fn new(capacity: usize) -> LRUCache<K, V> {
        LRUCache {
            map: HashMap::new(),
            head: None,
            tail: None,
            capacity,
        }
    }

    fn get(&mut self, _key: K) {
        // TODO: return the value if exists in cache, and update the position of the key in DLL
    }

    fn put(&mut self, key: K, value: V) {
        // TODO: insert key-value into the cache, if already exists the update the value and position in DLL
        // TODO: if cache is full, it will remove the least recently used item before inserting new item
        let node = match self.map.get_mut(&key).unwrap() {
            Some(node) => {
                node.borrow_mut().value = value.clone();
                node.clone()
            }
            None => {
                let new_node = Rc::new(RefCell::new(Node {
                    key: key.clone(),
                    value: value.clone(),
                    prev: None,
                    next: self.head.clone(),
                }));
                if let Some(head) = &self.head {
                    head.borrow_mut().prev = Some(new_node.clone());
                } else {
                    self.tail = Some(new_node.clone());
                }

                self.head = Some(new_node.clone());

                if self.map.len() > self.capacity {
                    if let Some(tail) = &self.tail {
                        let prev = tail.borrow().prev.clone();
                        match prev {
                            Some(ref prev) => prev.borrow_mut().next = None,
                            None => self.head = None,
                        }
                        let key_to_remove = tail.borrow().key.clone();
                        self.map.remove(&key_to_remove);
                        self.tail = prev;
                    }
                }

                self.map.insert(key.clone(), Some(new_node.clone()));
                new_node
            }
        };

        self.move_to_head(node);
    }

    fn move_to_head(&mut self, _node_ref: Rc<RefCell<Node<K, V>>>) {
        // TODO: remove the node from its current position, update the head/tail, insert the node at the front
    }
}
