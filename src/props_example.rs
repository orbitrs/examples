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

// Create a validator for ButtonProps that ensures label is not empty
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

// Example using an advanced props structure with defaults
define_props! {
    pub struct AdvancedButtonProps {
        pub label: String,
        pub disabled: bool,
        pub size: String,
        pub onclick: String
    }
}

fn main() {
    println!("Props system example");

    // Using basic props
    let basic_props = ButtonProps {
        label: "Click me".to_string(),
        disabled: false,
        size: "large".to_string(),
    };

    println!(
        "Basic props: label={}, disabled={}, size={}",
        basic_props.label, basic_props.disabled, basic_props.size
    );

    // Using builder pattern with validation
    let props_builder = ButtonProps::builder().with_validator(ButtonPropsValidator);

    // Build the props with validation
    match props_builder.build() {
        Ok(_props) => {
            println!("Validated props successfully");
        }
        Err(e) => {
            println!("Validation error: {}", e);
        }
    }

    // Using advanced props with defaults
    let advanced_props = AdvancedButtonProps {
        label: "Advanced Button".to_string(),
        disabled: false,
        size: "medium".to_string(),
        onclick: "handleClick".to_string(),
    };

    println!(
        "Advanced props: label={}, disabled={}, size={}, onclick={}",
        advanced_props.label, advanced_props.disabled, advanced_props.size, advanced_props.onclick
    );
}
