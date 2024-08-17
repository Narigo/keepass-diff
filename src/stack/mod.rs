use std::fmt::Display;
use std::rc::Rc;

pub struct Stack<T> {
    head: Link<T>,
}

type Link<T> = Option<Rc<Node<T>>>;

struct Node<T> {
    value: T,
    next: Link<T>,
}

impl<T> Stack<T> {
    pub fn empty() -> Self {
        Stack { head: None }
    }

    pub fn append(&self, value: T) -> Stack<T> {
        Stack {
            head: Some(Rc::new(Node {
                value,
                next: self.head.clone(),
            })),
        }
    }

    pub fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.value)
    }

    pub fn tail(&self) -> Stack<T> {
        Stack {
            head: self.head.as_ref().and_then(|node| node.next.clone()),
        }
    }

    pub fn len(&self) -> usize {
        match self.head() {
            Some(_) => 1 + self.tail().len(),
            None => 0,
        }
    }
}

impl<T: Display> Display for Stack<T> {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.mk_string("Stack(", ", ", ")"))
    }
}

impl<T: Display> Stack<T> {
    #[inline]
    pub fn mk_string(
        &self,
        start: &'static str,
        separator: &'static str,
        end: &'static str,
    ) -> String {
        match self.head() {
            Some(value) => {
                let tail = self.tail();
                let tail_string = tail.mk_string_helper(separator);
                format!("{}{}{}{}", start, tail_string, value, end)
            }
            None => format!("{}{}", start, end),
        }
    }

    #[inline]
    fn mk_string_helper(&self, separator: &'static str) -> String {
        match self.head() {
            Some(value) => format!(
                "{}{}{}",
                self.tail().mk_string_helper(separator),
                value,
                separator,
            ),
            None => String::new(),
        }
    }
}

impl<T> Drop for Stack<T> {
    fn drop(&mut self) {
        let mut head = self.head.take();
        while let Some(node) = head {
            if let Ok(mut node) = Rc::try_unwrap(node) {
                head = node.next.take();
            } else {
                break;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn stack_abcd<'a>() -> Stack<&'a str> {
        Stack::empty()
            .append("a")
            .append("b")
            .append("c")
            .append("d")
    }

    #[test]
    fn basics() {
        let stack = Stack::empty();

        assert_eq!(stack.head(), None);

        let stack = stack.append("a").append("b").append("c");

        assert_eq!(stack.head(), Some(&"c"));

        let stack = stack.tail();
        assert_eq!(stack.head(), Some(&"b"));
        let stack = stack.tail();
        assert_eq!(stack.head(), Some(&"a"));
        let stack = stack.tail();
        assert_eq!(stack.head(), None);
    }

    #[test]
    fn correct_len() {
        let stack: Stack<&str> = Stack::empty();
        assert_eq!(0, stack.len());

        assert_eq!(2, stack.append("one").append("two").len())
    }

    #[test]
    fn empty_stack() {
        let stack: Stack<&str> = Stack::empty();
        assert_eq!("Stack()", format!("{}", stack.to_string()))
    }

    #[test]
    fn single_element_stack() {
        let stack = Stack::empty().append("hello");
        assert_eq!("Stack(hello)", format!("{}", stack.to_string()))
    }

    #[test]
    fn two_elements_stack() {
        let stack = Stack::empty().append("hello").append("bye");
        assert_eq!("Stack(hello, bye)", format!("{}", stack.to_string()))
    }

    #[test]
    fn shows_its_strings() {
        let stack = stack_abcd();
        assert_eq!("Stack(a, b, c, d)", format!("{}", stack.to_string()))
    }

    #[test]
    fn mk_string_shows_correct() {
        let stack = stack_abcd();
        assert_eq!(
            "[a, b, c, d]",
            format!("{}", stack.mk_string("[", ", ", "]"))
        )
    }
}
