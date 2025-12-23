// Public API types
#[derive(Debug, PartialEq, Clone)]
pub enum ASTNode {
    MethodCall { name: String, args: Vec<ASTNode> },
    MethodChain(Vec<ASTNode>),
    Literal(String),
    Identifier(String),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Dot,
    Identifier(String),
    LeftParen,
    RightParen,
    StringLiteral(String),
    Comma,
}

const SKIP_EXP: &[&str] = &["funknotes", "funk"];

// Main entry point
pub fn interpret(input: &str) -> Result<ASTNode, String> {
    let tokens = tokenize(input)?;
    let mut parser = Parser::new(tokens);
    parser.parse()
}

// Lexer
fn tokenize(input: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&ch) = chars.peek() {
        match ch {
            ' ' | '\t' | '\n' => {
                chars.next();
            }
            '.' => {
                tokens.push(Token::Dot);
                chars.next();
            }
            '(' => {
                tokens.push(Token::LeftParen);
                chars.next();
            }
            ')' => {
                tokens.push(Token::RightParen);
                chars.next();
            }
            '"' => {
                chars.next();
                let string = consume_string(&mut chars)?;
                tokens.push(Token::StringLiteral(string));
            }
            'a'..='z' | 'A'..='Z' | '_' => {
                let ident = consume_identifier(&mut chars);
                tokens.push(Token::Identifier(ident));
            }
            ',' => {
                tokens.push(Token::Comma);
                chars.next();
            }
            _ => return Err(format!("Unexpected character: {}", ch)),
        }
    }

    Ok(tokens)
}

fn consume_string(chars: &mut std::iter::Peekable<std::str::Chars>) -> Result<String, String> {
    let mut string = String::new();

    while let Some(&ch) = chars.peek() {
        if ch == '"' {
            chars.next();
            return Ok(string);
        }
        string.push(ch);
        chars.next();
    }

    Err("Unclosed string literal".to_string())
}

fn consume_identifier(chars: &mut std::iter::Peekable<std::str::Chars>) -> String {
    let mut ident = String::new();

    while let Some(&ch) = chars.peek() {
        if ch.is_alphanumeric() || ch == '_' {
            ident.push(ch);
            chars.next();
        } else {
            break;
        }
    }

    ident
}

// Parser
struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, position: 0 }
    }

    fn current(&self) -> Option<&Token> {
        self.tokens.get(self.position)
    }

    fn advance(&mut self) {
        self.position += 1;
    }

    fn parse(&mut self) -> Result<ASTNode, String> {
        let mut calls = Vec::new();

        // Skip optional prefix
        if let Some(Token::Identifier(name)) = self.current() {
            if SKIP_EXP.contains(&name.as_str()) {
                self.advance();
            }
        }

        // Parse method chain
        while self.current().is_some() {
            if let Some(Token::Dot) = self.current() {
                self.advance();
            } else if calls.is_empty() {
                // First method doesn't need a dot
            } else {
                return Err("Expected '.'".to_string());
            }

            let method = self.parse_expression()?;
            calls.push(method);
        }

        Ok(ASTNode::MethodChain(calls))
    }

    fn parse_expression(&mut self) -> Result<ASTNode, String> {
        // Method name
        let name = match self.current() {
            Some(Token::Identifier(n)) => n.clone(),
            _ => return Err("Expected method name".to_string()),
        };
        self.advance();

        // Left paren
        match self.current() {
            Some(Token::LeftParen) => self.advance(),
            _ => return Err("Expected '('".to_string()),
        }

        // Arguments
        let mut args = Vec::new();
        loop {
            match self.current() {
                Some(Token::StringLiteral(s)) => {
                    args.push(ASTNode::Literal(s.clone()));
                    self.advance();
                }
                Some(Token::Identifier(s)) => {
                    args.push(ASTNode::Identifier(s.clone()));
                    self.advance();
                }
                Some(Token::RightParen) => break,
                _ => return Err("Expected string, identifier, or ')'".to_string()),
            }

            if let Some(Token::Comma) = self.current() {
                self.advance();
            } else {
                break;
            }
        }

        // Right paren
        match self.current() {
            Some(Token::RightParen) => self.advance(),
            _ => return Err("Expected ')'".to_string()),
        }

        Ok(ASTNode::MethodCall { name, args })
    }
}

// Example usage (for testing)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_parse() {
        let result = interpret("create(\"note\").tag(\"important\")");
        assert!(result.is_ok());
    }
}
