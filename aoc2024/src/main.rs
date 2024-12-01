use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::fs::read_to_string;

fn main() {
    println!("Hello, world!");

    let mut list1 = BinaryHeap::new();
    let mut list2 = BinaryHeap::new();

    for line in read_to_string("input.txt").unwrap().lines() {
        println!("{}", line);
        let mut parts = line.split_whitespace().map(|s| s.parse::<i32>());
        let first = parts.next().unwrap().unwrap();
        let second = parts.next().unwrap().unwrap();
        println!("parts: {:?} {:?}", first, second);

        list1.push(Reverse(first));
        list2.push(Reverse(second));
    }

    let part1 = false;
    if part1
    {
        let mut total_distance = 0;

        while !list1.is_empty() {
            let first = list1.pop().unwrap().0;
            let second = list2.pop().unwrap().0;

            println!("ordered parts: {:?} {:?}", first, second);
            total_distance += (first - second).abs();
        }
        println!("total distance: {}", total_distance);
    }
    else {
        let mut similarity_score = 0;

        let mut last = 0;
        let mut times = 0;
        while !list1.is_empty() {
            let first = list1.pop().unwrap().0;
            if first == last {
                similarity_score += first * times;
            } else {
                last = first;
                times = 0;
                while !list2.is_empty() {
                    let second = list2.peek().unwrap().0;
                    if second <= first {
                        list2.pop();
                        if second == first {
                            times += 1;
                        }
                    } else {
                        break;
                    }
                }
                similarity_score += first * times;
            }
        }
        println!("similarity_score: {}", similarity_score);
    }}
