//a good safe mutable singly linked stack that implements iterators

#[derive(Debug)]
pub struct List<T> {
    head: Link<T>,
}   //Because List is a struct with a single field, size is the same as that field: zero cost abstractions

type Link<T> = Option<Box<Node<T>>>; //type alias

#[derive(Debug)]
struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {    //hiding details inside struct, so we provide it as a func
    
    pub fn new() -> Self {  //Self is alias for what's next to impl: List
        List { head: None }
    } 

    pub fn push(&mut self, elem: T) {
        //cannot take out of a borrow without replacing
        let new_node = Box::new(Node {
            elem: elem,
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| {
            &node.elem
        })
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| {
            &mut node.elem
        })
    }
}

//IntoIter T
pub struct IntoIter<T>(List<T>); //tuple struct

impl<T> List<T> {
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()    //access tuple struct fields numerically
    }
}

//Iter &T 
pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<T> List<T> {
    pub fn iter<'a>(&'a self) -> Iter<'a, T> { //self must live as long as Iter //elision actually does this one for us, unecessary
        Iter { next: self.head.as_ref().map(|node| &**node) } //* deref to take out of box, extra deref since as_ref adds layer of ref indirection
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T; //type declarations usually need lifetimes
    //no lifetime needed here since it's handled above + self elision rule
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref(); //alternative to above manual derefs
            &node.elem
        })
    }
}

//IterMut &mut T
pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl<T> List<T> {
    pub fn iter_mut(&mut self) -> IterMut<'_, T> { //"explicitly elided lifetime" syntax to let us know what the compiler is doing for us
        IterMut { next: self.head.as_deref_mut() } 
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T; //type declarations usually need lifetimes
    //no lifetime needed here since it's handled above + self elision rule
    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {   //Option<&T> is copy, but Option<&mut T> isn't to prevent having two mut ptrs to same memory 
            self.next = node.next.as_deref_mut(); 
            &mut node.elem
        })
    }
}


impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur = self.head.take();
        while let Some(mut boxed_node) = cur {
            cur = boxed_node.next.take();
            //boxed_node drops out of scope and next is set to empty
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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

    #[test]
    fn peek() {
        let mut list = List::new();
        assert_eq!(list.peek(), None);
        assert_eq!(list.peek_mut(), None);
        list.push(1); 
        list.push(2);
        assert_eq!(list.peek(), Some(&2));
        assert_eq!(list.peek_mut(), Some(&mut 2));
        list.peek_mut().map(|value| {
            *value = 42
        });
        assert_eq!(list.peek(), Some(&42));
        assert_eq!(list.peek_mut(), Some(&mut 42));
    }

    #[test]
    fn into_iter() {
        let mut list = List::new();
        list.push(0); list.push(1);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(0));
        assert_eq!(iter.next(), None);
    }

    #[test] 
    fn iter() {
        let mut list = List::new();
        list.push(0); list.push(1);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&0));
        assert_eq!(iter.next(), None);
    }   

    #[test] 
    fn iter_mut() {
        let mut list = List::new();
        list.push(0); list.push(1);

        let mut iter = list.iter_mut();
        assert_eq!(iter.next(), Some(&mut 1));
        assert_eq!(iter.next(), Some(&mut 0));
        assert_eq!(iter.next(), None);
    }
}