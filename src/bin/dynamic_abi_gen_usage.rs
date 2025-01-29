use ethers::contract::abigen;

abigen!(
    BalancerContract,
    r"D:\Sanctuary\Dev\Repos\rust\ethereum-client\src\resources\balancer_abi.json", // Path to your ABI file
);

fn main() {
    println!("Bindings generated successfully! Ready for contract interaction.");
}