from tqdm import tqdm
from contract_gen_helpers import (
    generate_smart_contract,
    remove_mardown_markers,

)


def generate_new_contracts():
    # CONSTANTS
    MAX_CATEGORIES = 2
    CONTRACTS_PER_CATEGORY = 1  # How many contracts to generate for each category
    CATEGORY_FILE = 'config/categories_single_test.txt'  # Where to pull list of categories from
    PROMPT_FILE = 'config/prompt.txt'  # Where to pull basic prompt from
    CONTRACTS_DIR = 'contracts/generated/'  # Where to put contracts generated

    # GATHER ALL CATEGORIES FROM FILE
    categories = []
    with open(CATEGORY_FILE, 'r') as file:
        for line in file:
            cat = line.strip()
            categories.append(cat)

    if len(categories) > MAX_CATEGORIES:
        print("MAXMIMUM CATEGORIES EXCEEDED, INCLUDING ONLY FIRST", MAX_CATEGORIES, "CATEGORIES")
        categories = categories[:MAX_CATEGORIES]

    print("CATEGORIES:\n", categories[:10])
    print(len(categories), "categories loaded")

    # GATHER BASE PROMPT
    with open(PROMPT_FILE, 'r') as prompt_file:
        base_prompt = prompt_file.read()

    # FOR EACH CATEGORY, GENERATE CONTRACTS
    for category in tqdm(categories, desc="All Categories", disable=True):
        # Inject category into prompt
        current_system_prompt = base_prompt.replace("{type}", category)
        # GENERATE MULTIPLE CONTRACTS FOR EACH
        for i in tqdm(range(CONTRACTS_PER_CATEGORY), desc="Generating for " + category):
            # Generate smart contract with GPT with the new prompt
            contract = generate_smart_contract(current_system_prompt)
            # Clean up the response smart contract, remove markdown markers
            contract_clean = remove_mardown_markers(contract)
            # Format the file name
            file_name = category + "_" + str(i) + ".rs"
            file_name = file_name.replace(" ", "_")
            # Write contract to file
            with open(CONTRACTS_DIR + file_name, 'w') as file:
                file.write(contract_clean)

    print("DONE!")


# Run this bitch
generate_new_contracts()
