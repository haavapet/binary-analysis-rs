use ordered_float::NotNan;
use std::sync::{Arc, Mutex};
use std::{cmp::Reverse, collections::BinaryHeap};

pub struct MinHeap {
    top_candidates: Arc<Mutex<BinaryHeap<Reverse<NotNan<f64>>>>>,
}

impl Default for MinHeap {
    fn default() -> Self {
        MinHeap {
            top_candidates: Arc::new(Mutex::new(Default::default())),
        }
    }
}

impl MinHeap {
    pub fn add_maybe(&self, nr_cand: usize, probability: f64) {
        let mut heap = self.top_candidates.lock().unwrap();

        heap.push(Reverse(NotNan::new(probability).unwrap()));

        if heap.len() > nr_cand {
            heap.pop();
        }
    }

    pub fn get_result(&mut self) -> Vec<f64> {
        self.top_candidates
            .lock()
            .unwrap()
            .clone()
            .into_sorted_vec()
            .iter()
            .map(|x| f64::from(x.0))
            .collect()
    }
}
