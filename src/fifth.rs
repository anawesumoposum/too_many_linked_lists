//unsafe singly linked queue
use std::ptr;

pub struct List<T> {
    head: Link<T>,
    tail: *mut Node<T>, //prev node owns tail, so this must be a ref, also this is now a raw ptr
}   //raw pointers can be null, not using option here

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl <T> List<T> {
    pub fn new() -> Self {
        List { head: None, tail: ptr::null_mut() }
    }

    pub fn push_tail(&mut self, elem: T) {
        let mut new_tail = Box::new(Node {
            elem: elem,
            next: None,
        });
        let raw_tail: *mut _ = &mut *new_tail;  //raw ptr

        if !self.tail.is_null() {
            unsafe {
                (*self.tail).next = Some(new_tail);
            }
        } else {
            self.head = Some(new_tail);
        }
        self.tail = raw_tail;
    }

    pub fn pop_head(&mut self) -> Option<T> {
        self.head.take().map(|head| {
            let head = *head;
            self.head = head.next;
            if self.head.is_none() {
                self.tail = ptr::null_mut();
            }
            head.elem
        })
    }
}

pub struct IntoIter<T>(List<T>);

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop_head()
    }
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

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_deref_mut();
            &mut node.elem
        })
    }
}

impl<T> List<T> {
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

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter { next: self.head.as_deref() }
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut { next: self.head.as_deref_mut() }
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
        }
    }
}

#[cfg(test)]
mod test {
    use super::List;
    #[test]
    fn basics() {
        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop_head(), None);

        // Populate list
        list.push_tail(1);
        list.push_tail(2);
        list.push_tail(3);

        // Check normal removal
        assert_eq!(list.pop_head(), Some(1));
        assert_eq!(list.pop_head(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push_tail(4);
        list.push_tail(5);

        // Check normal removal
        assert_eq!(list.pop_head(), Some(3));
        assert_eq!(list.pop_head(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop_head(), Some(5));
        assert_eq!(list.pop_head(), None);

        // Check the exhaustion case fixed the pointer right
        list.push_tail(6);
        list.push_tail(7);

        // Check normal removal
        assert_eq!(list.pop_head(), Some(6));
        assert_eq!(list.pop_head(), Some(7));
        assert_eq!(list.pop_head(), None);
    }
}
