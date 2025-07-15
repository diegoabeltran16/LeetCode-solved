#!/usr/bin/env python3
import subprocess
import json
import sys

PROBLEM_DIR = "problems/21_merge_two_sorted_lists"
TEST_CASES_FILE = f"{PROBLEM_DIR}/test_cases.json"

LANGUAGE_COMMANDS = {
    "python":  ["python3", f"{PROBLEM_DIR}/merge_two_sorted_lists.py"],
    "go":      ["go", "run", f"{PROBLEM_DIR}/merge_two_sorted_list.go"],
    "rust":    ["bash", "-lc", f"cargo run --manifest-path {PROBLEM_DIR}/Cargo.toml"],
    "c":       ["bash", "-lc", f"gcc {PROBLEM_DIR}/merge_two_sorted_list.c -o /tmp/c_exec && /tmp/c_exec"],
    "cpp":     ["bash", "-lc", f"g++ -std=c++17 {PROBLEM_DIR}/merge_two_sorted_list.cpp -o /tmp/cpp_exec && /tmp/cpp_exec"],
    "java":    ["bash", "-lc", f"javac {PROBLEM_DIR}/merge_two_sorted_list.java && java -cp {PROBLEM_DIR} MergeTwoSortedList"],
    "js":      ["node", f"{PROBLEM_DIR}/merge_two_sorted_list.js"],
    "csharp":  ["dotnet", "run", "--project", f"{PROBLEM_DIR}/csharp/21_merge_two_sorted_list_cs.csproj"]
}


def run_language(lang, input_str):
    cmd = LANGUAGE_COMMANDS[lang]
    result = subprocess.run(cmd, input=input_str.encode(), capture_output=True)
    return result.stdout.decode().strip()


def main():
    # Load test cases
    with open(TEST_CASES_FILE) as f:
        cases = json.load(f)

    # Iterate languages
    for lang, cmd in LANGUAGE_COMMANDS.items():
        print(f"\n🔧 Running for language: {lang}")

        for case in cases:
            input1, input2 = case["input"]
            expected = case["expected"]

            # Prepare JSON stdin
            payload = json.dumps([input1, input2])
            output_raw = run_language(lang, payload)

            # Attempt to parse JSON output
            try:
                output = json.loads(output_raw)
            except json.JSONDecodeError:
                print(f"❌ {lang} did not return valid JSON:")
                print(output_raw)
                sys.exit(1)

            # Report and assert
            print(f"Input: {input1}, {input2} | Output: {output} | Expected: {expected}")
            if output != expected:
                print(f"❌ Mismatch in {lang}")
                sys.exit(1)

    print("\n✅ All tests passed for all languages!")


if __name__ == "__main__":
    main()
