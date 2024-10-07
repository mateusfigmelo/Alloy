mod comparison;
mod conversion;
mod create_instance;
mod math_operations;

fn main() {
    comparison::main();
    let _ = conversion::main();
    let _ = create_instance::main();
    let _ = math_operations::main();
}
