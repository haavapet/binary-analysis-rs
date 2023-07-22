use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::sync::{Arc, Mutex};

#[derive(Copy, Clone)]
pub struct Candidate {
    pub probability: f64,
    pub call_opcode: u64,
    pub ret_opcode: u64,
}

impl PartialEq for Candidate {
    fn eq(&self, other: &Self) -> bool {
        self.probability == other.probability
    }
}

impl Eq for Candidate {}

impl PartialOrd for Candidate {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other)) // Delegate to the implementation in `Ord`.
    }
}

impl Ord for Candidate {
    fn cmp(&self, other: &Self) -> Ordering {
        Reverse(self.probability)
            .partial_cmp(&Reverse(other.probability))
            .unwrap()
    }
}

pub struct MinHeap {
    top_candidates: Arc<Mutex<BinaryHeap<Candidate>>>,
}

impl Default for MinHeap {
    fn default() -> Self {
        MinHeap {
            top_candidates: Arc::new(Mutex::new(Default::default())),
        }
    }
}

impl MinHeap {
    pub fn add_maybe(&self, nr_cand: usize, candidate: Candidate) {
        let mut heap = self.top_candidates.lock().unwrap();

        heap.push(candidate);

        if heap.len() > nr_cand {
            heap.pop();
        }
    }

    pub fn get_result(&mut self) -> Vec<Candidate> {
        self.top_candidates
            .lock()
            .unwrap()
            .clone()
            .into_sorted_vec()
    }
}
