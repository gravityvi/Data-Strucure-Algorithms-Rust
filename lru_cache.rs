use std::collections::HashMap;
use std::collections::VecDeque;

struct LRUCache {
    map: HashMap<i32, i32>,
    capacity: i32,
    queue: VecDeque<i32>


}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LRUCache {

    fn new(capacity: i32) -> Self {
        return LRUCache {
            map: HashMap::new(),
            capacity: capacity,
            queue: VecDeque::with_capacity(10)
        }
    }
    
    fn get(&mut self, key: i32) -> i32 {
        let map = &self.map;
        let returnValue = match map.get(&key) {
            Some(&val) => { //cache hit
                self.remove(key);
                self.queue.push_front(key);
                val
            },
            None => -1, // cache miss
        };
        return returnValue;
    }
    
    fn put(&mut self, key: i32, value: i32) {
       

        if(self.queue.len() as i32 == self.capacity && !self.map.contains_key(&key)) {
            match self.queue.pop_back() {
                Some(removedKey) => {
                    self.map.remove(&removedKey);
                },
                _ => {}
            };
            self.queue.push_front(key);
            self.map.insert(key, value);
        } else {
            match self.map.get(&key) {
                Some(&val) => {
                    self.remove(key);
                },
                _ => {}
            }
            self.queue.push_front(key);
            self.map.insert(key, value);
            
        }

        
    }

    fn remove(&mut self, key: i32) {
        let mut queue = &mut self.queue;
        queue.retain(|&x| x != key);
    }
}



/**
 * Your LRUCache object will be instantiated and called as such:
 * let obj = LRUCache::new(capacity);
 * let ret_1: i32 = obj.get(key);
 * obj.put(key, value);
 */
