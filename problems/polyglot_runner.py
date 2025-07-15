import subprocess
import json
import os

PROBLEM_DIR = "problems/21_merge_two_sorted_lists"
TEST_CASES_FILE = f"{PROBLEM_DIR}/test_cases.json"
LANGUAGE_COMMANDS = {
    "python":  ["python3", "merge_two_sorted_lists.py"],
    "go":      ["go", "run", "merge_two_sorted_list.go"],
    "rust":    ["cargo", "run", "--manifest-path", f"{PROBLEM_DIR}/Cargo.toml"],
    "c":       ["gcc", "merge_two_sorted_list.c", "-o", "c_exec", "&&", "./c_exec"],
    "cpp":     ["g++", "merge_two_sorted_list.cpp", "-o", "cpp_exec", "&&", "./cpp_exec"],
    "java":    ["javac", "merge_two_sorted_list.java", "&&", "java", "MergeTwoSortedList"],
    "js":      ["node", "merge_two_sorted_list.js"],
    "csharp":  ["dotnet", "run", "--project", f"{PROBLEM_DIR}/csharp/21_merge_two_sorted_list_cs.csproj"]
}

def run_language(lang, input_str):
    cmd = LANGUAGE_COMMANDS[lang]
    if isinstance(cmd, str): cmd = cmd.split()
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
            input_str = f"{input1}\n{input2}"
            output = run_language(lang, input_str)
            print(f"Input: {input1}, {input2} | Output: {output} | Expected: {expected}")
            assert output == str(expected), f"❌ Mismatch in {lang}"

    print("\n✅ All tests passed for all languages!")

if __name__ == "__main__":
    main()
