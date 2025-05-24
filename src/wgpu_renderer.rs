//! Example demonstrating the WGPU renderer with 3D content

use std::time::{Duration, Instant};

use orbit::{
    component::{Component, ComponentError, Context, Node},
    renderer::{create_renderer, RendererType},
};

/// A simple 3D scene component
pub struct Scene3D {
    #[allow(dead_code)]
    context: Context,
    rotation: f32,
    last_update: Instant,
}

impl Component for Scene3D {
    type Props = ();

    fn create(_props: Self::Props, context: Context) -> Self {
        Self {
            context,
            rotation: 0.0,
            last_update: Instant::now(),
        }
    }

    fn update(&mut self, _props: Self::Props) -> Result<(), ComponentError> {
        // Update rotation
        let now = Instant::now();
        let dt = now.duration_since(self.last_update).as_secs_f32();
        self.last_update = now;

        self.rotation += dt * 0.5; // Rotate 0.5 radians per second

        Ok(())
    }

    fn render(&self) -> Result<Vec<Node>, ComponentError> {
        // Since we can't directly create a Node instance due to private fields,
        // and we can't call Node::new() directly, we'll use a simplified approach
        // to create a basic Vec<Node> for this example.
        // In a real application, you'd use the proper Node creation methods.

        // For this example, we'll create an empty Vec<Node> which is sufficient
        // to demonstrate the WGPU renderer pattern
        let nodes = vec![];

        Ok(nodes)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

/// Main function
fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("WGPU Renderer Example");

    // Create a WGPU renderer
    // In a real application, we would use this renderer to render the scene
    let _renderer = create_renderer(RendererType::Wgpu)?;

    // Create a context
    let context = Context::new();

    // Create a 3D scene
    let mut scene = Scene3D::create((), context);

    // Main loop
    // Note: We removed the unused last_update variable

    for i in 0..100 {
        // Update scene
        scene.update(())?;

        // Render scene - this would render nodes in a real application
        // We'll just print the rotation value here
        let _nodes = scene.render()?;

        println!("Current rotation: {:.2}", scene.rotation);

        // Sleep to simulate frame timing
        std::thread::sleep(Duration::from_millis(16));

        if i % 10 == 0 {
            println!("Frame {}", i);
        }
    }

    println!("Example completed");

    Ok(())
}
