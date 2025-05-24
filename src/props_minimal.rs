use orbit::component::props::*;
use orbit::define_props;

// Example using the basic props macro
define_props! {
    pub struct ButtonProps {
        pub label: String,
        pub disabled: bool,
        pub size: String
    }
}

fn main() {
    println!("Props system minimal example");

    // Using basic props
    let props = ButtonProps {
        label: "Click me".to_string(),
        disabled: false,
        size: "large".to_string(),
    };

    println!(
        "Basic props: label={}, disabled={}, size={}",
        props.label, props.disabled, props.size
    );

    // Using builder pattern
    let props_builder = ButtonProps::builder();
    let props = props_builder.build().unwrap();

    println!(
        "Builder props: label={}, disabled={}, size={}",
        props.label, props.disabled, props.size
    );
}
