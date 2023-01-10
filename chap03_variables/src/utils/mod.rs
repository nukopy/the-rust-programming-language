pub fn print_type_of<T>(_: &T) {
    println!("type: {}", std::any::type_name::<T>())
}
