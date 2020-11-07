
#![allow(dead_code)]
#![allow(unused_variables)]
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use super::cycle_queue::CycleQueue;
        let mut cq: CycleQueue<u32> = CycleQueue::new_withcapacity(100);
        for i in 1..101 {
            let _ = cq.push_back(i);
        }
        loop {
            let litem = cq.get_item();
            let (stat,wrap_item) = cq.remove_front();
            if cq.is_empty(){break;}
            if let Some(ritem) = wrap_item {
                assert_eq!(litem, ritem);
            };
        }
    }
}
pub mod cycle_queue{
    use std::ptr;
    
    #[derive(Debug)]
    pub enum Status{
        Full, Empty, Success
    }
    
    #[derive(Debug,Default)]
    pub struct CycleQueue<T> {
        capacity: usize,
        queue: Vec<T>,
        head: usize,
        tail: usize
    }
    
    impl<T> CycleQueue<T> {
        
        #[inline]
        pub fn new() -> Self {
            Self {
                capacity: 0,
                queue: Vec::new(),
                head: 0,
                tail: 0
            }
        }
    
        #[inline]
        pub fn new_withcapacity(capacity: usize) -> Self {
            Self {
                capacity,
                queue: Vec::with_capacity(capacity),
                head: 0,
                tail: 0
            }
        }
        
        #[inline]
        pub fn is_empty(&self) -> bool {
            if self.head == self.tail{
                true
            }else{
                false
            }
        }
        
        #[inline]
        pub fn is_full(&self) -> bool {
            if ((self.tail + 1) % self.capacity) == self.head {
                true
            }else {
                false
            }
        }
        #[inline]
        pub fn push_back(&mut self,item: T) -> (Status,std::io::Result<String>){
            if self.is_full() {
                return (Status::Full,Ok(String::from("当前队列已满!")));
            }
            unsafe {
                let tail = self.queue.as_mut_ptr().add(self.tail);
                ptr::write(tail, item);
            }
            self.tail = (self.tail + 1) % self.capacity;
            (Status::Success,Ok(String::from("添加到队列成功")))
        }
        
        #[inline]
        pub fn remove_front(&mut self) -> (Status,Option<T>) {
            if self.is_empty() { 
                return (Status::Empty,None);
            }
            let t;
            unsafe {
                {
                    let head = self.queue.as_ptr().add(self.head);
                    t =ptr::read(head);
                }
                
            }
            self.head = (self.head+1) % self.capacity;
            (Status::Success,Some(t))
        }
        
        pub fn clear(&mut self){
            self.queue.clear();
            self.head = 0;
            self.tail = 0;
        }
        
        #[inline]
        pub fn get_item(&self) -> T {
            self.get_itemidx(self.head)
        }
        
        #[inline]
        pub fn get_itemidx(&self,index: usize) -> T {
            let ret;
            unsafe {
                let ptr = self.queue.as_ptr().add(index);
                ret = ptr::read(ptr);
            }
            ret
        }
    
        #[inline]
        pub fn get_currsize(&self) -> usize {
            (self.tail + self.capacity - self.head) % self.capacity
        }
    }
}
