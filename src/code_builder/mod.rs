//! String builder for code generation

/// A builder for generating code
#[derive(Debug, Default)]
pub struct CodeBuilder {
    code: String,
    indent_level: usize,
    indent_str: String,
}

impl CodeBuilder {
    /// Creates a new CodeBuilder
    pub fn new() -> Self {
        Self {
            code: String::new(),
            indent_level: 0,
            indent_str: "    ".to_string(), // Default to 4 spaces
        }
    }

    /// Sets the indentation string
    pub fn with_indent_str(mut self, indent_str: &str) -> Self {
        self.indent_str = indent_str.to_string();
        self
    }

    /// Adds a line of code with current indentation
    pub fn add_line(&mut self, line: &str) -> &mut Self {
        if !line.is_empty() {
            for _ in 0..self.indent_level {
                self.code.push_str(&self.indent_str);
            }
        }
        self.code.push_str(line);
        self.code.push('\n');
        self
    }

    /// Adds a formatted line with current indentation
    pub fn add_line_fmt(&mut self, fmt: &str, args: &[&dyn std::fmt::Display]) -> &mut Self {
        let line = format_args_to_string(fmt, args);
        self.add_line(&line)
    }

    /// Adds a raw line without indentation
    pub fn add_raw_line(&mut self, line: &str) -> &mut Self {
        self.code.push_str(line);
        self.code.push('\n');
        self
    }

    /// Adds a documentation comment
    pub fn add_doc_comment(&mut self, comment: &str) -> &mut Self {
        self.add_line(&format!("/// {comment}"))
    }

    /// Adds a Rust attribute (e.g. #[derive(...)])
    pub fn add_attribute(&mut self, attribute: &str) -> &mut Self {
        self.add_line(&format!("#[{}]", attribute))
    }

    /// Adds multiple lines with indentation
    pub fn add_lines(&mut self, lines: &[&str]) -> &mut Self {
        for line in lines {
            self.add_line(line);
        }
        self
    }

    /// Increases the indentation level
    pub fn indent(&mut self) -> &mut Self {
        self.indent_level += 1;
        self
    }

    /// Decreases the indentation level
    pub fn dedent(&mut self) -> &mut Self {
        if self.indent_level > 0 {
            self.indent_level -= 1;
        }
        self
    }

    /// Adds a block with increased indentation
    pub fn block<F>(&mut self, open: &str, close: &str, f: F) -> &mut Self 
    where
        F: FnOnce(&mut Self),
    {
        self.add_line(open).indent();
        f(self);
        self.dedent().add_line(close)
    }

    /// Builds the final string
    pub fn build(self) -> String {
        self.code
    }
}

// Helper function to format arguments into a string
fn format_args_to_string(fmt: &str, args: &[&dyn std::fmt::Display]) -> String {
    let mut result = fmt.to_string();
    for (i, arg) in args.iter().enumerate() {
        let placeholder = format!("{{{}}}", i);
        result = result.replace(&placeholder, &format!("{}", arg));
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_basic_code_generation() {
        let mut builder = CodeBuilder::new();
        
        builder.add_line("fn main() {")
            .indent()
            .add_line("println!(\"Hello, world!\");")
            .dedent()
            .add_line("}");
        
        let code = builder.build();
        let expected = "fn main() {\n    println!(\"Hello, world!\");\n}\n";
        
        assert_eq!(code, expected);
    }
    
    #[test]
    fn test_doc_comments() {
        let mut builder = CodeBuilder::new();
        
        builder.add_doc_comment("This is a test function")
            .add_line("fn test() {}");
        
        let code = builder.build();
        let expected = "/// This is a test function\nfn test() {}\n";
        
        assert_eq!(code, expected);
    }
    
    #[test]
    fn test_block_generation() {
        let mut builder = CodeBuilder::new();
        
        builder.add_line("fn example() ->String {")
            .indent()
            .add_line("let mut result = String::new();")
            .add_line("result.push_str(\"test\");")
            .add_line("result")
            .dedent()
            .add_line("}");
        
        let code = builder.build();
        let expected = "fn example() ->String {\n    let mut result = String::new();\n    result.push_str(\"test\");\n    result\n}\n";
        
        assert_eq!(code, expected);
    }
}
