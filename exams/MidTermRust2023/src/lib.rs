use std::rc::Rc;

#[derive(Debug)]
struct Stack<T>(Option<Node<T>>);

impl<T> Clone for Stack<T> {
    fn clone(&self) -> Self {
        if let Some(ref n) = self.0 {
            Stack(Some(n.clone()))
        } else {
            Stack(None)
        }
    }
}

#[derive(Debug)]
struct Node<T>(T, Stack<T>);

impl<T> Stack<T> {
    fn new() -> Self {
        Stack(None)
    } // An empty Stack

    fn push(&self, value: T) -> Self {
        Stack(Some(
            Node(value, Stack(None))
        ))
    }

    fn peek(&self) -> Option<&T> {
        match &self {
            None => {None}
            Some(n) => Some(n)
        }
    }

    /// If the stack is empty returns `None`
    /// else returns Some(tuple) where tuple contains a reference to the value
    /// on the top of the stack plus the modified `Stack`
    fn pop(&self) -> Option<(&T, Stack<T>)> {
        if self.is_empty() {
            None
        } else {
            match self {
                None => {None}
                Some(n) => {}
            }
        }
    }

    fn is_empty(&self) -> bool {
        match self.0 {
            None => {true}
            Some(_) => {false}
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_push() {
        let stack = Stack::new();
        assert!(stack.is_empty());

        let stack1 = stack.push(1);
        let stack2 = stack1.push(2);

        let v = stack1.peek().unwrap();
        assert_eq!(1, *v);
        assert_eq!(2, *stack2.peek().unwrap());
    }

    #[test]
    fn check_pop() {
        let stack = Stack::new();
        assert!(stack.pop().is_none());
        let stack1 = stack.push(1).push(2);
        let (v, stack2) = stack1.pop().unwrap();
        assert_eq!(2, *v);
        assert_eq!(1, *stack2.peek().unwrap());
    }
}
