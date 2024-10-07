mod comparison;
mod conversion;
mod create_instance;
mod math_operations;
mod math_utilities;

fn main() {
    comparison::main();
    let _ = conversion::main();
    let _ = create_instance::main();
    let _ = math_operations::main();
    let _ = math_utilities::main();
}
