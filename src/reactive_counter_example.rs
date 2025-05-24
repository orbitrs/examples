//! Example demonstrating thread-safe reactive patterns in OrbitRS
//! Shows how to manage derived values with thread-safe primitives

use orbit::component::{Component, ComponentError, Context, Node};
use std::sync::{Arc, RwLock};

/// A reactive counter component using thread-safe primitives instead of the non-thread-safe reactive system
struct ReactiveCounter {
    #[allow(dead_code)]
    context: Context,
    // Base counter state
    count: Arc<RwLock<i32>>,
    // Derived state for square value
    square: Arc<RwLock<i32>>,
    // Derived state for is_even
    is_even: Arc<RwLock<bool>>,
}

#[derive(Clone)]
struct ReactiveCounterProps {
    initial: i32,
}

impl Component for ReactiveCounter {
    type Props = ReactiveCounterProps;
    
    fn component_id(&self) -> orbit::component::ComponentId {
        orbit::component::ComponentId::new()
    }

    fn create(props: Self::Props, context: Context) -> Self {
        // Initialize base state with thread-safe containers
        let count = Arc::new(RwLock::new(props.initial));
        
        // Calculate initial derived values
        let initial_square = props.initial * props.initial;
        let square = Arc::new(RwLock::new(initial_square));
        
        // Calculate if even
        let is_even = Arc::new(RwLock::new(initial_square % 2 == 0));

        // Log initial state (replacing the effect)
        println!("Initial counter state: value={}, square={}, is_even={}", 
                 props.initial, initial_square, initial_square % 2 == 0);

        Self {
            context,
            count,
            square,
            is_even,
        }
    }

    fn initialize(&mut self) -> Result<(), ComponentError> {
        let count = match self.count.read() {
            Ok(guard) => *guard,
            Err(_) => return Err(ComponentError::MountError("Failed to read count".into())),
        };
        
        println!("ReactiveCounter initialized with count: {}", count);
        
        if let Ok(square) = self.square.read() {
            println!("Square value: {}", *square);
        }
        
        if let Ok(is_even) = self.is_even.read() {
            println!("Is even: {}", *is_even);
        }

        Ok(())
    }

    fn update(&mut self, props: Self::Props) -> Result<(), ComponentError> {
        // Update the count value
        if let Ok(mut count) = self.count.write() {
            *count = props.initial;
            
            // Update derived values
            let new_square = props.initial * props.initial;
            
            if let Ok(mut square) = self.square.write() {
                *square = new_square;
            } else {
                return Err(ComponentError::UpdateError("Failed to update square".into()));
            }
            
            if let Ok(mut is_even) = self.is_even.write() {
                *is_even = new_square % 2 == 0;
            } else {
                return Err(ComponentError::UpdateError("Failed to update is_even".into()));
            }
            
            Ok(())
        } else {
            Err(ComponentError::UpdateError("Failed to update count".into()))
        }
    }

    fn render(&self) -> Result<Vec<Node>, ComponentError> {
        // In a real app, this would render DOM nodes
        println!("Rendering ReactiveCounter:");
        
        match self.count.read() {
            Ok(count) => println!("  Count: {}", *count),
            Err(_) => println!("  Count: [error reading value]"),
        }
        
        match self.square.read() {
            Ok(square) => println!("  Square: {}", *square),
            Err(_) => println!("  Square: [error reading value]"),
        }
        
        match self.is_even.read() {
            Ok(is_even) => println!("  Is even: {}", *is_even),
            Err(_) => println!("  Is even: [error reading value]"),
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
    /// Increment the counter and update derived values
    pub fn increment(&self) -> Result<(), ComponentError> {
        if let Ok(mut count) = self.count.write() {
            *count += 1;
            
            // Update derived values
            let new_square = *count * *count;
            
            if let Ok(mut square) = self.square.write() {
                *square = new_square;
                
                if let Ok(mut is_even) = self.is_even.write() {
                    *is_even = new_square % 2 == 0;
                    Ok(())
                } else {
                    Err(ComponentError::UpdateError("Failed to update is_even".into()))
                }
            } else {
                Err(ComponentError::UpdateError("Failed to update square".into()))
            }
        } else {
            Err(ComponentError::UpdateError("Failed to update count".into()))
        }
    }

    /// Decrement the counter and update derived values
    pub fn decrement(&self) -> Result<(), ComponentError> {
        if let Ok(mut count) = self.count.write() {
            *count -= 1;
            
            // Update derived values
            let new_square = *count * *count;
            
            if let Ok(mut square) = self.square.write() {
                *square = new_square;
                
                if let Ok(mut is_even) = self.is_even.write() {
                    *is_even = new_square % 2 == 0;
                    Ok(())
                } else {
                    Err(ComponentError::UpdateError("Failed to update is_even".into()))
                }
            } else {
                Err(ComponentError::UpdateError("Failed to update square".into()))
            }
        } else {
            Err(ComponentError::UpdateError("Failed to update count".into()))
        }
    }

    /// Get the current count
    pub fn get_count(&self) -> i32 {
        match self.count.read() {
            Ok(count) => *count,
            Err(_) => -1, // Error case
        }
    }

    /// Get the current square value
    pub fn get_square(&self) -> Result<i32, ComponentError> {
        match self.square.read() {
            Ok(square) => Ok(*square),
            Err(_) => Err(ComponentError::RenderError("Failed to read square".into())),
        }
    }

    /// Check if the current square value is even
    pub fn is_square_even(&self) -> Result<bool, ComponentError> {
        match self.is_even.read() {
            Ok(is_even) => Ok(*is_even),
            Err(_) => Err(ComponentError::RenderError("Failed to read is_even".into())),
        }
    }
}

fn main() {
    println!("Reactive Counter Example\n");

    // Create a context
    let context = Context::new();

    // Create our reactive counter component
    let mut counter = ReactiveCounter::create(ReactiveCounterProps { initial: 3 }, context);

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
