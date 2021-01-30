use std::fmt;
use std::num::Wrapping;

pub struct State {
    pointer: i32,
    memory: Vec<Wrapping<i32>>,
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "<pointer address: {}, value: {}, length: {}>",
            self.get_pointer(),
            self.get_value(),
            self.memory.len()
        )
    }
}

impl State {
    pub fn new() -> State {
        let mut state = State {
            pointer: 0,
            memory: Vec::with_capacity(30000),
        };
        state.memory.push(Wrapping(0));
        return state;
    }

    fn add_to_pointer(&mut self, value: i32) {
        self.pointer = self.pointer + value;
        if self.pointer < 0 {
            panic!("tried to set the pointer to negative address");
        }
        while self.pointer as usize >= self.memory.len() {
            self.memory.push(Wrapping(0));
        }
    }

    pub fn increment_pointer(&mut self) {
        self.add_to_pointer(1)
    }

    pub fn decrement_pointer(&mut self) {
        self.add_to_pointer(-1)
    }

    pub fn set_value(&mut self, value: i32) {
        self.memory[self.pointer as usize] = Wrapping(value);
    }

    pub fn get_value(&self) -> i32 {
        self.memory[self.pointer as usize].0
    }

    fn get_pointer(&self) -> i32 {
        self.pointer
    }

    fn add_to_value(&mut self, value: i32) {
        self.memory[self.pointer as usize] += Wrapping(value);
    }

    pub fn increment_value(&mut self) {
        self.add_to_value(1)
    }

    pub fn decrement_value(&mut self) {
        self.add_to_value(-1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn memory_growth() {
        let mut state = State::new();
        assert_eq!(state.memory.len(), 1);
        state.increment_pointer();
        assert_eq!(state.memory.len(), 2);
        state.add_to_pointer(10);
        assert_eq!(state.memory.len(), 12);
        state.add_to_pointer(10);
        state.add_to_pointer(-10);
        assert_eq!(state.memory.len(), 22);
    }

    #[test]
    fn persistence() {
        let mut state = State::new();
        state.set_value(42);
        state.increment_pointer();
        state.increment_value();
        state.decrement_pointer();
        assert_eq!(state.get_value(), 42);
        state.increment_pointer();
        assert_eq!(state.get_value(), 1);
    }

    #[test]
    fn init_zero() {
        let mut state = State::new();
        assert_eq!(state.get_value(), 0);
        state.increment_pointer();
        assert_eq!(state.get_value(), 0);
    }

    #[test]
    fn wrap_on_32_bits() {
        let mut state = State::new();
        state.set_value(i32::max_value());
        state.increment_value();
        assert!(state.get_value() < 0);
        state.set_value(i32::min_value());
        state.decrement_value();
        assert!(state.get_value() > 0);
    }

    #[test]
    #[should_panic(expected = "tried to set the pointer to negative address")]
    fn negative_index() {
        let mut state = State::new();
        state.decrement_pointer();
    }
}
