#!/usr/bin/env python

import re
import argparse

def remove_function(filepath, function_name):
    """Removes a function from a C code file.

    Args:
        filepath: The path to the C code file.
        function_name: The name of the function to remove.
    """

    try:
        with open(filepath, 'r') as f:
            code = f.read()
    except FileNotFoundError:
        print(f"Error: File '{filepath}' not found.")
        return

    # Construct the regex pattern.  Handles various return types,
    # whitespace, and parameter lists.  It's crucial to make this robust.
    # The (?:...) are non-capturing groups.
    pattern = r"^\s*(?:(?:(?:unsigned\s+)?(?:long\s+)?(?:int|char|float|double|void)\s+)+)?\s*" + \
              re.escape(function_name) + r"\s*\((?:[^)]*)\)\s*\{[^{}]*(?:(?:[^{}]+|(?:\{[^{}]*\}))+)[^{}]*\}\s*"

    # Use re.DOTALL to allow . to match newlines, and re.MULTILINE to allow ^ and $ to match start/end of lines
    # within the string.
    match = re.search(pattern, code, re.DOTALL | re.MULTILINE)

    if match:
        new_code = code[:match.start()] + code[match.end():]
        try:
            with open(filepath, 'w') as f:
                f.write(new_code)
            print(f"Function '{function_name}' removed from '{filepath}'.")
        except Exception as e:
            print(f"Error writing to file: {e}")

    else:
        print(f"Function '{function_name}' not found in '{filepath}'.")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Remove a function from a C code file.")
    parser.add_argument("filepath", help="The path to the C code file.")
    parser.add_argument("function_name", help="The name of the function to remove.")

    args = parser.parse_args()
    remove_function(args.filepath, args.function_name)

