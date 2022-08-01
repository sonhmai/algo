/*
Problem: https://leetcode.com/problems/top-k-frequent-elements/
 */
use std::collections::HashMap;
use std::collections::BinaryHeap;
use std::cmp::Ordering;

#[derive(PartialEq, Eq, Debug)]
struct Element {
    num: i32,
    count: i32,
}

impl Ord for Element {
    fn cmp(&self, other: &Self) -> Ordering {
        self.count.cmp(&other.count)
    }
}

impl PartialOrd for Element {
    fn partial_cmp(&self, other: &Element) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {

        // 1. create hashmap num->count from vec ====
        let mut counts = HashMap::new();
        for num in &nums {
            *counts.entry(*num).or_insert(0) += 1;
        }
        // println!("{:?}", counts);

        // 2. add to the heap ====
        let mut heap = BinaryHeap::<Element>::new();
        for (&n, &c) in counts.iter() {
            heap.push(Element{num: n, count: c})
        }
        // println!("{:?}", heap);

        // 3. pick top k most-frequent elements ====
        let mut tops = Vec::<i32>::new();
        for _ in 0..k {
            tops.push(heap.pop().unwrap().num); // guaranteed that k is smaller than the range
        }

        tops
    }
}

/*

top k frequent elements
-Given an integer array nums and an integer k, return the k most frequent elements. You may return the answer in any order.


Input: nums = [1,1,1,2,2,3], k = 2
Output: [1,2]

hashmap
	count O(n) {1: 3, 2: 2, 3: 1}
	loop thru the hashmap O(n) , pick the k most frequent elements pushed to the result Vec
		need to compare keep what are the last 2 most frequent count

heap
	k most frequent -> max heap based on count
	need to have the count before pushing to the heap, the heap is sorted by the count
		we need sth like a struct {count: int32, number: int32}

	count O(n) {1: 3, 2: 2, 3: 1}
	add to the heap O(n)
	pick k most frequent from the heap O(k)
	=> total O(n)

	how to push the struct (count + number) to the heap instead of only the count?

		impl Ord for Item {
		    fn cmp(&self, other: &Self) -> Ordering {
		        self.offset.cmp(&other.offset).reverse()
		    }
		}

*/

// TODO - add tests
