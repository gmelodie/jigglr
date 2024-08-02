use anyhow::{anyhow, Result};
use core::{cell::Cell, mem::MaybeUninit};
use std::sync::atomic::{AtomicIsize, Ordering};
// use std::alloc::Box;

// Queue ideas stolen from swap-buffer-queue

pub struct Queue<T> {
    items: Box<[Cell<MaybeUninit<T>>]>,
    size: usize,
    next_push_idx: AtomicIsize,
}

impl<T> Queue<T> {
    pub fn new(size: usize) -> Self {
        let items = (0..size)
            .map(|_| Cell::new(MaybeUninit::uninit()))
            .collect();
        Queue {
            items,
            size,
            next_push_idx: AtomicIsize::new(0),
        }
    }

    pub fn push(&self, elem: T) -> Result<()> {
        let push_idx: usize = self
            .next_push_idx
            .fetch_add(1, Ordering::SeqCst)
            .try_into()?;
        println!("push_idx: {push_idx}");
        if push_idx == self.size {
            // queue full
            self.next_push_idx.fetch_sub(1, Ordering::SeqCst); // revert the add
            Err(anyhow!("Queue is full"))
        } else {
            self.items[push_idx].set(MaybeUninit::new(elem));
            Ok(())
        }
    }

    unsafe fn pop_at_index(&mut self, index: usize) -> T {
        self.items[index].get_mut().assume_init_read()
    }

    pub fn pop(&mut self) -> Option<T> {
        let pop_idx = self.next_push_idx.fetch_sub(1, Ordering::SeqCst) - 1;

        if pop_idx < 0 {
            // queue empty
            self.next_push_idx.fetch_add(1, Ordering::SeqCst); // revert the change
            None
        } else {
            let pop_idx: usize = pop_idx.try_into().ok()?;
            unsafe { Some(self.pop_at_index(pop_idx)) }
        }
    }
}
