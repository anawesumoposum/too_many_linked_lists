#[derive(Debug)]
pub enum List0<T> {
    Empty,
    Elem(T, Box<List0<T>>)
}   //[Elem A, ptr] -> (Elem B, ptr) -> (Empty, *junk*)
//last Empty above is wasted space


#[derive(Debug)]
pub enum List1<T> {
    Empty,
    ElemThenEmpty(T),
    ElemthenNotEmpty(T, Box<List1<T>>)
}   //[ptr] -> (Elem A, ptr) -> (Elem B, *null*)
//invalid state: ElemThenNotEmpty(0, Box(Empty)), but it doesn't allocate the Empty case

pub mod bad {   //a bad stack
    use std::mem;

    #[derive(Debug)]
    pub struct List<T> {
        head: Link<T>,
    }   //Because List is a struct with a single field, size is the same as that field: zero cost abstractions
    
    #[derive(Debug)]
    enum Link<T> {
        Empty,
        More(Box<Node<T>>),
    }   //zero cost empty list with null ptr optimization
    //optimimization is: enums need space to store variant tag
    //if it's either null or non-null, it doesn't use that variant tag so we save space
    
    #[derive(Debug)]
    struct Node<T> {
        elem: T,
        next: Link<T>,
    }
    
    impl<T> List<T> {    //hiding details inside struct, so we provide it as a func
        
        pub fn new() -> Self {  //Self is alias for what's next to impl: List
            List { head: Link::Empty }
        } 

        pub fn push(&mut self, elem: T) {
            //cannot take out of a borrow without replacing
            let new_node = Box::new(Node {
                elem: elem,
                next: mem::replace(&mut self.head, Link::Empty),
            });
            self.head = Link::More(new_node);
        }

        pub fn pop(&mut self) -> Option<T> {
            match mem::replace(&mut self.head, Link::Empty) { //need to remove current head, so must replace it first
                Link::Empty => None,
                Link::More(node) => {
                    self.head = node.next;
                    Some(node.elem)
                }
            }
        }
    }
    //self can be self value, &mut self mut ref, or &self shared ref
    //shared ref can be made mutable in special cases
    impl<T> Drop for List<T> {
        fn drop(&mut self) {
            let mut cur = mem::replace(&mut self.head, Link::Empty);
            while let Link::More(mut boxed_node) = cur {
                cur = mem::replace(&mut boxed_node.next, Link::Empty);
                //boxed_node drops out of scope and next is set to empty
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::bad::List;
    #[test]
    fn basics() {
        let mut list = List::new();

        assert_eq!(list.pop(), None);

        list.push(0);
        list.push(1);

        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), Some(0));
        assert_eq!(list.pop(), None);
    }
}