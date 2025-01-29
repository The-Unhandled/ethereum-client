use std::fs;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct AbiEntry {
    #[serde(rename = "type")]
    entry_type: String,
    name: Option<String>,
    inputs: Option<Vec<AbiInput>>,
}

#[derive(Debug, Deserialize)]
struct AbiInput {
    name: String,
    #[serde(rename = "type")]
    input_type: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load the JSON file
    let file_content = fs::read_to_string(r"D:\Sanctuary\Dev\Repos\rust\ethereum-client\src\resources\balancer_abi.json")?;

    // Deserialize into a Vec<AbiEntry>
    let abi: Vec<AbiEntry> = serde_json::from_str(&file_content)?;

    // Print each ABI entry
    for entry in abi {
        println!("Type: {}", entry.entry_type);
        if let Some(name) = &entry.name {
            println!("Name: {}", name);
        }
        if let Some(inputs) = &entry.inputs {
            println!("Inputs:");
            for input in inputs {
                println!("  - {}: {}", input.name, input.input_type);
            }
        }
        println!();
    }

    Ok(())
}
