pub struct Queue<T> {
    queue: Vec<T>,
}

impl<T> Queue<T> {
    // Constructor for creating a new empty queue
    pub fn new() -> Self {
        Queue { queue: Vec::new() }
    }

    // Method to add an element to the back of the queue
    pub fn enqueue(&mut self, item: T) {
        self.queue.push(item);
    }

    // Method to remove and return the front element of the queue
    pub fn dequeue(&mut self) -> Option<T> {
        if self.queue.is_empty() {
            None
        } else {
            Some(self.queue.remove(0))
        }
    }

    // Method to check if the queue is empty
    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }

    // Method to view the front element without removing it
    pub fn peek(&self) -> Option<&T> {
        self.queue.first()
    }
}