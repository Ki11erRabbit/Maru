use std::{collections::VecDeque, sync::{Mutex, OnceLock}, time::SystemTime};

use refcounter::RefCounter;

use crate::vm::{Metadata, StackFrame, StackFrameCore, TypeSymbol, VariantId, allocator, tables::ObjectDescTable};

#[derive(Debug)]
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

unsafe impl Send for AllocationGroup {}
unsafe impl Sync for AllocationGroup {}

#[derive(Debug)]
pub struct Allocator {
    memory_pool: Vec<AllocationGroup>
}


impl Allocator {
    pub fn new(max_type_symbol: TypeSymbol) -> Self {
        Allocator { memory_pool: vec![AllocationGroup::new(); max_type_symbol as usize] }
    }

    pub fn allocate<T>(&mut self, symbol: TypeSymbol, variant: VariantId, desc_table: &ObjectDescTable) -> *mut T {
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

        {
            let meta = output as *mut Metadata;
            unsafe {
                (*meta).refcount = RefCounter::new();
                (*meta).type_id = symbol;
                (*meta).variant_id = variant;
            }
        }

        output as *mut T
    }

    pub fn reuse_memory<T>(&mut self, memory: *mut T) {
        let type_symbol = unsafe { (*(memory as *mut Metadata)).type_id };
        self.memory_pool[type_symbol as usize].push_front(memory);
    }

    pub fn deallocate<T>(memory: *mut T, desc_table: &ObjectDescTable) {
        let type_symbol = unsafe { (*(memory as *mut Metadata)).type_id };
        let desc = &desc_table[type_symbol];
        let layout = desc.layout;

        unsafe {
            std::alloc::dealloc(memory as *mut u8, layout);
        }
    }

    pub fn allocate_stack_frame(
        &mut self, 
        desc_table: &ObjectDescTable, 
        variable_size: usize
    ) -> *mut StackFrame {
        let frame = self.allocate(0, 0, desc_table) as *mut StackFrame;
        {
            let frame = unsafe { &mut *frame };
            frame.metadata = Metadata { refcount: RefCounter::new(), type_id: 0, variant_id: 0 };
            frame.core = StackFrameCore::new(variable_size);
        }

        frame
    }

    pub fn reuse_stack_frame_memory(&mut self, frame: *mut StackFrame) {
        {
            let frame = unsafe { &mut *frame };
            frame.core.free_memory();
        }
        self.reuse_memory(frame);
    }

    pub fn deallocate_stack_frame(desc_table: &ObjectDescTable, frame: *mut StackFrame) {
        {
            let frame = unsafe { &mut *frame };
            frame.core.free_memory();
        }
        Self::deallocate(frame, desc_table);
    }

    pub fn allocator_init(max_type_symbol: TypeSymbol) {
        ALLOCATOR.set(Mutex::new(Allocator::new(max_type_symbol)))
            .expect("allocator_init should only be called once");
    }

    fn get_allocator() -> &'static Mutex<Allocator> {
        ALLOCATOR.get()
            .expect("Allocator was not initialized")
    }

    pub fn create<T>(symbol: TypeSymbol, variant: VariantId) -> *mut T {
        let allocator = Self::get_allocator();
        let mut allocator = allocator.lock().expect("Allocator Poisoned");
        let desc_table = ObjectDescTable::get();
        allocator.allocate(symbol, variant, desc_table)
    }

    pub fn reuse<T>(memory: *mut T) {
        let allocator = Self::get_allocator();
        let mut allocator = allocator.lock().expect("Allocator Poisoned");
        allocator.reuse_memory(memory);
    }

    pub fn destroy<T>(desc_table: &ObjectDescTable, memory: *mut T) {
        Self::deallocate(memory, desc_table);
    }

    pub fn create_stack_frame(desc_table: &ObjectDescTable, variable_size: usize) -> *mut StackFrame {
        let allocator = Self::get_allocator();
        let mut allocator = allocator.lock().expect("Allocator Poisoned");
        allocator.allocate_stack_frame(desc_table, variable_size)
    }

    pub fn reuse_stack_frame(frame: *mut StackFrame) {
        let allocator = Self::get_allocator();
        let mut allocator = allocator.lock().expect("Allocator Poisoned");
        allocator.reuse_stack_frame_memory(frame);
    }

    pub fn destroy_stack_frame(desc_table: &ObjectDescTable, frame: *mut StackFrame) {
        Self::deallocate_stack_frame(desc_table, frame);
    }
}


static ALLOCATOR: OnceLock<Mutex<Allocator>> = OnceLock::new();