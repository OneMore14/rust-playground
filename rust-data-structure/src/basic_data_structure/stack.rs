
pub(crate) struct Stack<T> {
    size: i32,
    data: Vec<T>
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        return Stack{
            size: 0,
            data: Vec::new()
        }
    }

    pub fn push(&mut self, value: T) {
        self.data.insert(0, value);
        self.size += 1;
    }

    pub fn pop(&mut self) {
        if self.size == 0 {
            return;
        }
        self.data.remove(0);
        self.size -= 1;
    }

    pub fn size(&self) -> i32 {
        return self.size;
    }

    pub fn is_empty(&self) -> bool {
        return self.size == 0;
    }

    pub fn peek(&self) -> Option<&T> {
        return self.data.first();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let s :Stack<i32> = Stack::new();
        assert_eq!(0, s.size());
    }

    #[test]
    fn test_push() {
        let mut s  = Stack::new();
        s.push(1);
        s.push(2);
        assert_eq!(2, s.size());
        assert_eq!(2, *s.peek().unwrap())
    }

    #[test]
    fn test_pop() {
        let mut s  = Stack::new();
        s.push(1);
        s.push(2);
        s.pop();
        assert_eq!(1, s.size());
        assert_eq!(1, *s.peek().unwrap());

        s.pop();
        s.pop();
        assert_eq!(0, s.size);
        assert_eq!(None, s.peek());
        assert!(s.is_empty());
    }
}