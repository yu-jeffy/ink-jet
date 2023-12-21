import os
import subprocess
import openai
from dotenv import load_dotenv
import time
import json

load_dotenv()

# For each smart contract
# 1. Create new Rust contract project with "cargo contract new NAME"
# 2. Generate smart contract with GPT
# 3. Replace default smart contract in lib.rs with generated smart contract
# 4. Run "cargo contract build" to compile the smart contract
# 5. Save terminal output in build_result.txt if there are errors, otherwise save a file saying "success"
# 6. Run CoinFabrik Scout and save output as JSON

###############################################
# VARIABLES
###############################################
# set prompt to prompt.txt
base_prompt = ""
with open('prompt.txt', 'r') as file:
    base_prompt = file.read()


###############################################
# HELPER FUNCTIONS
###############################################
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

# Run Coinfabrik Scout
def run_coinfabrik_scout(folder_name):
    orig_dir = os.getcwd()
    os.chdir(folder_name)
    result = subprocess.run(["cargo", "scout-audit", "--output-format", "json"], capture_output=True, text=True)
    os.chdir(orig_dir)
    return result

# Write 'success' or errors to a file in the project folder
def write_build_result_to_file(folder_name, result):
    result_file_path = os.path.join(folder_name, "build_result.txt")
    with open(result_file_path, 'w') as file:
        if result.returncode == 0:
            file.write("success\n")
        else:
            file.write(result.stdout)
            file.write(result.stderr)

# Write Coinfabrik Scout run results to a file in the project folder
def write_audit_result_to_file(folder_name, result):
    result_file_path = os.path.join(folder_name, "audit_result.txt")
    with open(result_file_path, 'w') as file:
        if result.returncode == 0:
            file.write("success\n")
        else:
            file.write(result.stdout)
            file.write(result.stderr)

###############################################
# MAIN FUNCTION
###############################################
def main():
    # Open categories txt file
    with open('categories_single_test.txt', 'r') as file:
        # Iterate over each category/line
        for line in file:
            # Strip newline characters and any leading/trailing whitespace in category/line
            processed_line = line.strip()

            # Inject category into prompt
            current_system_prompt = base_prompt.replace("{type}", processed_line)

            # Generate smart contract with GPT with the new prompt
            contract = generate_smart_contract(current_system_prompt)

            # Clean up the response smart contract, remove markdown markers
            contract_clean = remove_mardown_markers(contract)

            # Format the folder name
            folder_name = processed_line.replace(" ", "_")

            # Create the new project
            result = create_cargo_contract_project(folder_name)
            if result.returncode != 0:
                print(f"Failed to create cargo contract project: {result.stderr}")
                return

            # Wait for 5 seconds for the contract project to be created (not sure if necessary)
            time.sleep(5)

            # Write the smart contract to the lib.rs file
            write_to_lib_rs(folder_name, contract_clean)

            # Build the cargo contract
            result = build_cargo_contract(folder_name)

            # Write the build/compile result to a file
            write_build_result_to_file(folder_name, result)

            # Print if the build was successful or not
            if result.returncode == 0:
                print("Cargo contract built successfully.")
            else:
                print(f"Cargo contract build failed: {result.stderr}")

            # If build compiles without errors, run CoinFabrik Scout
            if result.returncode == 0:
                # Run CoinFabrik Scout audit
                audit_result = run_coinfabrik_scout(folder_name)
                
                # Write the audit run result to a file
                write_audit_result_to_file(folder_name, audit_result)

                # Print if the audit run was successful or not
                if audit_result.returncode == 0:
                    print("CoinFabrik Scout ran successfully successfully.")
                else:
                    print(f"CoinFabrik Scout run failed: {audit_result.stderr}")
            
                # Print the full audit report (report.json)
                
                # Construct the full path to the audit report.json file
                scout_output = os.path.join(folder_name, "report.json")

                # Check if the report.json file exists
                if os.path.exists(scout_output):
                    try:
                        # Open the report.json file and load its content
                        with open(scout_output, 'r') as file:
                            report_content = json.load(file)

                        # Check if the report.json content was successful (empty dictionary) or not, print accordingly
                        if report_content == {}:
                            print("CoinFabrik Scout found no issues.")
                        else:
                            print("CoinFabrik Scout found vulnerabilities:")
                            print(report_content)
                    except json.JSONDecodeError as e:
                        print(f"Error reading CoinFabrik Scout report: {e}")
                    except Exception as e:
                        print(f"An error occurred: {e}")
                else:
                    print(f"No CoinFabrik Scout report.json found in {folder_name}")
            else:
                print("Skipping CoinFabrik Scout because build failed.")

# Run this bitch
main()