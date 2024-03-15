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
        // TODO: add tests
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

    fn move_to_head(&mut self, node_ref: Rc<RefCell<Node<K, V>>>) {
        if let Some(prev_node_ref) = &node_ref.borrow().prev {
            prev_node_ref.borrow_mut().next = node_ref.borrow().next.clone();
        } else {
            // node is head of LRUCache DLL, update the head
            self.head = node_ref.borrow().next.clone();
        }

        if let Some(next_node_ref) = &node_ref.borrow().next {
            next_node_ref.borrow_mut().prev = node_ref.borrow().prev.clone();
        } else {
            // node in tail, update tail
            self.tail = node_ref.borrow().prev.clone();
        }

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
