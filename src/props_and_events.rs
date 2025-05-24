//! Example demonstrating the enhanced props and event handling system

use std::any::Any;
use std::cell::Cell;
use std::collections::HashMap;
use std::sync::Arc;

// Import the minimum required from orbit
use orbit::prelude::MouseButton;

// We'll use a simple Callback type for the example
#[derive(Clone)]
pub struct Callback<T> {
    f: Arc<dyn Fn(T) + Send + Sync>,
}

impl<T> std::fmt::Debug for Callback<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Callback")
            .field("f", &"<function>")
            .finish()
    }
}

impl<T: Clone + 'static> Callback<T> {
    pub fn new<F>(f: F) -> Self
    where
        F: Fn(T) + Send + Sync + 'static,
    {
        Self { f: Arc::new(f) }
    }

    pub fn call(&self, value: T) {
        (self.f)(value);
    }
}

// We'll use a simplified State type that's thread-safe
#[derive(Clone)]
pub struct State<T> {
    value: Arc<Cell<T>>,
}

impl<T: Copy + 'static> State<T> {
    pub fn new(initial: T) -> Self {
        Self {
            value: Arc::new(Cell::new(initial)),
        }
    }

    pub fn get(&self) -> T {
        self.value.get()
    }

    pub fn set(&self, new_value: T) {
        self.value.set(new_value);
    }
}

// Define a MouseEvent struct for our example
#[derive(Clone, Debug)]
pub struct MouseEvent {
    pub x: f32,
    pub y: f32,
    pub button: MouseButton,
    pub event_type: MouseEventType,
}

// Define event types
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MouseEventType {
    Down,
    Up,
    Move,
    Enter,
    Leave,
    Click,
    DoubleClick,
}

// Define a simplified Node struct for our example
pub struct Node {
    pub attributes: HashMap<String, String>,
    pub children: Vec<Node>,
}

impl Node {
    pub fn new() -> Self {
        Self {
            attributes: HashMap::new(),
            children: Vec::new(),
        }
    }

    pub fn add_child(&mut self, child: Node) {
        self.children.push(child);
    }

    pub fn children(&self) -> &[Node] {
        &self.children
    }
}

// Define a simple DelegatedEvent wrapper for our example
pub struct DelegatedEvent<T> {
    pub event: T,
    propagation_stopped: bool,
    default_prevented: bool,
}

impl<T: Clone> DelegatedEvent<T> {
    pub fn new(event: T) -> Self {
        Self {
            event,
            propagation_stopped: false,
            default_prevented: false,
        }
    }

    pub fn stop_propagation(&mut self) {
        self.propagation_stopped = true;
    }

    pub fn prevent_default(&mut self) {
        self.default_prevented = true;
    }
}

// Define our own ButtonProps for simplicity
#[derive(Clone, Debug)]
pub struct ButtonProps {
    pub label: String,
    pub disabled: bool,
    pub primary: bool,
    pub on_click: Option<Callback<MouseEvent>>,
}

impl ButtonProps {
    // Create a new ButtonProps with default values
    pub fn new() -> Self {
        Self {
            label: "Button".to_string(),
            disabled: false,
            primary: false,
            on_click: None,
        }
    }

    // Builder pattern methods
    pub fn label(mut self, label: String) -> Self {
        self.label = label;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn primary(mut self, primary: bool) -> Self {
        self.primary = primary;
        self
    }

    pub fn on_click(mut self, on_click: Option<Callback<MouseEvent>>) -> Self {
        self.on_click = on_click;
        self
    }
}

// ValidationError type
#[derive(Debug)]
pub enum PropValidationError {
    InvalidValue { name: String, reason: String },
}

impl std::fmt::Display for PropValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PropValidationError::InvalidValue { name, reason } => {
                write!(f, "Invalid value for '{}': {}", name, reason)
            }
        }
    }
}

impl std::error::Error for PropValidationError {}

// Validator trait
pub trait PropValidator<P> {
    fn validate(&self, props: &P) -> Result<(), PropValidationError>;
}

// Create a validator for ButtonProps
struct ButtonPropsValidator;

impl PropValidator<ButtonProps> for ButtonPropsValidator {
    fn validate(&self, props: &ButtonProps) -> Result<(), PropValidationError> {
        if props.label.is_empty() {
            return Err(PropValidationError::InvalidValue {
                name: "label".to_string(),
                reason: "Button label cannot be empty".to_string(),
            });
        }
        Ok(())
    }
}

// Simple Context struct
pub struct Context;

impl Context {
    pub fn new() -> Self {
        Self {}
    }

    pub fn with_parent(_parent: &Context) -> Self {
        Self {}
    }

    pub fn on_mount<F>(&self, f: F)
    where
        F: FnOnce(&Context) + 'static,
    {
        f(self);
    }

    pub fn on_unmount<F>(&self, _f: F)
    where
        F: FnOnce(&Context) + 'static,
    {
        // This would be called when component is unmounted
    }
}

// Component error type
#[derive(Debug)]
pub enum ComponentError {
    ValidationError(PropValidationError),
}

impl std::fmt::Display for ComponentError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ComponentError::ValidationError(err) => {
                write!(f, "Component validation error: {}", err)
            }
        }
    }
}

impl std::error::Error for ComponentError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ComponentError::ValidationError(err) => Some(err),
        }
    }
}

impl From<PropValidationError> for ComponentError {
    fn from(err: PropValidationError) -> Self {
        Self::ValidationError(err)
    }
}

// Component trait
pub trait Component {
    type Props;

    fn create(props: Self::Props, context: Context) -> Self;
    fn update(&mut self, props: Self::Props) -> Result<(), ComponentError>;
    fn render(&self) -> Result<Vec<Node>, ComponentError>;
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

/// Button component with enhanced props and event handling
pub struct Button {
    context: Context,
    props: ButtonProps,
    click_count: State<i32>,
}

impl Component for Button {
    type Props = ButtonProps;

    fn create(props: Self::Props, context: Context) -> Self {
        // Create a state to track clicks (thread-safe)
        let click_count = State::new(0);

        // Register a mount callback
        context.on_mount(|_| {
            println!("Button mounted!");
        });

        Self {
            context,
            props,
            click_count,
        }
    }

    fn update(&mut self, props: Self::Props) -> Result<(), ComponentError> {
        self.props = props;
        Ok(())
    }

    fn render(&self) -> Result<Vec<Node>, ComponentError> {
        // Create a simple node
        let mut node = Node::new();

        // Add attributes
        node.attributes.insert(
            "class".to_string(),
            format!(
                "button {} {}",
                if self.props.primary { "primary" } else { "" },
                if self.props.disabled { "disabled" } else { "" },
            ),
        );

        // In our simplified example, we don't have actual event delegation
        // But we'll simulate the concept

        // Simulate capturing phase
        println!("Capturing phase: would handle button clicks here");

        // Simulate bubbling phase
        println!("Bubbling phase: Button clicked!");

        // Update click count
        let current = self.click_count.get();
        self.click_count.set(current + 1);

        // Here we would call the on_click callback if provided
        if let Some(on_click) = &self.props.on_click {
            let event = MouseEvent {
                x: 100.0,
                y: 100.0,
                button: MouseButton::Left,
                event_type: MouseEventType::Click,
            };
            on_click.call(event);
        }

        // Example of stopping propagation (in real implementation)
        if self.props.disabled {
            println!("Button is disabled, would stop propagation");
            // In a real implementation: event.stop_propagation(), event.prevent_default()
        }

        Ok(vec![node])
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

/// A form component that demonstrates parent-child communication
pub struct Form {
    context: Context,
    submitted: State<bool>,
}

impl Component for Form {
    type Props = ();

    fn create(_props: Self::Props, context: Context) -> Self {
        // Create a state for form submission state (thread-safe)
        let submitted = State::new(false);

        Self { context, submitted }
    }

    fn update(&mut self, _props: Self::Props) -> Result<(), ComponentError> {
        Ok(())
    }

    fn render(&self) -> Result<Vec<Node>, ComponentError> {
        // Create form node
        let mut form_node = Node::new();

        // Create a context for the button
        let button_context = Context::with_parent(&self.context);

        // Create ButtonProps
        let button_props = ButtonProps::new()
            .label("Submit".to_string())
            .primary(true)
            .on_click(Some(Callback::new(|_| {
                println!("Form submitted!");
            })));

        // Create button component
        let button = Button::create(button_props, button_context);

        // Create button node
        let button_nodes = button.render()?;

        // Add button node to form
        for node in button_nodes {
            form_node.add_child(node);
        }

        Ok(vec![form_node])
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

/// Main function
fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Props and Event Handling Example");

    // Create a context
    let context = Context::new();

    // Create a form
    let form = Form::create((), context);

    // Render the form
    let form_nodes = form.render()?;

    println!("Form rendered successfully with {} nodes", form_nodes.len());

    // Print node tree (simplified)
    println!("Node tree:");
    for (i, node) in form_nodes.iter().enumerate() {
        println!("  Node {}: {} children", i, node.children().len());
    }

    Ok(())
}
