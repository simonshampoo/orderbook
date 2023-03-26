

enum Color {
    Red, 
    Black
}

struct Node<T> {
    value: T, 
    color: Color, 
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn new(value: T, color: Color) -> Self {
        Self {
            value, 
            color,
            left: None, 
            right: None
        }
    }

    fn is_red(&self) -> bool {
        self.color == Color::Red
    }

    fn is_black(&self) -> bool {
        self.color == Color::Black
    }

    fn flip_coor(&self) -> bool {
        self.color == match self.color () {
            Color::Red => Color::Black, 
            Color::Black => Color::Red
        }
    }

}

struct RBTree<T> {
    root: Option<Box<Node<T>>>
}

impl<T: Ord> RBTree<T> {
    fn new() -> Self {
        Self { root: None }
    }
}

