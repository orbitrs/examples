//! Example demonstrating advanced state management patterns in OrbitRS
//! Shows reactive state with derived values, computed properties, and shared state

use std::sync::{Arc, Mutex, RwLock};
use orbit::component::{Component, ComponentError, Context, Node};
use orbit::state::reactive::{Signal, SignalRead};

// A simple counter with advanced reactive state features
struct AdvancedCounter {
    context: Context,
    // Base counter state using thread-safe RwLock
    count: Arc<RwLock<i32>>,
    // Derived state that depends on count
    square: Signal<i32>,
    // Cached computation that depends on square
    is_even: Signal<bool>,
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
        // Initialize base state
        let count = Arc::new(RwLock::new(props.initial));
        
        // Create a reference for closures
        let count_ref = count.clone();
        
        // Create a derived signal that computes the square of count
        let square = Signal::derive(move || {
            let count_guard = count_ref.read().unwrap_or_else(|_| panic!("Failed to read count"));
            *count_guard * *count_guard
        });
        
        // Create another derived signal that depends on the first derived signal
        let square_ref = square.clone();
        let is_even = Signal::derive(move || {
            // Read the derived value
            let square_val = square_ref.get();
            square_val % 2 == 0
        });
        
        Self {
            context,
            count,
            square,
            is_even,
            shared_total: props.shared_total,
        }
    }
    
    fn initialize(&mut self) -> Result<(), ComponentError> {
        println!("AdvancedCounter initialized with count: {}", self.get_count().unwrap());
        println!("Square value: {}", self.square.get());
        println!("Is even: {}", self.is_even.get());
        
        // Register lifecycle hooks
        let is_even = self.is_even.clone();
        self.context.on_update(move |_| {
            println!("Component updated, is_even: {}", is_even.get());
        });
        
        Ok(())
    }
    
    fn update(&mut self, props: Self::Props) -> Result<(), ComponentError> {
        if let Ok(mut count) = self.count.write() {
            *count = props.initial;
        } else {
            return Err(ComponentError::UpdateError("Failed to update count".into()));
        }
        
        // Note: derived signals will automatically update
        
        Ok(())
    }
    
    fn render(&self) -> Result<Vec<Node>, ComponentError> {
        // In a real app, this would render DOM nodes
        println!("Rendering AdvancedCounter:");
        println!("  Count: {}", self.get_count().unwrap());
        println!("  Square: {}", self.square.get());
        println!("  Is even: {}", self.is_even.get());
        
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
            
            // Note: No need to manually update derived values,
            // they will update automatically when accessed
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
        }
    }
    
    // Get the current count
    pub fn get_count(&self) -> Result<i32, &str> {
        match self.count.read() {
            Ok(count) => Ok(*count),
            Err(_) => Err("Failed to read count"),
        }
    }
    
    // Get the square value directly from the derived signal
    pub fn get_square(&self) -> i32 {
        self.square.get()
    }
    
    // Check if the current square value is even
    pub fn is_square_even(&self) -> bool {
        self.is_even.get()
    }
    
    // Get the shared total value
    pub fn get_shared_total(&self) -> Result<i32, &str> {
        match self.shared_total.lock() {
            Ok(total) => Ok(*total),
            Err(_) => Err("Failed to read shared total"),
        }
    }
}

/// A component that shares state with the counter
struct SharedStateComponent {
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
        context.clone()
    );
    
    // Create shared state component
    let shared_component = SharedStateComponent::create(
        shared_total.clone(), 
        context
    );
    
    // Initialize component
    counter.initialize().expect("Failed to initialize counter");
    
    // Render initial state
    counter.render().expect("Failed to render counter");
    shared_component.render().expect("Failed to render shared component");
    
    println!("\nIncrementing counter...");
    counter.increment();
    
    // Render updated state
    counter.render().expect("Failed to render counter");
    shared_component.render().expect("Failed to render shared component");
    
    println!("\nIncrementing counter again...");
    counter.increment();
    
    // Render final state
    counter.render().expect("Failed to render counter");
    shared_component.render().expect("Failed to render shared component");
    
    println!("\nAdvanced State Management example completed!");
}
