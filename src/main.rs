mod comparison;
mod conversion;
mod create_instance;
mod deploy_from_artifacts;
mod deploy_from_bytecode;
mod interact_with_abi;
mod math_operations;
mod math_utilities;

fn main() {
    comparison::main();
    let _ = conversion::main();
    let _ = create_instance::main();
    let _ = math_operations::main();
    let _ = math_utilities::main();
    let _ = deploy_from_artifacts::main();
    let _ = deploy_from_bytecode::main();
    let _ = interact_with_abi::main();
}
