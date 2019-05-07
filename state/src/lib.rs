use std::cell::RefCell;
use std::rc::Rc;

pub mod backends;

pub trait StateBackend: 'static {
    fn new() -> Self;
    fn store_count(&mut self, name: &str, count: u64);
    fn get_count(&self, name: &str) -> u64;
}

pub struct StateHandle<T: StateBackend> {
    backend: Rc<RefCell<T>>,
    name: String,
}

pub struct ManagedCount<T: StateBackend> {
    backend: Rc<RefCell<T>>,
    name: String,
    count: u64,
}

impl<T: StateBackend> StateHandle<T> {
    pub fn new(backend: Rc<RefCell<T>>, name: &str) -> Self {
        StateHandle {
            backend,
            name: name.to_owned(),
        }
    }

    pub fn get_managed_count(&self, name: &str) -> ManagedCount<T> {
        ManagedCount {
            backend: self.backend.clone(),
            name: name.to_owned(),
            count: self.backend.borrow().get_count(name),
        }
    }
}

impl<T: StateBackend> ManagedCount<T> {
    pub fn incr(&mut self, amount: u64) {
        self.count += amount;
        self.backend
            .borrow_mut()
            .store_count(&self.name, self.count);
    }

    pub fn get(&self) -> u64 {
        self.count
    }
}
