//! Example of using component lifecycle and reactive state
//! To run: cargo run --example component_lifecycle

use orbit::{
    component::{Component, ComponentError, Context, LifecyclePhase, Node, Props},
    prelude::*,
};
use std::cell::RefCell;
use std::rc::Rc;

// Simple counter component using the reactive state system
struct Counter {
    context: Context,
    props: CounterProps,
    // Use Arc instead of Rc for thread-safety
    count: State<i32>,
    double_count: i32,
    _effect: Option<Effect>,
}

#[derive(Clone)]
struct CounterProps {
    initial: i32,
    on_change: Option<Box<dyn Fn(i32) + Send + Sync>>,
}

impl Component for Counter {
    type Props = CounterProps;

    fn create(props: Self::Props, context: Context) -> Self {
        // Create a reactive state for the count
        let count = State::new(props.initial);
        
        // Pre-compute the double count
        let double_count = props.initial * 2;

        Self {
            context,
            props,
            count,
            double_count,
            _effect: None,
        }
    }

    fn initialize(&mut self) -> Result<(), ComponentError> {
        println!(
            "Counter component initialized with count {}",
            *self.count
        );

        // We'll handle effects manually in the update method instead
        // since we can't use Rc for thread safety

        // Register lifecycle hooks
        self.context.on_mount(|_| {
            println!("Counter mounted");
        });

        self.context.on_unmount(|_| {
            println!("Counter unmounted");
        });

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
            *self.count = props.initial;
            self.double_count = props.initial * 2;
            
            // Manually call on_change callback
            if let Some(callback) = &props.on_change {
                callback(*self.count);
            }
        }

        self.props = props;
        Ok(())
    }

    fn after_update(&mut self) -> Result<(), ComponentError> {
        println!(
            "Counter.after_update() called, count is now {}",
            *self.count
        );
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
        println!(
            "Counter.render() called, count={}, double_count={}",
            *self.count,
            self.double_count
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
        *self.count += 1;
        self.double_count = *self.count * 2;
        
        // Call the on_change callback if present
        if let Some(callback) = &self.props.on_change {
            callback(*self.count);
        }
    }

    // Method to decrement the counter
    pub fn decrement(&mut self) {
        *self.count -= 1;
        self.double_count = *self.count * 2;
        
        // Call the on_change callback if present
        if let Some(callback) = &self.props.on_change {
            callback(*self.count);
        }
    }

    // Method to get the current count
    pub fn get_count(&self) -> i32 {
        *self.count
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

    // Store a reference to the counter
    let counter_ref = Rc::new(RefCell::new(counter));

    // Initialize the component
    counter_ref.borrow_mut().initialize().unwrap();

    // Mount the component
    counter_ref.borrow_mut().mount().unwrap();

    // Render the component
    counter_ref.borrow().render().unwrap();

    // Increment the counter
    println!("\nIncrementing counter...");
    counter_ref.borrow_mut().increment();

    // Render to see the updated value
    counter_ref.borrow().render().unwrap();

    // Get the current values
    println!("\nCurrent count: {}", counter_ref.borrow().get_count());
    println!(
        "Current double count: {}",
        counter_ref.borrow().get_double_count()
    );

    // Update the component with new props
    println!("\nUpdating props...");
    let new_props = CounterProps {
        initial: 10,
        on_change: Some(Box::new(|n| {
            println!("New onChange callback: count is now {}", n)
        })),
    };

    counter_ref.borrow_mut().update(new_props).unwrap();

    // Render to see the updated value
    counter_ref.borrow().render().unwrap();

    // Increment again to see effect with new callback
    println!("\nIncrementing counter...");
    counter_ref.borrow_mut().increment();
    counter_ref.borrow().render().unwrap();

    // Unmount the component
    println!("\nUnmounting component...");
    counter_ref.borrow_mut().before_unmount().unwrap();
    counter_ref.borrow_mut().unmount().unwrap();
}
