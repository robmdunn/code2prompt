# Introduction to `code2prompt`

`code2prompt` is a powerful command-line tool designed to generate large language model (LLM) prompts directly from your codebase. It simplifies the process of combining and formatting code, making it easy to analyze, document, or refactor code using LLMs like GPT or Claude.

This tool is built with scalability and customization in mind, allowing you to handle even the largest codebases effortlessly. Whether you are generating documentation, searching for bugs, or refactoring code, `code2prompt` provides the flexibility and precision needed for various development tasks.

---

## Key Features

- **Generate LLM Prompts**: Quickly convert entire codebases into structured LLM prompts.
- **Glob Pattern Filtering**: Include or exclude specific files and directories using glob patterns.
- **Customizable Templates**: Tailor prompt generation with Handlebars templates.
- **Token Counting**: Analyze token usage and optimize for LLMs with varying context windows.
- **Git Integration**: Include Git diffs and commit messages in prompts for code reviews.
- **Respects `.gitignore`**: Automatically ignores files listed in `.gitignore` to streamline prompt generation.

---

## Getting Started

### Installation

Check out the 👉 [Installation Guide](../how_to/install.md) for detailed instructions on how to install `code2prompt`.

### Usage

Check out the 👉 [Filter Files Guide](../how_to/filter_files.md) to learn how to filter files using glob patterns.

Check out the 👉 [Templates Guide](../tutorials/templates.md) to learn how to use custom Handlebars templates.

With `code2prompt`, you can unlock the full potential of LLMs by creating well-structured and meaningful prompts from your codebase. Let's get started!

---

## Why `code2prompt`?

1. **Save Time**:
   - Automates the process of traversing a codebase and formatting files for LLMs.
   - Avoids repetitive copy-pasting of code.

2. **Improve Productivity**:
   - Provides a structured and consistent format for code analysis.
   - Helps identify bugs, refactor code, and write documentation faster.

3. **Handle Large Codebases**:
   - Designed to work seamlessly with large codebases, respecting context limits of LLMs.

4. **Customizable Workflows**:
   - Flexible options for filtering files, using templates, and generating targeted prompts.

---

## Example Use Cases

- **Code Documentation**:
  Automatically generate documentation for public functions, methods, and classes.

- **Bug Detection**:
  Find potential bugs and vulnerabilities by analyzing your codebase with LLMs.

- **Refactoring**:
  Simplify and optimize code by generating prompts for code quality improvements.

- **Learning and Exploration**:
  Understand new codebases by generating summaries and detailed breakdowns.

- **Git Commit and PR Descriptions**:
  Generate meaningful commit messages and pull request descriptions from Git diffs.

---
