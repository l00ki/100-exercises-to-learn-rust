// TODO: Given a vector of integers, leak its heap allocation.
//  Then split the resulting static slice into two halves and
//  sum each half in a separate thread.
//  Hint: check out `Vec::leak`.

use std::thread;

pub fn sum(v: Vec<i32>) -> i32 {
    let slice: &'static [i32] = Vec::leak(v);

    let n = slice.len();

    let slices = vec![&slice[..n/2], &slice[n/2..]];

    let mut threads = vec![];
    for s in slices {
        threads.push(
            thread::spawn(
                || -> i32 {
                    s.into_iter().sum::<i32>()
                }
            )
        )
    }

    threads.into_iter().map(|t| t.join().unwrap()).sum::<i32>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(sum(vec![]), 0);
    }

    #[test]
    fn one() {
        assert_eq!(sum(vec![1]), 1);
    }

    #[test]
    fn five() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5]), 15);
    }

    #[test]
    fn nine() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]), 45);
    }

    #[test]
    fn ten() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 55);
    }
}
