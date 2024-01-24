pub struct Queue<T: Clone> {
    queue: Vec<T>,
}


impl<T: Clone> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue { queue: Vec::new() }
    }

    pub fn push(&mut self, value: T) {
        self.queue.push(value);
    }
    pub fn pop(&mut self) -> Option<T> {
        if self.queue.len() > 0 {
            Some(self.queue.remove(0))
        } else {
            None
        }
    }
    pub fn peek(&self) -> Option<T> {
        match self.queue.first() {
            Some(value) => Some(value.clone()),
            None => None,
        }
    }
    pub fn size(&self) -> usize {
        self.queue.len()
    }
}


#[macro_export]
macro_rules! queue {
    ($($x:expr), *) => {
        {
            let mut temp_queue = Queue::new();
            $(
                temp_queue.push($x);
            )*
            temp_queue
        }
    };
}