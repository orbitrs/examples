//! Example demonstrating the enhanced props and event handling system

use orbit::prelude::*;
use std::collections::HashMap;

// Define a custom MouseEvent for the example since it's not available in the framework prelude
#[derive(Debug, Clone)]
pub struct MouseEvent {
    pub x: f32,
    pub y: f32,
    pub button: MouseButton,
    pub event_type: MouseEventType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MouseEventType {
    Down,
    Up,
    Move,
    Enter,
    Leave,
    Click,
    DoubleClick,
}

// Define our ButtonProps for the example
#[derive(Clone, Debug)]
pub struct ButtonProps {
    pub label: String,
    pub disabled: bool,
    pub primary: bool,
    pub on_click: Option<Callback<MouseEvent>>,
}

// ButtonProps automatically implements Props via the blanket impl in the framework

impl Default for ButtonProps {
    fn default() -> Self {
        Self::new()
    }
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

// Button component with enhanced props and event handling
pub struct Button {
    id: ComponentId,
    context: Context,
    props: ButtonProps,
    click_count: State<i32>,
}

impl Component for Button {
    type Props = ButtonProps;

    fn component_id(&self) -> ComponentId {
        self.id
    }

    fn create(props: Self::Props, context: Context) -> Self {
        // Create a state to track clicks
        let click_count = State::new(0);

        Self {
            id: ComponentId::new(),
            context,
            props,
            click_count,
        }
    }

    fn initialize(&mut self) -> Result<(), ComponentError> {
        println!("Button initialized with label: {}", self.props.label);
        Ok(())
    }

    fn mount(&mut self) -> Result<(), ComponentError> {
        println!("Button mounted!");
        Ok(())
    }

    fn update(&mut self, props: Self::Props) -> Result<(), ComponentError> {
        self.props = props;
        Ok(())
    }    fn render(&self) -> Result<Vec<Node>, ComponentError> {
        // Create a simple node
        let mut node = Node::new();

        // Add attributes based on props
        node.attributes.insert(
            "class".to_string(),
            format!(
                "button {} {}",
                if self.props.primary { "primary" } else { "" },
                if self.props.disabled { "disabled" } else { "" },
            ),
        );

        node.attributes.insert("label".to_string(), self.props.label.clone());
        
        if self.props.disabled {
            node.attributes.insert("disabled".to_string(), "true".to_string());
        }

        // Add click count as an attribute (for demonstration)
        node.attributes.insert(
            "data-click-count".to_string(), 
            self.click_count.get().unwrap_or(0).to_string()
        );

        // In a real implementation, we would handle event registration through the framework
        // For now, we'll simulate the click handling in the render output
        if let Some(_on_click) = &self.props.on_click {
            println!("Button has click handler registered");
            
            // Simulate a click event for demonstration
            if !self.props.disabled {
                let current_count = self.click_count.get().unwrap_or(0);
                println!("Button can be clicked (current count: {})", current_count);
            }
        }

        Ok(vec![node])
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

/// A form component that demonstrates parent-child communication
pub struct Form {
    id: ComponentId,
    context: Context,
    submitted: State<bool>,
}

impl Component for Form {
    type Props = ();

    fn component_id(&self) -> ComponentId {
        self.id
    }

    fn create(_props: Self::Props, context: Context) -> Self {
        // Create a state for form submission state
        let submitted = State::new(false);

        Self { 
            id: ComponentId::new(),
            context, 
            submitted 
        }
    }

    fn initialize(&mut self) -> Result<(), ComponentError> {
        println!("Form initialized");
        Ok(())
    }

    fn mount(&mut self) -> Result<(), ComponentError> {
        println!("Form mounted!");
        Ok(())
    }

    fn update(&mut self, _props: Self::Props) -> Result<(), ComponentError> {
        Ok(())
    }    fn render(&self) -> Result<Vec<Node>, ComponentError> {
        // Create form node
        let mut form_node = Node::new();
        form_node.attributes.insert("tag".to_string(), "form".to_string());

        // Create a child context for the button
        let button_context = Context::with_parent(&self.context);

        // Create ButtonProps using the framework's Callback
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

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

/// Main function demonstrating the real orbit framework component usage
fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Props and Event Handling Example with Real Orbit Framework");

    // Create a context using the framework
    let context = Context::new();

    // Create a form component
    let mut form = Form::create((), context);

    // Initialize and mount the form (following component lifecycle)
    form.initialize()?;
    form.mount()?;

    // Render the form
    let form_nodes = form.render()?;

    println!("Form rendered successfully with {} nodes", form_nodes.len());

    // Print node tree with more details
    println!("Node tree:");
    for (i, node) in form_nodes.iter().enumerate() {
        println!("  Node {}: {} children", i, node.children().len());
        println!("    Attributes: {:?}", node.attributes);
        
        // Print child details
        for (j, child) in node.children().iter().enumerate() {
            println!("    Child {}: {:?}", j, child.attributes);
        }
    }

    // Demonstrate component lifecycle
    println!("\nComponent ID: {:?}", form.component_id());

    Ok(())
}
