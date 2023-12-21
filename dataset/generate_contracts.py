import os
import subprocess
import openai
from dotenv import load_dotenv
import time

load_dotenv()

# For each smart contract
# 1. Create new Rust contract project with "cargo contract new NAME"
# 2. Generate smart contract with GPT
# 3. Replace default smart contract in lib.rs with generated smart contract
# 4. Run "cargo contract build" to compile the smart contract
# 5. Save terminal output in build_result.txt if there are errors, otherwise save a file saying "success"
# 6. Run CoinFabrik Scout and save output as JSON <--- TO DO


# set system prompt to prompt.txt
base_prompt = ""
with open('prompt.txt', 'r') as file:
    base_prompt = file.read()

# gpt call
def generate_smart_contract(prompt):
    # Create the chat completion
    response = openai.chat.completions.create(
        model="gpt-4-1106-preview",
        messages=[
            {"role": "system", "content": ""},
            {"role": "user", "content": prompt}
        ],
        temperature=0.8,
        max_tokens=3000, # larger token size to fit full smart contract
    )
    content = response.choices[0].message.content
    # print(content)
    return content

# Removes ```rust from the beginning and ``` from the end of the string (gpt response).
def remove_mardown_markers(text):
    # Check if the string starts with ```rust and ends with ```
    if text.startswith("```rust") and text.endswith("```"):
        # Remove ```rust from the beginning (7 characters) and ``` from the end (3 characters)
        return text[7:-3]
    else:
        # Return the original string if it doesn't have the specific markers
        return text

# Create a new cargo contract project
def create_cargo_contract_project(folder_name):
    return subprocess.run(["cargo", "contract", "new", folder_name], capture_output=True, text=True)

# Write your Rust code to the lib.rs file in the new project folder
def write_to_lib_rs(folder_name, rust_code):
    lib_rs_path = os.path.join(folder_name, "lib.rs")
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
    with open('categories_single_test.txt', 'r') as file:
        # Iterate over each line
        for line in file:
            # Strip newline characters and any leading/trailing whitespace
            processed_line = line.strip()
            # inject category into prompt
            current_system_prompt = base_prompt.replace("{type}", processed_line)
            # Generate smart contract with GPT
            contract = generate_smart_contract(current_system_prompt)
            # Clean up the contract
            contract_clean = remove_mardown_markers(contract)

            folder_name = processed_line.replace(" ", "_")
            # Create the new project
            result = create_cargo_contract_project(folder_name)
            if result.returncode != 0:
                print(f"Failed to create cargo contract project: {result.stderr}")
                return

            # Wait for 5 seconds for the contract project to be created
            time.sleep(5)

            # Write the smart contract to the lib.rs file
            write_to_lib_rs(folder_name, contract_clean)

            # Build the cargo contract
            result = build_cargo_contract(folder_name)

            # Write the build/compile result to a file
            write_result_to_file(folder_name, result)

            if result.returncode == 0:
                print("Cargo contract built successfully.")
            else:
                print(f"Cargo contract build failed: {result.stderr}")
            
            # TO DO:
            # Run CoinFabrik Scout and save output as JSON

main()