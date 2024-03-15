#[cfg(test)]
mod tests {
    use pandas_pouch::lru::LRUCache;

    #[test]
    fn test_lru_cache() {
        let mut cache = LRUCache::new(2);

        cache.put(1, "a");
        assert_eq!(cache.get(1), Some("a"));
        cache.put(2, "b");
        assert_eq!(cache.get(2), Some("b"));

        cache.put(3, "c");
        assert_eq!(cache.get(1), None);
        assert_eq!(cache.get(2), Some("b"));
        assert_eq!(cache.get(3), Some("c"));

        cache.put(4, "d");
        assert_eq!(cache.get(2), None);
        assert_eq!(cache.get(3), Some("c"));
        assert_eq!(cache.get(4), Some("d"));
    }
}