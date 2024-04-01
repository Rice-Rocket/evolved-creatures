#[derive(Debug, Clone)]
pub struct Queue<T: Clone> {
    queue: Vec<T>,
}


impl<T: Clone> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue { queue: Vec::new() }
    }

    /// Push a value to the end of the queue
    pub fn push(&mut self, value: T) {
        self.queue.push(value);
    }

    /// Pop a value from the front of the queue
    pub fn pop(&mut self) -> Option<T> {
        if !self.queue.is_empty() {
            Some(self.queue.remove(0))
        } else {
            None
        }
    }

    /// Peek the next value in the queue
    pub fn peek(&self) -> Option<T> {
        self.queue.first().cloned()
    }

    /// The size or length of the queue
    pub fn size(&self) -> usize {
        self.queue.len()
    }

    /// Whether or not the queue is empty
    pub fn is_empty(&self) -> bool {
        self.size() == 0
    }
}

impl<T: Clone> Default for Queue<T> {
    fn default() -> Self {
        Self::new()
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
