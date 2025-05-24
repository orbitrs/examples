//! Example demonstrating the new scope-based reactive system in OrbitRS
//! Shows how to use signals, effects, and computed values with the reactive system

use orbit::component::{Component, ComponentError, Context, Node};
use orbit::state::{create_signal, create_effect, create_computed, ReactiveScope, Signal, Effect, ReactiveComputed};
use std::cell::RefCell;
use std::rc::Rc;

/// A reactive counter component using the new reactive system
struct ReactiveCounter {
    context: Context,
    scope: ReactiveScope,
    count: Signal<i32>,
    square: ReactiveComputed<i32, Box<dyn FnMut() -> i32>>,
    is_even: ReactiveComputed<bool, Box<dyn FnMut() -> bool>>,
    #[allow(dead_code)]
    effect: Effect<Box<dyn FnMut()>>,
}

#[derive(Clone)]
struct ReactiveCounterProps {
    initial: i32,
}

impl Component for ReactiveCounter {
    type Props = ReactiveCounterProps;

    fn create(props: Self::Props, context: Context) -> Self {
        let scope = ReactiveScope::new();
        
        // Create the base signal
        let count = create_signal(&scope, props.initial);
        
        // Create computed values that depend on the count
        let count_value = count.value.clone();
        let square = create_computed(&scope, Box::new(move || {
            let val = *count_value.borrow();
            val * val
        }));
        
        let count_value2 = count.value.clone();
        let is_even = create_computed(&scope, Box::new(move || {
            let val = *count_value2.borrow();
            let square_val = val * val;
            square_val % 2 == 0
        }));

        // Create an effect that logs changes
        let count_value3 = count.value.clone();
        let log_counter = Rc::new(RefCell::new(0));
        let log_counter_clone = log_counter.clone();
        let effect = create_effect(&scope, Box::new(move || {
            let val = *count_value3.borrow();
            let mut counter = log_counter_clone.borrow_mut();
            *counter += 1;
            println!("Effect #{}: Count changed to {}", *counter, val);
        }));

        Self {
            context,
            scope,
            count,
            square,
            is_even,
            effect,
        }
    }

    fn initialize(&mut self) -> Result<(), ComponentError> {
        println!(
            "ReactiveCounter initialized with count: {}",
            *self.count.get()
        );
        println!("Square value: {}", *self.square.get().unwrap());
        println!("Is even: {}", *self.is_even.get().unwrap());

        Ok(())
    }

    fn update(&mut self, props: Self::Props) -> Result<(), ComponentError> {
        // Update the signal value - this should trigger all dependent computations
        self.count.set(props.initial)?;
        Ok(())
    }

    fn render(&self) -> Result<Vec<Node>, ComponentError> {
        // In a real app, this would render DOM nodes
        println!("Rendering ReactiveCounter:");
        println!("  Count: {}", *self.count.get());
        
        if let Ok(square) = self.square.get() {
            println!("  Square: {}", *square);
        } else {
            println!("  Square: [error computing value]");
        }
        
        if let Ok(is_even) = self.is_even.get() {
            println!("  Is even: {}", *is_even);
        } else {
            println!("  Is even: [error computing value]");
        }

        Ok(vec![])
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

impl ReactiveCounter {
    /// Increment the counter using reactive signals
    pub fn increment(&self) -> Result<(), orbit::state::SignalError> {
        let current = *self.count.get();
        self.count.set(current + 1)
    }

    /// Decrement the counter using reactive signals
    pub fn decrement(&self) -> Result<(), orbit::state::SignalError> {
        let current = *self.count.get();
        self.count.set(current - 1)
    }

    /// Get the current count
    pub fn get_count(&self) -> i32 {
        *self.count.get()
    }

    /// Get the current square value
    pub fn get_square(&self) -> Result<i32, orbit::state::SignalError> {
        self.square.get().map(|val| *val)
    }

    /// Check if the current square value is even
    pub fn is_square_even(&self) -> Result<bool, orbit::state::SignalError> {
        self.is_even.get().map(|val| *val)
    }
}

fn main() {
    println!("Reactive Counter Example\n");

    // Create a context
    let context = Context::new();

    // Create our reactive counter component
    let mut counter = ReactiveCounter::create(
        ReactiveCounterProps { initial: 3 },
        context,
    );

    // Initialize component
    counter.initialize().expect("Failed to initialize counter");

    // Render initial state
    counter.render().expect("Failed to render counter");

    println!("\nIncrementing counter...");
    counter.increment().expect("Failed to increment");

    // Render updated state
    counter.render().expect("Failed to render counter");

    println!("\nIncrementing counter again...");
    counter.increment().expect("Failed to increment");

    // Render final state
    counter.render().expect("Failed to render counter");

    println!("\nDecrementing counter...");
    counter.decrement().expect("Failed to decrement");

    // Render final state
    counter.render().expect("Failed to render counter");

    println!("\nReactive Counter example completed!");
    println!("Final count: {}", counter.get_count());
    println!("Final square: {}", counter.get_square().unwrap());
    println!("Final is_even: {}", counter.is_square_even().unwrap());
}
