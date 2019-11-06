use std::rc::Rc;

#[derive(Debug, PartialEq)]
pub enum StringStack {
    Empty,
    Cons(&'static str, Rc<StringStack>),
}

impl StringStack {
    pub fn empty() -> StringStack {
        StringStack::Empty
    }
    pub fn is_empty(&self) -> bool {
        match *self {
            StringStack::Cons(_, _) => false,
            StringStack::Empty => true,
        }
    }
    pub fn head(&self) -> Option<&'static str> {
        match self {
            StringStack::Empty => None,
            StringStack::Cons(data, _) => Some(data),
        }
    }
    pub fn push(&self, data: &'static str) -> StringStack {
        match self {
            StringStack::Empty => StringStack::Cons(data, Rc::new(StringStack::Empty)),
            StringStack::Cons(b, next) => {
                StringStack::Cons(data, Rc::new(StringStack::Cons(b, next.clone())))
            }
        }
    }
    pub fn tail(&self) -> Option<&StringStack> {
        match self {
            StringStack::Empty => None,
            StringStack::Cons(_, next) => Some(next.as_ref()),
        }
    }

    pub fn to_string(&self) -> String {
        self.mk_string("Stack(", ", ", ")")
    }

    pub fn mk_string(
        &self,
        start: &'static str,
        separator: &'static str,
        end: &'static str,
    ) -> String {
        format!("{}{}", start, self.mk_string_helper(separator, end))
    }

    fn mk_string_helper(&self, separator: &'static str, end: &'static str) -> String {
        match self {
            StringStack::Empty => format!("{}", end),
            StringStack::Cons(data, next) => {
                let stack = next.as_ref();
                match stack {
                    StringStack::Empty => format!("{}{}", data, end),
                    StringStack::Cons(_, _) => format!(
                        "{}{}{}",
                        data,
                        separator,
                        next.as_ref().mk_string_helper(separator, end)
                    ),
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn stack_abc() -> StringStack {
        StringStack::empty().push("c").push("b").push("a")
    }

    #[test]
    fn sharing_with_immutable_cons_compiles() {
        let stack = stack_abc();
        let _x = stack.push("100");
        let _y = stack.push("200");
    }

    #[test]
    fn shows_its_strings() {
        let stack = stack_abc();
        assert_eq!("Stack(a, b, c)", format!("{}", stack.to_string()))
    }

    #[test]
    fn mk_string_shows_correct() {
        let stack = stack_abc();
        assert_eq!("a, b, c", format!("{}", stack.mk_string("", ", ", "")))
    }
}
