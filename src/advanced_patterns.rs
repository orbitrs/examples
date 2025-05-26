//! Examples demonstrating advanced component patterns in Orbit UI framework
//!
//! This example showcases higher-order components, composition patterns,
//! and performance optimization hooks in action.

use std::sync::Arc;
use std::time::Duration;

use orbit::component::{
    Component, ComponentBase, ComponentError, Context, HOCWrapper, HigherOrderComponent,
    LazyComponent, LoadTrigger, MemoComponent, Memoizable, Node, PerformanceRegistry, Props,
    RenderProp, RenderPropComponent, Slot, SlottedComponent, SlottedProps, WithLogging,
    WithPerformanceMonitoring,
};

/// Example: Basic component for demonstration
#[derive(Clone)]
pub struct ButtonProps {
    pub text: String,
    pub disabled: bool,
}

impl Props for ButtonProps {}

pub struct Button {
    base: ComponentBase,
    props: ButtonProps,
}

impl Component for Button {
    type Props = ButtonProps;

    fn component_id(&self) -> orbit::component::ComponentId {
        self.base.id()
    }

    fn create(props: Self::Props, context: Context) -> Self {
        Self {
            base: ComponentBase::new(context),
            props,
        }
    }

    fn update(&mut self, props: Self::Props) -> Result<(), ComponentError> {
        self.props = props;
        Ok(())
    }

    fn render(&self) -> Result<Vec<Node>, ComponentError> {
        println!("Rendering button: {}", self.props.text);
        // Return a text node
        Ok(vec![Node::text(&self.props.text)])
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

/// Example: Memoizable component
impl Memoizable for Button {
    type MemoKey = String;

    fn memo_key(&self) -> Self::MemoKey {
        format!("{}_{}", self.props.text, self.props.disabled)
    }
}

/// Example: Custom HOC that adds click analytics
pub struct WithAnalytics;

impl HigherOrderComponent<Button> for WithAnalytics {
    type WrappedProps = ButtonProps;
    type HOCProps = ButtonProps;

    fn transform_props(hoc_props: &Self::HOCProps) -> Self::WrappedProps {
        hoc_props.clone()
    }

    fn enhance_component(
        _component: &mut Button,
        hoc_props: &Self::HOCProps,
    ) -> Result<(), ComponentError> {
        println!("📊 Analytics: Button '{}' enhanced", hoc_props.text);
        Ok(())
    }

    fn on_wrapped_mount(
        component: &mut Button,
        _context: &orbit::component::MountContext,
        _hoc_props: &Self::HOCProps,
    ) -> Result<(), ComponentError> {
        println!("📊 Analytics: Button '{}' mounted", component.props.text);
        Ok(())
    }
}

/// Example: Data provider component using render props
#[derive(Clone)]
pub struct UserData {
    pub name: String,
    pub email: String,
}

pub fn user_list_renderer(users: Vec<UserData>) -> Result<Vec<Node>, ComponentError> {
    println!("Rendering {} users", users.len());
    let mut nodes = Vec::new();
    for user in users {
        nodes.push(Node::text(&format!("{} ({})", user.name, user.email)));
    }
    Ok(nodes)
}

/// Example: Modal component using slots
pub struct Modal {
    base: ComponentBase,
}

impl Component for Modal {
    type Props = SlottedProps;

    fn component_id(&self) -> orbit::component::ComponentId {
        self.base.id()
    }

    fn create(_props: Self::Props, context: Context) -> Self {
        Self {
            base: ComponentBase::new(context),
        }
    }

    fn update(&mut self, _props: Self::Props) -> Result<(), ComponentError> {
        Ok(())
    }

    fn render(&self) -> Result<Vec<Node>, ComponentError> {
        println!("Rendering modal with slots");
        Ok(vec![Node::text("Modal Container")])
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

/// Demonstration function showcasing all patterns
pub fn demo_advanced_patterns() -> Result<(), ComponentError> {
    println!("🚀 Advanced Component Patterns Demo\n");

    let context = Context::new();

    // 1. Higher-Order Components Demo
    println!("1. Higher-Order Components (HOCs)");
    println!("================================");

    let button_props = ButtonProps {
        text: "Click Me".to_string(),
        disabled: false,
    };

    // Basic button
    let mut basic_button = Button::create(button_props.clone(), context.clone());
    println!("Basic button render:");
    basic_button.render()?;

    // Button with logging HOC
    let mut logged_button = HOCWrapper::<WithLogging, Button>::new(button_props.clone(), context.clone())?;
    println!("\nLogged button render:");
    logged_button.render()?;

    // Button with performance monitoring HOC
    let mut monitored_button = HOCWrapper::<WithPerformanceMonitoring, Button>::new(button_props.clone(), context.clone())?;
    println!("\nMonitored button render:");
    monitored_button.render()?;

    // Button with custom analytics HOC
    let mut analytics_button = HOCWrapper::<WithAnalytics, Button>::new(button_props.clone(), context.clone())?;
    println!("\nAnalytics button render:");
    analytics_button.render()?;

    println!("\n");

    // 2. Memoization Demo
    println!("2. Component Memoization");
    println!("========================");

    let memo_button = MemoComponent::new(Button::create(button_props.clone(), context.clone()));
    println!("First render (cache miss):");
    memo_button.render()?;
    println!("Second render (cache hit):");
    memo_button.render()?;

    println!("\n");

    // 3. Render Props Demo
    println!("3. Render Props Pattern");
    println!("======================");

    let users = vec![
        UserData {
            name: "Alice".to_string(),
            email: "alice@example.com".to_string(),
        },
        UserData {
            name: "Bob".to_string(),
            email: "bob@example.com".to_string(),
        },
    ];

    let render_prop_component = RenderPropComponent::new(
        users,
        user_list_renderer,
        context.clone(),
    );
    render_prop_component.render()?;

    println!("\n");

    // 4. Slots Demo
    println!("4. Slot-based Composition");
    println!("=========================");

    use orbit::{slot, slotted_props};

    let modal_props = slotted_props!(
        slot!("header"),
        slot!("content", required),
        slot!("footer")
    );

    let modal = Modal::create(modal_props, context.clone());
    modal.render()?;

    println!("\n");

    // 5. Lazy Loading Demo
    println!("5. Lazy Component Loading");
    println!("========================");

    let mut lazy_button = LazyComponent::<Button>::new(context.clone(), LoadTrigger::OnMount);
    println!("Before mount (should be empty):");
    lazy_button.render()?;

    lazy_button.mount()?;
    println!("After mount (should render):");
    lazy_button.render()?;

    println!("\n");

    // 6. Performance Registry Demo
    println!("6. Performance Monitoring");
    println!("========================");

    let registry = PerformanceRegistry::new();
    let monitor = registry.monitor();
    
    let component_id = basic_button.component_id();
    let _timer = monitor.start_render_timing(component_id);
    
    // Simulate some work
    std::thread::sleep(Duration::from_millis(1));
    
    // Timer automatically records when dropped
    drop(_timer);
    
    if let Some(avg_time) = monitor.get_average_render_time(component_id) {
        println!("Average render time: {:?}", avg_time);
    }

    let stats = monitor.get_render_statistics(component_id);
    println!("Render statistics: {:?}", stats);

    println!("\n🎉 Advanced Patterns Demo Complete!");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hoc_wrapper() {
        let context = Context::new();
        let props = ButtonProps {
            text: "Test".to_string(),
            disabled: false,
        };

        let wrapped = HOCWrapper::<WithLogging, Button>::new(props, context);
        assert!(wrapped.is_ok());
    }

    #[test]
    fn test_memoized_component() {
        let context = Context::new();
        let props = ButtonProps {
            text: "Test".to_string(),
            disabled: false,
        };

        let button = Button::create(props, context);
        let memo_button = MemoComponent::new(button);
        
        assert!(memo_button.render().is_ok());
    }

    #[test]
    fn test_lazy_component() {
        let context = Context::new();
        let mut lazy = LazyComponent::<Button>::new(context, LoadTrigger::OnMount);
        
        // Should render empty before mount
        let result = lazy.render();
        assert!(result.is_ok());
        assert!(result.unwrap().is_empty());
    }

    #[test]
    fn test_demo_runs_without_errors() {
        assert!(demo_advanced_patterns().is_ok());
    }
}

fn main() -> Result<(), ComponentError> {
    demo_advanced_patterns()
}
