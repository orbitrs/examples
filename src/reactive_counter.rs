//! Example demonstrating the new scope-based reactive system in OrbitRS
//! Shows how to use ReactiveScope, Signal, Effect, and ReactiveComputed

use orbit::component::{Component, ComponentError, Context, Node};
use orbit::state::{
    create_computed, create_effect, create_signal, ReactiveComputed, ReactiveScope, Signal,
};

// A simple counter using the new scope-based reactive system
struct ReactiveCounter {
    context: Context,
    // Reactive scope for this component
    scope: ReactiveScope,
    // Base counter state as a signal
    count: Signal<i32>,
    // Derived state for square value
    square: ReactiveComputed<i32, Box<dyn FnMut() -> i32>>,
    // Derived state for is_even
    is_even: ReactiveComputed<bool, Box<dyn FnMut() -> bool>>,
}

// Props for our counter
#[derive(Clone)]
struct CounterProps {
    initial: i32,
}

impl Component for ReactiveCounter {
    type Props = CounterProps;

    fn create(props: Self::Props, context: Context) -> Self {
        // Create reactive scope
        let scope = ReactiveScope::new();
        
        // Initialize base state with signal
        let count = create_signal(&scope, props.initial);
        
        // Create computed values
        // We use Box<dyn FnMut> to avoid explicit lifetime parameters
        let count_clone = count.value.clone();
        let square = create_computed(&scope, Box::new(move || {
            let count_value = *count_clone.borrow();
            count_value * count_value
        }) as Box<dyn FnMut() -> i32>);
        
        let square_clone = square.value.clone();
        let is_even = create_computed(&scope, Box::new(move || {
            if let Some(square_value) = *square_clone.borrow() {
                square_value % 2 == 0
            } else {
                false
            }
        }) as Box<dyn FnMut() -> bool>);

        Self {
            context,
            scope,
            count,
            square,
            is_even,
        }
    }

    fn initialize(&mut self) -> Result<(), ComponentError> {
        println!(
            "ReactiveCounter initialized with count: {}",
            *self.count.get()
        );
        
        let square_value = match self.square.get() {
            Ok(val) => *val,
            Err(_) => -1,
        };
        println!("Square value: {}", square_value);
        
        let is_even = match self.is_even.get() {
            Ok(val) => *val,
            Err(_) => false,
        };
        println!("Is even: {}", is_even);

        // Create an effect that logs when values change
        let count_for_effect = self.count.value.clone();
        let square_for_effect = self.square.value.clone();
        let is_even_for_effect = self.is_even.value.clone();
        
        create_effect(&self.scope, move || {
            let count = *count_for_effect.borrow();
            println!("Effect triggered: count changed to {}", count);
            
            if let Some(square) = *square_for_effect.borrow() {
                if let Some(is_even) = *is_even_for_effect.borrow() {
                    println!("Square: {}, is_even: {}", square, is_even);
                }
            }
        });

        Ok(())
    }

    fn update(&mut self, props: Self::Props) -> Result<(), ComponentError> {
        // Update count signal, which will automatically trigger reactions
        self.count.set(props.initial).map_err(|e| {
            ComponentError::UpdateError(format!("Failed to update count: {}", e))
        })?;
        
        Ok(())
    }

    fn render(&self) -> Result<Vec<Node>, ComponentError> {
        // In a real app, this would render DOM nodes
        println!("Rendering ReactiveCounter:");
        println!("  Count: {}", *self.count.get());
        
        let square_value = match self.square.get() {
            Ok(val) => *val,
            Err(_) => -1,
        };
        println!("  Square: {}", square_value);
        
        let is_even = match self.is_even.get() {
            Ok(val) => *val,
            Err(_) => false,
        };
        println!("  Is even: {}", is_even);

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
    // Increment the counter
    pub fn increment(&mut self) {
        self.count.update(|v| *v += 1).unwrap_or_else(|e| {
            eprintln!("Failed to increment: {}", e);
        });
    }

    // Decrement the counter
    #[allow(dead_code)]
    pub fn decrement(&mut self) {
        self.count.update(|v| *v -= 1).unwrap_or_else(|e| {
            eprintln!("Failed to decrement: {}", e);
        });
    }
}

fn main() {
    println!("Reactive System Example\n");

    // Create a context
    let context = Context::new();

    // Create our counter component
    let mut counter = ReactiveCounter::create(
        CounterProps { initial: 5 },
        context.clone(),
    );

    // Initialize component
    counter.initialize().expect("Failed to initialize counter");

    // Render initial state
    counter.render().expect("Failed to render counter");

    println!("\nIncrementing counter...");
    counter.increment();

    // Render updated state
    counter.render().expect("Failed to render counter");

    println!("\nIncrementing counter again...");
    counter.increment();

    // Render final state
    counter.render().expect("Failed to render counter");

    println!("\nReactive System Example completed!");
}
