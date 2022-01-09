//a good persistent singly linked stack for shared ownership
/*  desired behavior below
list1 = A -> B -> C -> D
list2 = tail(list1) = B -> C -> D
list3 = push(list2, X) = X -> B -> C -> D

list1 -> A ---+
              |
              v
list2 ------> B -> C -> D
              ^
              |
list3 -> X ---+
free in gc langauges, but speed >> laziness in Rust so we'll do some ref counting 
*/  
use std::rc::Rc;
//in this case we can safely replace Rc with Arc if we really wanted to

pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Rc<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn prepend(&self, elem: T) -> List<T> {
        List {
            head: Some(Rc::new(Node {
                elem: elem,
                next: self.head.clone(),    //Rc++
            }))
        }
    }

    pub fn tail(&self) -> List<T> {
        List { head: self.head.as_ref().and_then(|node| node.next.clone()) } //Rc++
    }   
    //and_then: this returns Option<Option<RC<T>>>, and_then strips out an Option

    pub fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }
}

//Iter &T
pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
} 

impl<T> List<T> {
    pub fn iter(&self) -> Iter<'_, T> {
        Iter { next: self.head.as_deref() } //as_deref pulls value out
    }                                       //as_ref adds ref to value
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.elem
        })
    }
}

//drop
impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur = self.head.take();
        while let Some(rc_node) = cur {
            if let Ok(mut rc_node) = Rc::try_unwrap(rc_node) {  //is there only one ref to it?
                cur = rc_node.next.take();
                //rc_node falls out of scope
            } else {
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::List;

    #[test]
    fn basics() {
        let list = List::new();
        assert_eq!(list.head(), None);

        let list = list.prepend(1).prepend(2).prepend(3);
        assert_eq!(list.head(), Some(&3));

        let list = list.tail();
        assert_eq!(list.head(), Some(&2));

        let list = list.tail();
        assert_eq!(list.head(), Some(&1));

        let list = list.tail();
        assert_eq!(list.head(), None);

        // Make sure empty tail works
        let list = list.tail();
        assert_eq!(list.head(), None);
    }

    #[test]
    fn iter() {
        let list = List::new().prepend(0).prepend(1);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&0));
        assert_eq!(iter.next(), None);
    }
}