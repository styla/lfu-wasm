#![no_std]
#![feature(core_intrinsics, lang_items)]

extern crate alloc;
extern crate wee_alloc;

use alloc::vec::Vec;
use hashbrown::HashMap;

use wasm_bindgen;
use wasm_bindgen::prelude::*;

use core::iter::repeat;
use core::hash::Hash;
use alloc::string::String;
use core::convert::TryInto;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

struct LfuInner<T> {
    heap: Vec<(T, usize)>,
    indices: HashMap<T, usize>,

    max_size: usize,
    num_items: usize,
}

impl<T: Clone + Hash + Eq + Ord> LfuInner<T> {
    fn new(capacity: usize) -> LfuInner<T> {
        LfuInner {
            heap: Vec::with_capacity(capacity as usize),
            indices: HashMap::new(),

            max_size: capacity,
            num_items: 0,
        }
    }

    pub fn refer(&mut self, val: T) -> Option<T> {
        let index_item =
            self.indices
                .get_mut(&val);

        if index_item.is_none() {
            return self.insert(&val);
        }

        let index_item = *index_item.unwrap();

        self.increment(
            index_item,
        );

        None
    }

    fn insert(&mut self, val: &T) -> Option<T> {
        let mut evicted_item: Option<T> = None;

        if self.num_items == self.max_size {
            let heap_item =
                self.heap
                    .get_mut(0)
                    .unwrap();

            self.indices
                .remove(
                    &heap_item.0,
                );

            evicted_item = Some(heap_item.0.clone());

            self.num_items -= 1;

            self.heap
                .swap(
                    0,
                    self.num_items as usize,
                );

            self.heapify(0);
        }

        self.num_items += 1;

        self.heap.push(
            (val.clone(), 1),
        );

        self.indices
            .insert(
                val.clone(),
                self.num_items - 1,
            );

        let mut i = self.num_items - 1;

        while i != 0 {
            let parent = (i - 1) >> 1;

            if self.heap[parent].1
                <= self.heap[i].1 {
                break;
            }

            self.indices
                .insert(
                    self
                        .heap[i as usize]
                        .0
                        .clone(),
                    parent,
                );

            self.indices
                .insert(
                    self
                        .heap[parent as usize]
                        .0
                        .clone(),
                    i,
                );

            self.heap.swap(
                i,
                parent,
            );

            i = parent;
        }

        evicted_item
    }

    fn increment(&mut self, index: usize) {
        self.heap
            .get_mut(
                index,
            )
            .unwrap()
            .1 += 1;

        self.heapify(index);
    }

    fn heapify(&mut self, index: usize) {
        let left = 2 * index + 1;
        let right = 2 * index + 2;

        let mut minim: usize;

        if left < self.num_items {
            minim = {
                if self.heap[index].1
                    < self.heap[left].1 {
                    index
                } else {
                    left
                }
            }
        } else {
            minim = index;
        }

        if right < self.num_items {
            minim =
                if self.heap[minim].1
                    < self.heap[right].1 {
                    minim
                } else {
                    right
                }
        }

        if minim != index {
            self.indices.insert(
                self.heap[minim].0.clone(),
                index,
            );

            self.indices.insert(
                self.heap[index].0.clone(),
                minim,
            );

            self.heap.swap(
                minim,
                index,
            );

            self.heapify(minim);
        }
    }
}

#[wasm_bindgen]
struct Lfu {
    lfu: LfuInner<String>,
}

#[wasm_bindgen]
impl Lfu {
    #[wasm_bindgen(constructor)]
    pub fn new(capacity: u32) -> Lfu {
        Lfu {
            lfu: LfuInner::new(capacity.try_into().unwrap()),
        }
    }

    pub fn refer(&mut self, val: String) -> Option<String> {
        self.lfu.refer(val.into())
    }
}
