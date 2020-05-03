#[derive(Debug, PartialEq, Eq)]
pub enum List {
    // Cons: Tuple struct that wraps an element and a pointer to the next node
    Nodes(u32, Box<List>),
    // Nil: A node that signifies the end of the linked list
    Nil,
}

// Methods can be attached to an enum
impl List {
    // Create an empty list
    fn new() -> List {
        // `Nil` has type `List`
        Self::Nil
    }

    // Consume a list, and return the same list with a new element at its front
    fn push_front(self, elem: u32) -> List {
        // `Node` also has type List
        Self::Nodes(elem, Box::new(self))
    }
    
    // Return the length of the list
    fn len(&self) -> u32 {
        // `self` has to be matched, because the behavior of this method
        // depends on the variant of `self`
        // `self` has type `&List`, and `*self` has type `List`, matching on a
        // concrete type `T` is preferred over a match on a reference `&T`
        match *self {
            // Can't take ownership of the tail, because `self` is borrowed;
            // instead take a reference to the tail
            Self::Nodes(_, ref tail) => 1 + tail.len(),
            // Base Case: An empty list has zero length
            Self::Nil => 0
        }
    }

 // Return representation of the list as a (heap allocated) string
    fn stringify(&self) -> String {
        match *self {
            Self::Nodes(head, ref tail) => {
                // `format!` is similar to `print!`, but returns a heap
                // allocated string instead of printing to the console
                format!("{}, {}", head, tail.stringify())
            },
            Self::Nil => {
                format!("Nil")
            },
        }
    }
}

fn main() {
    // Create an empty linked list
    let mut list = List::new();

    // Prepend some elements
    list = list.push_front(1);
    list = list.push_front(2);
    list = list.push_front(3);

    // Show the final state of the list
    println!("linked list has length: {}", list.len());
    println!("{}", list.stringify());
}