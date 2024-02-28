use std::vec;

struct Countdown {
    remaining: i32,
}

impl Iterator for Countdown {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            None
        } else {
            self.remaining -= 1;
            Some(self.remaining + 1)
        }
    }

}



struct Fibonacci {
    curr: u32,
    next: u32,
}

// Implement 'Iterator' for 'Fibonacci'

impl Iterator for Fibonacci {
    
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.curr;
        self.curr = self.next;
        self.next = self.curr + current;
        Some(current)
    }

}



fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    let mut iter = numbers.iter();

    assert_eq!(iter.next(), Some(&1));
    assert_eq!(iter.next(), Some(&2));
    assert_eq!(iter.next(), Some(&3));
    assert_eq!(iter.next(), Some(&4));
    assert_eq!(iter.next(), Some(&5));
    assert_eq!(iter.next(), None);

    let mut fib = Fibonacci { curr: 1, next: 1 };

    for _ in 0..10 {
        println!("{}", fib.next().unwrap());
    }

    let mut vec = vec![1, 2, 3];

    for item in vec.iter_mut() {
        *item += 2;
    }

    
    for item in vec.iter() {
        println!("{}", item);
    }

    let vec = vec![1, 2, 3];

    let mut v_iter = vec.into_iter();

    for item in v_iter.by_ref() {
        println!("{}", item);
    }

    println!("{:?}", &v_iter);

    let countdown = Countdown { remaining: 3 };

    for i in countdown {
        println!("{}", i);
    }

}
