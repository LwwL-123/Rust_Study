use crate::Coin::Penny;

fn largest<T: std::cmp::PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_coin(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {1}
        Coin::Nickel => {2}
        Coin::Dime => {3}
        Coin::Quarter => {4}
    }
}

fn main() {
    let vec1 = vec![30, 40, 50];
    println!("{:?}", largest(&vec1));

    let p = Point { x: 5, y: 6 };
    println!("{}", p.x);

    println!("{}", value_in_coin(Penny));
}

