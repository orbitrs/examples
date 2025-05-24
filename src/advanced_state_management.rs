//! Example demonstrating advanced state management patterns in OrbitRS
//! Shows thread-safe state with derived values, computed properties, and shared state
//! 
//! NOTE: This example is temporarily using direct thread-safe primitives while the reactive system
//! is being redesigned. It will be updated to use the new reactive system once it supports
//! thread-safe operations.

use orbit::component::{Component, ComponentError, Context, Node};
use std::sync::{Arc, Mutex, RwLock};

// A simple counter with advanced state management features
struct AdvancedCounter {
    context: Context,
    // Base counter state using thread-safe RwLock
    count: Arc<RwLock<i32>>,
    // Derived state for square value
    square: Arc<RwLock<i32>>,
    // Derived state for is_even
    is_even: Arc<RwLock<bool>>,
    // Shared state that could be accessed from other components
    shared_total: Arc<Mutex<i32>>,
}

// Props for our counter
#[derive(Clone)]
struct CounterProps {
    initial: i32,
    shared_total: Arc<Mutex<i32>>,
}

impl Component for AdvancedCounter {
    type Props = CounterProps;

    fn create(props: Self::Props, context: Context) -> Self {
        // Initialize base state with thread-safe containers
        let count = Arc::new(RwLock::new(props.initial));
        
        // Calculate initial square value
        let initial_square = props.initial * props.initial;
        let square = Arc::new(RwLock::new(initial_square));
        
        // Calculate if even
        let is_even = Arc::new(RwLock::new(initial_square % 2 == 0));

        Self {
            context,
            count,
            square,
            is_even,
            shared_total: props.shared_total,
        }
    }

    fn initialize(&mut self) -> Result<(), ComponentError> {
        println!(
            "AdvancedCounter initialized with count: {}",
            self.get_count().unwrap()
        );
        println!("Square value: {}", self.get_square().unwrap());
        println!("Is even: {}", self.is_square_even().unwrap());

        // Register lifecycle hooks using clone of Arc references
        let count_for_hook = self.count.clone();
        let square_for_hook = self.square.clone();
        let is_even_for_hook = self.is_even.clone();
        
        self.context.on_update(move |_| {
            if let Ok(count) = count_for_hook.read() {
                println!("Component updated, count: {}", *count);
                if let (Ok(square), Ok(is_even)) = (square_for_hook.read(), is_even_for_hook.read()) {
                    println!("Square: {}, is_even: {}", *square, *is_even);
                }
            }
        });

        Ok(())
    }

    fn update(&mut self, props: Self::Props) -> Result<(), ComponentError> {
        if let Ok(mut count) = self.count.write() {
            *count = props.initial;

            // Update the square value manually
            let square_value = props.initial * props.initial;
            if let Ok(mut square) = self.square.write() {
                *square = square_value;
            } else {
                return Err(ComponentError::UpdateError("Failed to update square".into()));
            }

            // Update is_even manually
            if let Ok(mut is_even) = self.is_even.write() {
                *is_even = square_value % 2 == 0;
            } else {
                return Err(ComponentError::UpdateError("Failed to update is_even".into()));
            }
        } else {
            return Err(ComponentError::UpdateError("Failed to update count".into()));
        }

        Ok(())
    }

    fn render(&self) -> Result<Vec<Node>, ComponentError> {
        // In a real app, this would render DOM nodes
        println!("Rendering AdvancedCounter:");
        println!("  Count: {}", self.get_count().unwrap());
        
        // Use proper RwLock::read() method instead of non-existent get() method
        if let Ok(square) = self.square.read() {
            println!("  Square: {}", *square);
        } else {
            println!("  Square: [error reading value]");
        }
        
        if let Ok(is_even) = self.is_even.read() {
            println!("  Is even: {}", *is_even);
        } else {
            println!("  Is even: [error reading value]");
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

impl AdvancedCounter {
    // Increment the counter
    pub fn increment(&mut self) {
        if let Ok(mut count) = self.count.write() {
            *count += 1;

            // Update shared state
            if let Ok(mut total) = self.shared_total.lock() {
                *total += 1;
            }

            // Manually update the derived values
            let new_square = *count * *count;
            
            // Update square value
            if let Ok(mut square) = self.square.write() {
                *square = new_square;
            }
            
            // Update is_even value
            if let Ok(mut is_even) = self.is_even.write() {
                *is_even = new_square % 2 == 0;
            }
        }
    }

    // Decrement the counter
    #[allow(dead_code)]
    pub fn decrement(&mut self) {
        if let Ok(mut count) = self.count.write() {
            *count -= 1;

            // Update shared state
            if let Ok(mut total) = self.shared_total.lock() {
                *total -= 1;
            }

            // Manually update the derived values
            let new_square = *count * *count;
            
            // Update square value
            if let Ok(mut square) = self.square.write() {
                *square = new_square;
            }
            
            // Update is_even value
            if let Ok(mut is_even) = self.is_even.write() {
                *is_even = new_square % 2 == 0;
            }
        }
    }

    // Get the current count
    pub fn get_count(&self) -> Result<i32, &str> {
        match self.count.read() {
            Ok(count) => Ok(*count),
            Err(_) => Err("Failed to read count"),
        }
    }

    // Get the square value directly from the RwLock
    #[allow(dead_code)]
    pub fn get_square(&self) -> Result<i32, &str> {
        match self.square.read() {
            Ok(square) => Ok(*square),
            Err(_) => Err("Failed to read square value"),
        }
    }

    // Check if the current square value is even
    #[allow(dead_code)]
    pub fn is_square_even(&self) -> Result<bool, &str> {
        match self.is_even.read() {
            Ok(is_even) => Ok(*is_even),
            Err(_) => Err("Failed to read is_even value"),
        }
    }

    // Get the shared total value
    #[allow(dead_code)]
    pub fn get_shared_total(&self) -> Result<i32, &str> {
        match self.shared_total.lock() {
            Ok(total) => Ok(*total),
            Err(_) => Err("Failed to read shared total"),
        }
    }
}

/// A component that shares state with the counter
struct SharedStateComponent {
    #[allow(dead_code)]
    context: Context,
    shared_total: Arc<Mutex<i32>>,
}

impl Component for SharedStateComponent {
    type Props = Arc<Mutex<i32>>;

    fn create(props: Self::Props, context: Context) -> Self {
        Self {
            context,
            shared_total: props,
        }
    }

    fn update(&mut self, props: Self::Props) -> Result<(), ComponentError> {
        // Update the shared state reference
        self.shared_total = props;
        Ok(())
    }

    fn render(&self) -> Result<Vec<Node>, ComponentError> {
        let total = match self.shared_total.lock() {
            Ok(guard) => *guard,
            Err(_) => -1, // Error case
        };

        println!("SharedStateComponent - Current shared total: {}", total);
        Ok(vec![])
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

fn main() {
    println!("Advanced State Management Example\n");

    // Create shared state
    let shared_total = Arc::new(Mutex::new(0));

    // Create a context
    let context = Context::new();

    // Create our counter component
    let mut counter = AdvancedCounter::create(
        CounterProps {
            initial: 5,
            shared_total: shared_total.clone(),
        },
        context.clone(),
    );

    // Create shared state component
    let shared_component = SharedStateComponent::create(shared_total.clone(), context);

    // Initialize component
    counter.initialize().expect("Failed to initialize counter");

    // Render initial state
    counter.render().expect("Failed to render counter");
    shared_component
        .render()
        .expect("Failed to render shared component");

    println!("\nIncrementing counter...");
    counter.increment();

    // Render updated state
    counter.render().expect("Failed to render counter");
    shared_component
        .render()
        .expect("Failed to render shared component");

    println!("\nIncrementing counter again...");
    counter.increment();

    // Render final state
    counter.render().expect("Failed to render counter");
    shared_component
        .render()
        .expect("Failed to render shared component");

    println!("\nAdvanced State Management example completed!");
}
