#![allow(unused_variables)]

struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    // Attention: We have `self` and `other: Point<V, W>`
    // as parameter types, which means that we are moving
    // the ownership of `self` and `other`.
    fn mixup<V, W> (self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y
        }
    }
}

fn main() {
    let p1: Point<u8, u8> = Point {
        x: 1,
        y: 2,
    };

    let p2: Point<u8, u8> = Point {
        x: 3,
        y: 4,
    };

    let p3 = p1.mixup(p2);
    // We have moved ownership of `P1` and `P2`
    // to P3!
    // This line won't compile:
    // println!("{}, {}", P1.x, P2.y)
}
