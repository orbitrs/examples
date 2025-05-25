//! Example of using component lifecycle and reactive state with thread-safe containers
//! To run: cargo run --example component_lifecycle

use orbit::component::{Component, ComponentError, Context, Node};
use std::sync::{Arc, RwLock};

// Simple counter component using thread-safe state management
struct Counter {
    #[allow(dead_code)]
    context: Context,
    props: CounterProps,
    // Use Arc<RwLock<T>> for thread-safety
    count: Arc<RwLock<i32>>,
    double_count: i32,
}

struct CounterProps {
    initial: i32,
    on_change: Option<Box<dyn Fn(i32) + Send + Sync>>,
}

// Since Box<dyn Fn> doesn't implement Clone, we need to handle this differently
impl Clone for CounterProps {
    fn clone(&self) -> Self {
        Self {
            initial: self.initial,
            on_change: None, // We can't clone the function, so we set it to None
        }
    }
}

impl Component for Counter {
    type Props = CounterProps;

    fn component_id(&self) -> orbit::component::ComponentId {
        orbit::component::ComponentId::new()
    }

    fn create(props: Self::Props, context: Context) -> Self {
        // Create a thread-safe state for the count
        let count = Arc::new(RwLock::new(props.initial));

        // Pre-compute the double count
        let double_count = props.initial * 2;

        Self {
            context,
            props,
            count,
            double_count,
        }
    }

    fn initialize(&mut self) -> Result<(), ComponentError> {
        let count = match self.count.read() {
            Ok(guard) => *guard,
            Err(_) => {
                return Err(ComponentError::MountError(
                    "Failed to read count".to_string(),
                ))
            }
        };

        println!("Counter component initialized with count {}", count);

        // Since Context doesn't have lifecycle hook registration methods,
        // we'll just print the messages directly
        println!("Counter lifecycle hooks would be registered here");
        println!("Counter will be mounted soon");

        Ok(())
    }

    fn mount(&mut self) -> Result<(), ComponentError> {
        println!("Counter.mount() called");
        Ok(())
    }

    fn before_update(&mut self, new_props: &Self::Props) -> Result<(), ComponentError> {
        println!(
            "Counter.before_update() called with new initial value {}",
            new_props.initial
        );
        Ok(())
    }

    fn update(&mut self, props: Self::Props) -> Result<(), ComponentError> {
        println!("Counter.update() called with new props");

        // Only update count if initial value changed
        if self.props.initial != props.initial {
            // Update count safely
            match self.count.write() {
                Ok(mut count) => {
                    *count = props.initial;
                    self.double_count = props.initial * 2;

                    // Manually call on_change callback
                    if let Some(callback) = &props.on_change {
                        callback(props.initial);
                    }
                }
                Err(_) => {
                    return Err(ComponentError::UpdateError(
                        "Failed to write to count".to_string(),
                    ))
                }
            }
        }

        self.props = props;
        Ok(())
    }

    fn after_update(&mut self) -> Result<(), ComponentError> {
        // Get count safely
        let count = match self.count.read() {
            Ok(guard) => *guard,
            Err(_) => {
                return Err(ComponentError::UpdateError(
                    "Failed to read count".to_string(),
                ))
            }
        };

        println!("Counter.after_update() called, count is now {}", count);
        Ok(())
    }

    fn before_unmount(&mut self) -> Result<(), ComponentError> {
        println!("Counter.before_unmount() called");
        Ok(())
    }

    fn unmount(&mut self) -> Result<(), ComponentError> {
        println!("Counter.unmount() called");
        Ok(())
    }

    fn render(&self) -> Result<Vec<Node>, ComponentError> {
        // Get count safely
        let count = match self.count.read() {
            Ok(guard) => *guard,
            Err(_) => {
                return Err(ComponentError::RenderError(
                    "Failed to read count".to_string(),
                ))
            }
        };

        println!(
            "Counter.render() called, count={}, double_count={}",
            count, self.double_count
        );

        // This would normally create DOM nodes
        Ok(vec![])
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

impl Counter {
    // Method to increment the counter
    pub fn increment(&mut self) {
        match self.count.write() {
            Ok(mut count) => {
                *count += 1;
                let new_count = *count;
                self.double_count = new_count * 2;

                // Call the on_change callback if present
                if let Some(callback) = &self.props.on_change {
                    callback(new_count);
                }
            }
            Err(e) => {
                eprintln!("Failed to increment counter: {e:?}");
            }
        }
    }

    // Method to decrement the counter
    #[allow(dead_code)] // Included for example purposes even though not used in this demo
    pub fn decrement(&mut self) {
        match self.count.write() {
            Ok(mut count) => {
                *count -= 1;
                let new_count = *count;
                self.double_count = new_count * 2;

                // Call the on_change callback if present
                if let Some(callback) = &self.props.on_change {
                    callback(new_count);
                }
            }
            Err(e) => {
                eprintln!("Failed to decrement counter: {e:?}");
            }
        }
    }

    // Method to get the current count
    pub fn get_count(&self) -> Result<i32, &str> {
        match self.count.read() {
            Ok(count) => Ok(*count),
            Err(_) => Err("Failed to read count"),
        }
    }

    // Method to get the double count
    pub fn get_double_count(&self) -> i32 {
        self.double_count
    }
}

fn main() {
    // Create a counter component
    let context = Context::new();

    let counter_props = CounterProps {
        initial: 0,
        on_change: Some(Box::new(|n| {
            println!("onChange callback: count is now {}", n)
        })),
    };

    let counter = Counter::create(counter_props, context.clone());

    // Store a thread-safe reference to the counter
    let counter_ref = Arc::new(RwLock::new(counter));

    // Initialize the component
    counter_ref.write().unwrap().initialize().unwrap();

    // Mount the component
    counter_ref.write().unwrap().mount().unwrap();

    // Render the component
    counter_ref.read().unwrap().render().unwrap();

    // Increment the counter
    println!("\nIncrementing counter...");
    counter_ref.write().unwrap().increment();

    // Render to see the updated value
    counter_ref.read().unwrap().render().unwrap();

    // Get the current values
    println!(
        "\nCurrent count: {}",
        counter_ref.read().unwrap().get_count().unwrap_or(-1)
    );
    println!(
        "Current double count: {}",
        counter_ref.read().unwrap().get_double_count()
    );

    // Update the component with new props
    println!("\nUpdating props...");
    let new_props = CounterProps {
        initial: 10,
        on_change: Some(Box::new(|n| {
            println!("New onChange callback: count is now {}", n)
        })),
    };

    counter_ref.write().unwrap().update(new_props).unwrap();

    // Render to see the updated value
    counter_ref.read().unwrap().render().unwrap();

    // Increment again to see effect with new callback
    println!("\nIncrementing counter...");
    counter_ref.write().unwrap().increment();
    counter_ref.read().unwrap().render().unwrap();

    // Unmount the component
    println!("\nUnmounting component...");
    counter_ref.write().unwrap().before_unmount().unwrap();
    counter_ref.write().unwrap().unmount().unwrap();
}
