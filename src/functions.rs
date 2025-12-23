use crate::interpret::ASTNode;
use crate::types::FunkState;
use std::collections::HashMap;


// Argument specification for a method parameter
#[derive(Clone, Debug)]
pub enum ArgSpec {
    Identifier(Vec<String>),  // Must be one of these identifiers
    Literal,                   // Must be a quoted string
    Either(Vec<ArgSpec>),      // Must match ANY of these specs
    Any,                       // Any ASTNode type
}

// Type alias for method functions
type MethodFn = fn(&mut FunkState, &[ASTNode]) -> Result<(), String>;

#[derive(Clone, Debug)]
pub struct MethodSignature {
    name: String,
    required_args: Vec<ArgSpec>,
    optional_args: Vec<ArgSpec>,
    help_text: String,
    examples: Vec<String>,
    func: MethodFn,
}

pub struct MethodRegistry {
    methods: HashMap<String, MethodSignature>,
}

impl MethodRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            methods: HashMap::new(),
        };
        
        // Register 'new' with Either variant to accept identifier OR literal as first arg
        registry.register_with_spec(
            "new",
            vec![
                // First arg can be either an identifier (type) OR a literal (name)
                ArgSpec::Either(vec![
                    ArgSpec::Identifier(vec![
                        "project".to_string(),
                        "object".to_string(),
                        "item".to_string(),
                        "milestone".to_string(),
                    ]),
                    ArgSpec::Literal,
                ]),
            ],
            vec![
                // Second arg is optional, must be literal (name when type is specified)
                ArgSpec::Literal,
            ],
            "Creates a new note of the specified type",
            vec![
                "new(\"My Project\")              # Creates a project (default)".to_string(),
                "new(object, \"My Object\")       # Creates an object".to_string(),
                "new(milestone, \"Release 1.0\")  # Creates a milestone".to_string(),
            ],
            method_new
        );
       
        registry.register_with_spec(
            "delete",
            vec![
                // First arg either ident or literal
                ArgSpec::Either(vec![
                    ArgSpec::Identifier(vec![
                    "project".to_string(),
                    "object".to_string(),
                    "item".to_string(),
                    "milestone".to_string(),
                ]),
                ArgSpec::Literal,
            ]),
        ],
        vec![
            // Optional second arg must be literal
            ArgSpec::Literal,
        ],
        "Deletes the specified type with the specified name.",
        vec![
            "delete(\"My Project\")             # Deletes project (default)".to_string(),
            "delete(object, \"My Object\")      # Deletes obejct".to_string(),
            "delete(milestone, \"Release 1.0\") # Delete milstone".to_string(),
        ],
        method_delete
        );

        registry.register_with_spec(
            "list",
            vec![
                // First arg either ident or literal
                ArgSpec::Either(vec![
                    ArgSpec::Identifier(vec![
                    "project".to_string(),
                    "object".to_string(),
                    "item".to_string(),
                    "milestone".to_string(),
                ]),
                ArgSpec::Literal,
            ]),
        ],
        vec![
            // Optional second arg must be literal
            ArgSpec::Literal,
        ],
        "Lists all the children of the specified identifier.",
        vec![
            "list(\"My Project\")             # Lists children of project (default)".to_string(),
            "list(object, \"My Object\")      # Lists obeject".to_string(),
            "list(milestone, \"Release 1.0\") # Lists milstone".to_string(),
        ],
        method_list
        );

        registry.register_with_spec(
            "title",
            vec![ArgSpec::Literal],
            vec![],
            "Sets the title of the current note",
            vec![
                "title(\"My Amazing Project\")".to_string(),
            ],
            method_title
        );
        
        registry
    }
    
    pub fn register_with_spec(
        &mut self,
        name: &str,
        required_args: Vec<ArgSpec>,
        optional_args: Vec<ArgSpec>,
        help_text: &str,
        examples: Vec<String>,
        func: MethodFn,
    ) {
        self.methods.insert(
            name.to_string(),
            MethodSignature {
                name: name.to_string(),
                required_args,
                optional_args,
                help_text: help_text.to_string(),
                examples,
                func,
            },
        );
    }

    pub fn execute(&self, name: &str, state: &mut FunkState, args: &[ASTNode]) -> Result<(), String> {
        match self.methods.get(name) {
            Some(signature) => {
                // Check if the first argument is the help identifier
                if args.len() == 1 {
                    if let ASTNode::Identifier(id) = &args[0] {
                        if id == "help" {
                            // Print help instead of executing
                            self.print_help(signature);
                            return Ok(());
                        }
                    }
                }
                
                // Normal execution path - validate then execute
                self.validate_args(signature, args)?;
                (signature.func)(state, args)
            }
            None => Err(format!("Unknown method: {}", name)),
        }
    }
    
    fn print_help(&self, signature: &MethodSignature) {
        println!("\n=== {} ===", signature.name.to_uppercase());
        println!("{}", signature.help_text);
        println!("\nUsage:");
        
        // Generate signature
        let sig_str = self.format_signature(signature);
        println!("  {}", sig_str);
        
        // Print examples
        if !signature.examples.is_empty() {
            println!("\nExamples:");
            for example in &signature.examples {
                println!("  {}", example);
            }
        }
        
        println!();
    }
    
    fn format_signature(&self, signature: &MethodSignature) -> String {
        let mut parts = vec![signature.name.clone()];
        parts.push("(".to_string());
        
        let mut arg_strs = Vec::new();
        
        // Required args
        for spec in &signature.required_args {
            arg_strs.push(format_arg_spec(spec));
        }
        
        // Optional args
        for spec in &signature.optional_args {
            arg_strs.push(format!("[{}]", format_arg_spec(spec)));
        }
        
        parts.push(arg_strs.join(", "));
        parts.push(")".to_string());
        
        parts.concat()
    }    

    pub fn print_all_methods(&self) {
        println!("\n=== AVAILABLE METHODS ===\n");
        
        let mut methods: Vec<_> = self.methods.values().collect();
        methods.sort_by_key(|m| &m.name);
        
        for signature in methods {
            let sig_str = self.format_signature(signature);
            println!("  {:<30} {}", sig_str, signature.help_text);
        }
        
        println!("\nType .method_name(help) for detailed help on any method.");
        println!();
    }

    pub fn list_methods(&self) -> Vec<String> {
        self.methods.keys().cloned().collect()
    }

    fn validate_args(&self, signature: &MethodSignature, args: &[ASTNode]) -> Result<(), String> {
        let min_args = signature.required_args.len();
        let max_args = min_args + signature.optional_args.len();

        // Check argument count
        if args.len() < min_args {
            return Err(format!(
                "{}() requires at least {} argument(s), got {}",
                signature.name, min_args, args.len()
            ));
        }

        if args.len() > max_args && max_args > 0 {
            return Err(format!(
                "{}() accepts at most {} argument(s), got {}",
                signature.name, max_args, args.len()
            ));
        }

        // Validate each argument against its specification
        for (i, arg) in args.iter().enumerate() {
            let spec = if i < signature.required_args.len() {
                &signature.required_args[i]
            } else {
                let optional_index = i - signature.required_args.len();
                if optional_index < signature.optional_args.len() {
                    &signature.optional_args[optional_index]
                } else {
                    continue; // No more specs to check
                }
            };

            self.validate_arg(arg, spec, i, &signature.name)?;
        }

        Ok(())
    }

    fn validate_arg(&self, arg: &ASTNode, spec: &ArgSpec, position: usize, method_name: &str) -> Result<(), String> {
        match spec {
            ArgSpec::Identifier(valid_ids) => {
                match arg {
                    ASTNode::Identifier(id) => {
                        if !valid_ids.contains(id) {
                            return Err(format!(
                                "{}() argument {} must be one of: {}. Got '{}'",
                                method_name,
                                position + 1,
                                valid_ids.join(", "),
                                id
                            ));
                        }
                        Ok(())
                    }
                    _ => Err(format!(
                        "{}() argument {} must be an identifier (one of: {})",
                        method_name,
                        position + 1,
                        valid_ids.join(", ")
                    )),
                }
            }
            ArgSpec::Literal => {
                match arg {
                    ASTNode::Literal(_) => Ok(()),
                    _ => Err(format!(
                        "{}() argument {} must be a quoted string literal",
                        method_name,
                        position + 1
                    )),
                }
            }
            ArgSpec::Either(specs) => {
                // Try each spec until one succeeds
                let mut errors = Vec::new();
                for spec in specs {
                    match self.validate_arg(arg, spec, position, method_name) {
                        Ok(()) => return Ok(()),  // First match wins!
                        Err(e) => errors.push(e),
                    }
                }
                // None matched - return error indicating what was tried
                Err(format!(
                    "{}() argument {} must match one of: {}",
                    method_name,
                    position + 1,
                    specs.iter().map(|s| format_arg_spec(s)).collect::<Vec<_>>().join(" OR ")
                ))
            }
            ArgSpec::Any => Ok(()),
        }
    }
}


// Helper function to format ArgSpec for display
fn format_arg_spec(spec: &ArgSpec) -> String {
    match spec {
        ArgSpec::Identifier(ids) => ids.join("|"),
        ArgSpec::Literal => "\"string\"".to_string(),
        ArgSpec::Either(specs) => {
            let parts: Vec<String> = specs.iter()
                .map(|s| format_arg_spec(s))
                .collect();
            format!("({})", parts.join("|"))
        }
        ArgSpec::Any => "any".to_string(),
    }
}

// ============ Method Implementations ============

fn method_new(_state: &mut FunkState, args: &[ASTNode]) -> Result<(), String> {
    match args.len() {
        0 => Err("new() requires at least a name: new(\"name\") or new(type, \"name\")".to_string()),
        
        1 => {
            // Only one arg - must be a literal (the name)
            match &args[0] {
                ASTNode::Literal(name) => {
                    println!("Creating new project (default) with name: {}", name);
                    Ok(())
                }
                ASTNode::Identifier(_) => {
                    Err("new() with one argument expects a quoted string name".to_string())
                }
                _ => Err("Invalid argument type".to_string()),
            }
        }
        
        2 => {
            // Two args - identifier (type) + literal (name)
            let note_type = match &args[0] {
                ASTNode::Identifier(type_id) => {
                    // Validate the type
                    match type_id.as_str() {
                        "project" | "object" | "item" | "milestone" => type_id.clone(),
                        _ => return Err(format!("Unknown type '{}'. Valid types: project, object, item, milestone", type_id)),
                    }
                }
                _ => return Err("First argument must be an identifier (type)".to_string()),
            };
            
            let name = match &args[1] {
                ASTNode::Literal(n) => n.clone(),
                _ => return Err("Second argument must be a quoted string (name)".to_string()),
            };
            
            println!("Creating new {} with name: {}", note_type, name);
            Ok(())
        }
        
        _ => Err("new() expects 1 or 2 arguments".to_string()),
    }
}

fn method_delete(_state: &mut FunkState, args: &[ASTNode]) -> Result<(), String> {
    
    match args.len() {
        0 => Err("new() requires at least a name: new(\"name\") or new(type, \"name\")".to_string()),
        
        1 => {
            // Only one arg - must be a literal (the name)
            match &args[0] {
                ASTNode::Literal(name) => {
                    println!("Deleting project (default) with name: {}", name);
                    Ok(())
                }
                ASTNode::Identifier(_) => {
                    Err("new() with one argument expects a quoted string name".to_string())
                }
                _ => Err("Invalid argument type".to_string()),
            }
        }
        
        2 => {
            // Two args - identifier (type) + literal (name)
            let note_type = match &args[0] {
                ASTNode::Identifier(type_id) => {
                    // Validate the type
                    match type_id.as_str() {
                        "project" | "object" | "item" | "milestone" => type_id.clone(),
                        _ => return Err(format!("Unknown type '{}'. Valid types: project, object, item, milestone", type_id)),
                    }
                }
                _ => return Err("First argument must be an identifier (type)".to_string()),
            };
            
            let name = match &args[1] {
                ASTNode::Literal(n) => n.clone(),
                _ => return Err("Second argument must be a quoted string (name)".to_string()),
            };
            
            println!("Deleting {} with name: {}", note_type, name);
            Ok(())
        }
        
        _ => Err("new() expects 1 or 2 arguments".to_string()),
    }
}

fn method_list(_state: &mut FunkState, args: &[ASTNode]) -> Result<(), String> {
    
    match args.len() {
        0 => Err("new() requires at least a name: list(\"name\") or new(type, \"name\")".to_string()),
        
        1 => {
            // Only one arg - must be a literal (the name)
            match &args[0] {
                ASTNode::Literal(name) => {
                    println!("Listing all children of project (default) with name: {}", name);
                    Ok(())
                }
                ASTNode::Identifier(_) => {
                    Err("new() with one argument expects a quoted string name".to_string())
                }
                _ => Err("Invalid argument type".to_string()),
            }
        }
        
        2 => {
            // Two args - identifier (type) + literal (name)
            let note_type = match &args[0] {
                ASTNode::Identifier(type_id) => {
                    // Validate the type
                    match type_id.as_str() {
                        "project" | "object" | "item" | "milestone" => type_id.clone(),
                        _ => return Err(format!("Unknown type '{}'. Valid types: project, object, item, milestone", type_id)),
                    }
                }
                _ => return Err("First argument must be an identifier (type)".to_string()),
            };
            
            let name = match &args[1] {
                ASTNode::Literal(n) => n.clone(),
                _ => return Err("Second argument must be a quoted string (name)".to_string()),
            };
            
            println!("Listing children of {} with name: {}", note_type, name);
            Ok(())
        }
        
        _ => Err("new() expects 1 or 2 arguments".to_string()),
    }
}


// End delete method

fn method_title(state: &mut FunkState, args: &[ASTNode]) -> Result<(), String> {
    // Validation already done by registry
    if let ASTNode::Literal(title) = &args[0] {
        state.title = title.clone();
        println!("Set title to: {}", state.title);
        Ok(())
    } else {
        Err("title() expects a string literal".to_string())
    }
}


// ============ Main Handler ============

pub fn handle_input(ast: ASTNode, state: &mut FunkState, registry: &MethodRegistry) {
    match ast {
        ASTNode::MethodChain(calls) => {
            for call in calls {
                if let Err(e) = execute_method(call, state, registry) {
                    println!("Error: {}", e);
                    break;
                }
            }
        }
        _ => println!("Expected MethodChain"),
    }
}

fn execute_method(node: ASTNode, state: &mut FunkState, registry: &MethodRegistry) -> Result<(), String> {
    match node {
        ASTNode::MethodCall { name, args } => {
            registry.execute(&name, state, &args)
        }
        _ => Err("Expected MethodCall".to_string()),
    }
}
