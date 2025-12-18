use std::{collections::VecDeque, time::SystemTime};

use crate::vm::{Metadata, TypeSymbol, tables::ObjectDescTable};


struct AllocationGroup {
    objects: VecDeque<*mut ()>,
}

impl AllocationGroup {
    pub fn new() -> Self {
        Self {
            objects: VecDeque::new(),
        }
    }

    pub fn push_front<T>(&mut self, memory: *mut T) {
        self.objects.push_front(memory as *mut ());
    }

    pub fn pop_front<T>(&mut self) -> Option<*mut T> {
        self.objects.pop_front().map(|mem| mem as *mut T)
    }
}

impl Clone for AllocationGroup {
    fn clone(&self) -> Self {
        AllocationGroup { objects: VecDeque::new() }
    }
}

pub struct Allocator {
    memory_pool: Vec<AllocationGroup>
}


impl Allocator {
    pub fn new(max_type_symbol: TypeSymbol) -> Self {
        Allocator { memory_pool: vec![AllocationGroup::new(); max_type_symbol as usize] }
    }

    pub fn allocate<T>(&mut self, symbol: TypeSymbol, desc_table: &ObjectDescTable) -> *mut T {
        let output = if let Some(prev) = self.memory_pool[symbol as usize].pop_front::<T>() {
            prev
        } else {
            let description = &desc_table[symbol];
            let layout = description.layout;

            let mem = unsafe { std::alloc::alloc(layout) };
            if mem.is_null() {
                std::alloc::handle_alloc_error(layout);
            }
            mem as *mut T
        };

        output as *mut T
    }

    pub fn deallocate<T>(&mut self, memory: *mut T) {
        let type_symbol = unsafe { (*(memory as *mut Metadata)).type_id };
        self.memory_pool[type_symbol as usize].push_front(memory);
    }

    pub fn destroy<T>(&self, memory: *mut T, desc_table: &ObjectDescTable) {
        let type_symbol = unsafe { (*(memory as *mut Metadata)).type_id };
        let desc = &desc_table[type_symbol];
        let layout = desc.layout;

        unsafe {
            std::alloc::dealloc(memory as *mut u8, layout);
        }
    }
}