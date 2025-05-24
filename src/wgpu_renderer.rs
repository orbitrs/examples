//! Example demonstrating the WGPU renderer with 3D content

use std::time::{Duration, Instant};

use orbit::{
    component::{Component, ComponentError, Context, Node},
    create_renderer,
    renderer::{
        wgpu::{
            camera::{Camera, CameraController},
            mesh::{Mesh, MeshPrimitives},
        },
        Renderer, RendererType,
    },
};

/// A simple 3D scene component
pub struct Scene3D {
    context: Context,
    camera_controller: CameraController,
    last_update: Instant,
}

impl Component for Scene3D {
    type Props = ();

    fn create(_props: Self::Props, context: Context) -> Self {
        // Create a camera
        let camera = Camera::new(
            cgmath::Point3::new(0.0, 1.5, 5.0),
            cgmath::Point3::new(0.0, 0.0, 0.0),
            cgmath::Vector3::unit_y(),
            16.0 / 9.0, // aspect ratio
            45.0,       // fov
            0.1,        // near
            100.0,      // far
        );

        // Create a camera controller
        let camera_controller = CameraController::new(camera, 3.0);

        Self {
            context,
            camera_controller,
            last_update: Instant::now(),
        }
    }

    fn update(&mut self, _props: Self::Props) -> Result<(), ComponentError> {
        // Update camera
        let now = Instant::now();
        let dt = now.duration_since(self.last_update).as_secs_f32();
        self.last_update = now;

        self.camera_controller.update(dt);

        Ok(())
    }

    fn render(&self) -> Result<Vec<Node>, ComponentError> {
        // Create a node for the 3D scene
        let mut node = Node::new(None);

        // Add 3D-specific attributes
        node.add_attribute("renderer".to_string(), "wgpu".to_string());
        node.add_attribute("type".to_string(), "3d".to_string());

        // Add camera data
        // In a real implementation, we would store this data in a more structured way
        // that the renderer can access directly

        Ok(vec![node])
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
    let mut renderer = create_renderer(RendererType::Wgpu)?;

    // Create a context
    let context = Context::new();

    // Create a 3D scene
    let mut scene = Scene3D::create((), context);

    // Main loop
    let mut last_update = Instant::now();

    for i in 0..100 {
        // Update scene
        scene.update(())?;

        // Render scene
        let nodes = scene.render()?;

        // Render nodes
        renderer.render(&nodes[0])?;

        // Sleep to simulate frame timing
        std::thread::sleep(Duration::from_millis(16));

        if i % 10 == 0 {
            println!("Frame {}", i);
        }
    }

    println!("Example completed");

    Ok(())
}
