use std::mem;

pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, elem: T) {
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
  
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
    
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_link = mem::replace(&mut self.head, None);

        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
        }
    }
}

pub struct IntoIter<T>(List<T>);
impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        // access fields of a tuple struct numerically
        self.0.pop()
    }
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<T> List<T> {
    pub fn iter<'a>(&'a self) -> Iter<'a, T> {
        Iter { next: self.head.as_ref().map(|node| &**node) }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_ref().map(|node| &**node);
            &node.elem
        })
    }
}

pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

//https://rust-unofficial.github.io/too-many-lists/second-iter-mut.html
//IterMut之所以可以工作， 原文给出了2个理由：
//1. 通过take确实保证了&mut型引用的唯一排他性。
//2. https://doc.rust-lang.org/nomicon/borrow-splitting.html
impl<T> List<T> {
    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        //this works coz: Rust understands that it's ok to shard a mutable reference into the subfields of the pointed-to struct, 
        //because there's no way to "go back up", and they're definitely disjoint.
        IterMut { next: self.head.as_mut().map(|node| &mut **node) }
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            //this works coz: Rust understands that it's ok to shard a mutable reference into the subfields of the pointed-to struct,
            // because there's no way to "go back up", and they're definitely disjoint.
            //We take the Option<&mut> so we have exclusive access to the mutable reference. 
            //No need to worry about someone looking at it again.
            self.next = node.next.as_mut().map(|node| &mut **node);
            &mut node.elem
        })
    }
}