import os
from tqdm import tqdm
import time
import json
import glob
import os

from contract_gen_helpers import build_cargo_contract, run_coinfabrik_scout


def test_contracts():
    # CONSTANTS
    TEST_CARGO_DIR = 'Test-Cargo/'  # File to use for testing contracts
    CONTRACTS_DIR = 'contracts/generated/'  # Where to pull contracts to test from
    FAILED_BUILD_DIR = 'contracts/failed_build/'  # Where to put contracts which fail to build
    FAILED_COINFABRIK_DIR = 'contracts/failed_coinfabrik/'  # Where to put contracts which fail CoinFabrik
    FOUND_VULNERABILITY_DIR = 'contracts/found_vulnerabilities/'  # Where to put contracts which pass CoinFabrik with issues
    PASSED_DIR = 'contracts/passed/'  # Where to put contracts which pass CoinFabrik with no issues

    # GATHER ALL CONTRACTS FROM FILE
    contracts = glob.glob(pathname='*.rs', root_dir=CONTRACTS_DIR)
    print("CONTRACTS:\n", contracts[:10])
    print(len(contracts), "contracts found")

    # TEST ALL CONTRACTS
    for contract_file in tqdm(contracts, desc="Testing Contracts", disable=True):
        # Get Contract Name
        contract_name = contract_file.rstrip(".rs")

        # Read contract code
        with open(CONTRACTS_DIR + contract_file, 'r') as f:
            contract_data = f.read()

        # Delete original generated contract file
        # TODO ONLY REALLY IMPORTANT ONCE WE START RUNNING THIS REGULARLY

        # Write contract code into Test-Cargo
        with open(TEST_CARGO_DIR + 'lib.rs', 'w') as lib:
            lib.write(contract_data)

        # Try Building the Contract
        build_result = build_cargo_contract(TEST_CARGO_DIR)

        # IF BUILD FAILED
        if build_result.returncode != 0:
            print("CARGO BUILD FAILED FOR", contract_file)
            # SAVE CONTRACT TO FAILED_BUILD_DIR
            with open(FAILED_BUILD_DIR + contract_file, 'w') as f:
                f.write(contract_data)

            # SAVE ERROR INFO FOR FURTHER FIXING
            with open(FAILED_BUILD_DIR + contract_name + '.txt', 'w') as f:
                f.write(build_result.stdout)
                f.write(build_result.stderr)

        # IF BUILD SUCCEEDED, TEST WITH COINFABRIK
        else:
            # Run CoinFabrik Scout audit
            audit_result = run_coinfabrik_scout(TEST_CARGO_DIR)

            # IF AUDIT FAILED
            if audit_result.returncode != 0:
                print("COINFABRIK AUDIT FAILED FOR", contract_file)
                # SAVE CONTRACT TO FAILED_COINFABRIK_DIR
                with open(FAILED_COINFABRIK_DIR + contract_file, 'w') as f:
                    f.write(contract_data)

                # SAVE ERROR INFO FOR FURTHER FIXING
                with open(FAILED_COINFABRIK_DIR + contract_name + '.txt', 'w', encoding='utf-8') as f:
                    f.write(build_result.stdout)
                    f.write(build_result.stderr)

            # IF AUDIT SUCCEEDED, ANALYZE RESULTS
            else:
                print("CARGO BUILD SUCCEEDED FOR", contract_file)
                # Construct the full path to the audit report.json file
                scout_output = os.path.join(TEST_CARGO_DIR, "report.json")

                # Check if the report.json file exists
                if os.path.exists(scout_output):
                    try:
                        # Open the report.json file and load its content
                        with open(scout_output, 'r') as file:
                            report_content = json.load(file)

                        # Check if the report.json content was successful (empty dictionary) or not, print accordingly
                        if report_content == {}:
                            print("COINFABRIK AUDIT SUCCEEDED WITH NO ISSUES FOR", contract_file)
                            # SAVE CONTRACT TO PASSED_DIR
                            with open(PASSED_DIR + contract_file, 'w') as f:
                                f.write(contract_data)

                        else:
                            print("COINFABRIK AUDIT FOUND ISSUES FOR", contract_file)
                            # SAVE CONTRACT TO FOUND_VULNERABILITY_DIR
                            with open(FOUND_VULNERABILITY_DIR + contract_file, 'w') as f:
                                f.write(contract_data)

                            # SAVE ERROR INFO FOR FURTHER FIXING
                            with open(FOUND_VULNERABILITY_DIR + contract_name + '.txt', 'w') as f:
                                json.dump(report_content, f)

                    except json.JSONDecodeError as e:
                        print(f"Error reading CoinFabrik Scout report: {e}")
                        # TODO ???
                    except Exception as e:
                        print(f"An error occurred: {e}")
                        # TODO ???
                else:
                    print(f"No CoinFabrik Scout report.json found in {TEST_CARGO_DIR}")
                    # TODO ???

    # TODO: OVERWRITE TEST-CARGO lib.rs with dummy code

    print("DONE!")


# Run this bitch
test_contracts()
