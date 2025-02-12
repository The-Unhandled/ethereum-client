use ethers::contract::Abigen;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Path to your ABI file
    let abi_path = r"./src/resources/contracts/aura_abi.json";

    // Name for the generated contract bindings
    let contract_name = "AuraContract";

    // Output path for the generated bindings
    let output_path = "./src/contracts/aura_service";

    // Generate the bindings
    let bindings = Abigen::new(contract_name, abi_path)?
        .generate()?
        .to_string();

    // Write the bindings to a file
    fs::write(output_path, bindings)?;
    println!("Bindings successfully generated at: {}", output_path);

    Ok(())
}
