use {{crate_name}}::ENV;

fn main() {
    println!("Example bool: {}", ENV.example_bool);
    println!("Example list: {:?}", ENV.example_list);
}
