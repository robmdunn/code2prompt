// src/claude.rs

use serde_json::Value;
use std::path::Path;

/// Formats the content as Claude-compatible XML output
pub struct ClaudeFormatter;

impl ClaudeFormatter {
    /// Indents each line of text by specified amount
    fn indent_text(text: &str, indent: usize) -> String {
        text.lines()
            .map(|line| {
                if line.trim().is_empty() {
                    String::from("")
                } else {
                    format!("{:indent$}{}", "", line, indent = indent)
                }
            })
            .collect::<Vec<_>>()
            .join("\n")
    }
    
    /// Creates a new XML formatted string for Claude from the provided data
    pub fn format(
        path: &Path,
        source_tree: &str,
        files: &[Value],
        git_diff: Option<&str>,
        git_diff_branch: Option<&str>,
        git_log_branch: Option<&str>,
        instructions: Option<&str>,
    ) -> String {
        let mut output = String::with_capacity(1024 * 1024);

        // Project path
        output.push_str(&format!("<project_path>{}</project_path>\n\n", path.display()));

        // Source tree
        if !source_tree.is_empty() {
            output.push_str("<source_tree>\n");
            output.push_str(&Self::indent_text(source_tree, 4));
            output.push_str("\n</source_tree>\n\n");
        }

        // Files section
        output.push_str("<files>\n");
        for file in files {
            if let (Some(path), Some(code)) = (
                file.get("path").and_then(Value::as_str),
                file.get("code").and_then(Value::as_str),
            ) {
                output.push_str("    <file>\n");
                output.push_str(&format!("        <path>{}</path>\n", path));
                output.push_str("        <code>\n");
                
                // Format and indent the code
                let clean_code = Self::remove_markdown_fences(code);
                output.push_str(&Self::indent_text(&clean_code, 12));
                output.push_str("\n        </code>\n");
                output.push_str("    </file>\n");
            }
        }
        output.push_str("</files>\n\n");

        // Git information
        if let Some(diff) = git_diff {
            if !diff.is_empty() {
                output.push_str("<git_diff>\n");
                output.push_str(&Self::indent_text(diff, 4));
                output.push_str("\n</git_diff>\n\n");
            }
        }

        if let Some(branch_diff) = git_diff_branch {
            if !branch_diff.is_empty() {
                output.push_str("<git_diff_branch>\n");
                output.push_str(&Self::indent_text(branch_diff, 4));
                output.push_str("\n</git_diff_branch>\n\n");
            }
        }

        if let Some(branch_log) = git_log_branch {
            if !branch_log.is_empty() {
                output.push_str("<git_log_branch>\n");
                output.push_str(&Self::indent_text(branch_log, 4));
                output.push_str("\n</git_log_branch>\n\n");
            }
        }

        // Instructions if provided
        if let Some(inst) = instructions {
            output.push_str("<instructions>\n");
            output.push_str(&Self::indent_text(inst, 4));
            output.push_str("\n</instructions>\n\n");
        }

        // Final instruction
        output.push_str("<final_instruction>\n");
        output.push_str("    Consider the project path in <project_path>");
        if !source_tree.is_empty() {
            output.push_str(", the source tree in <source_tree>");
        }
        output.push_str(", and the files in <files>");
        
        if git_diff.is_some() || git_diff_branch.is_some() || git_log_branch.is_some() {
            output.push_str(". Review any git changes in ");
            let mut git_parts = Vec::new();
            if git_diff.is_some() { git_parts.push("<git_diff>"); }
            if git_diff_branch.is_some() { git_parts.push("<git_diff_branch>"); }
            if git_log_branch.is_some() { git_parts.push("<git_log_branch>"); }
            output.push_str(&git_parts.join(", "));
            output.push_str(" if present");
        }

        if instructions.is_some() {
            output.push_str(". Then, follow the instructions given in <instructions>");
        }

        output.push_str(". Take a deep breath and think step by step about how to best complete this task.\n");
        output.push_str("</final_instruction>\n");

        output
    }

    /// Removes markdown code fences and language indicators from code
    fn remove_markdown_fences(code: &str) -> String {
        let lines: Vec<&str> = code.lines().collect();
        if lines.len() >= 2 {
            let first = lines[0].trim();
            let last = lines[lines.len() - 1].trim();
            
            if first.starts_with("```") && last == "```" {
                return lines[1..lines.len()-1].join("\n");
            }
        }
        code.to_string()
    }
}