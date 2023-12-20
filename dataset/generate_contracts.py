import os
import subprocess

# For each smart contract
# 1. Create new Rust contract project (cargo contract new name_number)
# 2. Generate smart contract with GPT
# 3. Replace default smart contract in lib.rs with generated smart contract
# 4. Run cargo contract build to compile the smart contract
# 5. Save terminal output if there are errors, otherwise save a file saying "success"

# Set the folder name
folder_name = "your_project_name"  # Change this to your desired name

# The Rust code to replace in lib.rs
rust_code = r'''
// Your Rust code here
fn main() {
    println!("Hello, world!");
}
'''

# Create a new cargo contract project
def create_cargo_contract_project(folder_name):
    return subprocess.run(["cargo", "contract", "new", folder_name], capture_output=True, text=True)

# Write your Rust code to the lib.rs file in the new project folder
def write_to_lib_rs(folder_name, rust_code):
    lib_rs_path = os.path.join(folder_name, "src", "lib.rs")
    with open(lib_rs_path, 'w') as file:
        file.write(rust_code)

# Build the cargo contract
def build_cargo_contract(folder_name):
    orig_dir = os.getcwd()
    os.chdir(folder_name)
    result = subprocess.run(["cargo", "contract", "build"], capture_output=True, text=True)
    os.chdir(orig_dir)
    return result

# Write 'success' or errors to a file in the project folder
def write_result_to_file(folder_name, result):
    result_file_path = os.path.join(folder_name, "build_result.txt")
    with open(result_file_path, 'w') as file:
        if result.returncode == 0:
            file.write("success\n")
        else:
            file.write(result.stdout)
            file.write(result.stderr)

# Run the sequence of actions
def main():
    result = create_cargo_contract_project(folder_name)
    if result.returncode != 0:
        print(f"Failed to create cargo contract project: {result.stderr}")
        return
    
    write_to_lib_rs(folder_name, rust_code)
    result = build_cargo_contract(folder_name)
    write_result_to_file(folder_name, result)

    if result.returncode == 0:
        print("Cargo contract built successfully.")
    else:
        print(f"Cargo contract build failed: {result.stderr}")

main()