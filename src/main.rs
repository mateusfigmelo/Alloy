mod comparison;
mod conversion;
mod create_instance;
mod deploy_from_artifacts;
mod deploy_from_bytecode;
mod gas_filler;
mod handling_unknown_return_types;
mod interact_with_abi;
mod interact_with_contract_instance;
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
    let _ = interact_with_contract_instance::main();
    let _ = handling_unknown_return_types::main();
    let _ = gas_filler::main();
}
