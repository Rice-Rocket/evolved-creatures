pub struct Stack<T: Clone> {
    stack: Vec<T>,
}


impl<T: Clone> Stack<T> {
    pub fn new() -> Stack<T> {
        Stack { stack: Vec::new() }
    }

    /// Push a value to the end of the stack
    pub fn push(&mut self, value: T) {
        self.stack.push(value);
    }
    /// Pop a value from the end of the stack
    pub fn pop(&mut self) -> Option<T> {
        self.stack.pop()
    }
    /// Peek the next value in the stack
    pub fn peek(&self) -> Option<T> {
        match self.stack.last() {
            Some(value) => Some(value.clone()),
            None => None,
        }
    }
    /// The size or length of the stack
    pub fn size(&self) -> usize {
        self.stack.len()
    }
    /// Whether or not the stack is empty
    pub fn is_empty(&self) -> bool {
        self.size() == 0
    }
}


#[macro_export]
macro_rules! stack {
    ($($x:expr), *) => {
        {
            let mut temp_stack = Stack::new();
            $(
                temp_stack.push($x);
            )*
            temp_stack
        }
    };
}