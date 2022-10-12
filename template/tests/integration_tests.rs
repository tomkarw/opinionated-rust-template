{% if crate_type == "bin" %}use {{crate_name}}::CONFIG;
{% endif %}use pretty_assertions::assert_eq;

/// TODO: Example integration test, feel free to replace it with something meaningful.
#[test]
fn integrate() {
    let x = 42;
    assert_eq!(x, 42);
}{% if crate_type == "bin" %}

/// Make sure all necessary values are available in the environment.
#[test]
fn test_config() {
    let _ = CONFIG.example_bool;
}{% endif %}
