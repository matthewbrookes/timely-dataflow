use crate::StateBackend;
use faster_rs::FasterValue;
use std::cell::RefCell;
use std::rc::Rc;

pub struct ManagedValue<S: StateBackend> {
    backend: Rc<RefCell<S>>,
    name: String,
}

impl<S: StateBackend> ManagedValue<S> {
    pub fn new(backend: Rc<RefCell<S>>, name: &str) -> Self {
        ManagedValue {
            backend,
            name: name.to_owned(),
        }
    }

    pub fn set<T: 'static + FasterValue>(&self, value: T) {
        self.backend.borrow_mut().store_value(&self.name, value);
    }

    pub fn get<T: 'static + FasterValue>(&self) -> Option<T> {
        self.backend.borrow_mut().get_value(&self.name)
    }
}

#[cfg(test)]
mod tests {
    use crate::backends::InMemoryBackend;
    use crate::primitives::ManagedValue;
    use crate::{StateBackend, StateBackendInfo};
    use std::cell::RefCell;
    use std::rc::Rc;

    const STATE_BACKEND_INFO: StateBackendInfo = StateBackendInfo {};

    #[test]
    fn new_value_returns_none() {
        let backend = Rc::new(RefCell::new(InMemoryBackend::new(&STATE_BACKEND_INFO)));
        let managed_value = ManagedValue::new(backend.clone(), "value");

        assert!(managed_value.get::<String>().is_none());
    }

    #[test]
    fn set_get_value_returns_some() {
        let backend = Rc::new(RefCell::new(InMemoryBackend::new(&STATE_BACKEND_INFO)));
        let managed_value = ManagedValue::new(backend.clone(), "value");

        managed_value.set("hello".to_owned());
        assert!(managed_value.get::<String>().is_some());
    }

    #[test]
    fn set_get_value_returns_value() {
        let backend = Rc::new(RefCell::new(InMemoryBackend::new(&STATE_BACKEND_INFO)));
        let managed_value = ManagedValue::new(backend.clone(), "value");

        let value = "hello".to_owned();

        managed_value.set(value.clone());
        assert_eq!(managed_value.get::<String>(), Some(value));
    }

    #[test]
    fn set_get_get_value_returns_none() {
        let backend = Rc::new(RefCell::new(InMemoryBackend::new(&STATE_BACKEND_INFO)));
        let managed_value = ManagedValue::new(backend.clone(), "value");

        managed_value.set("hello".to_owned());
        managed_value.get::<String>();
        assert!(managed_value.get::<String>().is_none());
    }

    #[test]
    fn set_get_set_get_different_types_allowed() {
        let backend = Rc::new(RefCell::new(InMemoryBackend::new(&STATE_BACKEND_INFO)));
        let managed_value = ManagedValue::new(backend.clone(), "value");

        managed_value.set("hello".to_owned());
        managed_value.get::<String>();

        let value: i32 = 42;
        managed_value.set(value);
        assert_eq!(managed_value.get::<i32>(), Some(value));
    }
}