# Using the Glob Pattern Filter

This tutorial demonstrates how to use the **glob pattern tool** in `code2prompt` to filter and manage files based on include and exclude patterns. Glob patterns work similarly to tools like `tree` or `grep`, providing powerful filtering capabilities. Check out the [detailed explanation](../explanations/glob_pattern_tool.md) for more information.

---

## Prerequisites

Ensure you have `code2prompt` installed. If you haven’t installed it yet, refer to the [Installation Guide](../how_to/install.md).

---

## Understanding Include and Exclude Patterns

Glob patterns allow you to specify rules for filtering files and directories.

- **Include Patterns** (`--include`): Specify files and directories you want to include.
- **Exclude Patterns** (`--exclude`): Specify files and directories you want to exclude.
- **Priority** (`--include-priority`): Resolves conflicts between include and exclude patterns.

---

## Setting Up the Environment

To practice with glob patterns, let’s create a sample folder structure with some files.

### Bash Script to Generate the Test Structure

Run this script to set up a temporary directory structure:

```bash
#!/bin/bash

# Create base directory
mkdir -p test_dir/{lowercase,uppercase,.secret}

# Create files in the structure
echo "content foo.py" > "test_dir/lowercase/foo.py"
echo "content bar.py" > "test_dir/lowercase/bar.py"
echo "content baz.py" > "test_dir/lowercase/baz.py"
echo "content qux.txt" > "test_dir/lowercase/qux.txt"
echo "content corge.txt" > "test_dir/lowercase/corge.txt"
echo "content grault.txt" > "test_dir/lowercase/grault.txt"

echo "CONTENT FOO.py" > "test_dir/uppercase/FOO.PY"
echo "CONTENT BAR.py" > "test_dir/uppercase/BAR.PY"
echo "CONTENT BAZ.py" > "test_dir/uppercase/BAZ.PY"
echo "CONTENT QUX.txt" > "test_dir/uppercase/QUX.TXT"
echo "CONTENT CORGE.txt" > "test_dir/uppercase/CORGE.TXT"
echo "CONTENT GRAULT.txt" > "test_dir/uppercase/GRAULT.TXT"

echo "top secret" > "test_dir/.secret/secret.txt"
```

To clean up the structure later, run:

```bash
rm -rf test_dir
```

---

## Examples: Filtering Files with Include and Exclude Patterns

### Case 1: No Include, No Exclude

Command:

```bash
code2prompt test_dir
```

#### Result

All files are included:

- `lowercase/foo.py`
- `lowercase/bar.py`
- `uppercase/FOO.py`
- `.secret/secret.txt`

---

### Case 2: Exclude Specific File Types

Exclude `.txt` files:

```bash
code2prompt test_dir --exclude="*.txt"
```

#### Result

Excluded:

- All `.txt` files

Included:

- `lowercase/foo.py`
- `lowercase/bar.py`
- `uppercase/FOO.py`

---

### Case 3: Include Specific File Types

Include only Python files:

```bash
code2prompt test_dir --include="*.py"
```

#### Result

Included:

- All `.py` files

Excluded:

- `.secret/secret.txt`

---

### Case 4: Include and Exclude with Priority

Include `.py` files but exclude files in the `uppercase` folder:

```bash
code2prompt test_dir --include="*.py" --exclude="**/uppercase/*" --include-priority=true
```

#### Result

Included:

- All `lowercase/1` files having `.py` extension

Excluded:

- All `uppercase` files
- `.secret/secret.txt`

---

## Summary

The glob pattern tool in `code2prompt` allows you to filter files and directories effectively using:

- `--include` for specifying files to include
- `--exclude` for files to exclude
- `--include-priority` for resolving conflicts between patterns

To practice, set up the sample directory, try out the commands, and see how the tool filters files dynamically.
