import subprocess
import json
import os

PROBLEM_DIR = "problems/21_merge_two_sorted_lists"
TEST_CASES_FILE = f"{PROBLEM_DIR}/test_cases.json"
LANGUAGE_COMMANDS = {
    "python":  ["python3", f"{PROBLEM_DIR}/merge_two_sorted_lists.py"],
    "go":      ["go", "run", f"{PROBLEM_DIR}/merge_two_sorted_list.go"],
    "rust":    ["cargo", "run", "--manifest-path", f"{PROBLEM_DIR}/Cargo.toml"],
    "c":       ["bash", "-lc", f"gcc {PROBLEM_DIR}/merge_two_sorted_list.c -o /tmp/c_exec && /tmp/c_exec"],
    "cpp":     ["bash", "-lc", f"g++ -std=c++17 {PROBLEM_DIR}/merge_two_sorted_list.cpp -o /tmp/cpp_exec && /tmp/cpp_exec"],
    "java":    ["bash", "-lc", f"javac {PROBLEM_DIR}/merge_two_sorted_list.java && java -cp {PROBLEM_DIR} MergeTwoSortedList"],
    "js":      ["node", f"{PROBLEM_DIR}/merge_two_sorted_list.js"],
    "csharp":  ["dotnet", "run", "--project", f"{PROBLEM_DIR}/csharp/21_merge_two_sorted_list_cs.csproj"]
}

def run_language(lang, input_str):
    cmd = LANGUAGE_COMMANDS[lang]
    # Support both list and bash-combined commands
    if isinstance(cmd, str):
        cmd = cmd.split()
    result = subprocess.run(cmd, input=input_str.encode(), capture_output=True)
    return result.stdout.decode().strip()

def main():
    with open(TEST_CASES_FILE) as f:
        cases = json.load(f)

    for lang in LANGUAGE_COMMANDS:
        print(f"\n🔧 Running for language: {lang}")
        for case in cases:
            input1, input2 = case["input"]
            expected = case["expected"]
            # Send one JSON-encoded array [list1, list2] on stdin:
            input_str = json.dumps([input1, input2])
            output = run_language(lang, input_str)
            print(f"Input: {input1}, {input2} | Output: {output} | Expected: {expected}")
            assert output == str(expected), f"❌ Mismatch in {lang}"

    print("\n✅ All tests passed for all languages!")

if __name__ == "__main__":
    main()
