// TODO Equation parsing and processing!
// Start with basic arithmetic with two operands and move on from there
pub mod equations {
    use rust_decimal::{Decimal, MathematicalOps};

    // Supported mathematical operations
    enum Operation {
        Add,
        Subtract,
        Multiply,
        Divide,
        Exponent,
        SquareRoot
    }

    // Data types permitted in the equation parsing tree
    enum NodeType {
        Operation(Operation),
        Decimal(Decimal),
    }

    // Nodes for the equation parsing tree
    struct EquationTreeNode {
        content: NodeType,
        first_child: Option<Box<EquationTreeNode>>,
        second_child: Option<Box<EquationTreeNode>>
    }

    // Public function to take an equation as a string slice and present a Decimal
    // This is the main external entry point!
    pub fn process(s: &str) -> Result<Decimal, String> {
        // This will recursively build the entire operations tree
        let root = parse_node(s)?;
        // Recursively solve the equation from the tree
        Ok(collapse_node(root))
    }

    // Solve the node's children recursively, then perform the node's operation
    fn collapse_node(n: EquationTreeNode) -> Decimal {
        // Look at the content of the node
        let operation = match n.content {
            // Decimal is the base case
            NodeType::Decimal(d) => return d,
            NodeType::Operation(o) => o
        };

        // Recursively solve the children
        let first_operand = collapse_node(*n.first_child.expect("Operation node must have at least one operand"));
        let second_operand = collapse_node(*n.second_child.expect("Operation node has only one operand, but operation requires two operands"));

        // Perform math based on what the actual operation is and implicit return
        match operation {
            Operation::Add => first_operand + second_operand,
            Operation::Subtract => first_operand - second_operand,
            Operation::Multiply => first_operand * second_operand,
            Operation::Divide => first_operand / second_operand,
            Operation::Exponent => first_operand.powd(second_operand),
            _ => 0.into(),
        }
    }

    // Recursively break the equation down into a tree of nodes
    fn parse_node(s: &str) -> Result<EquationTreeNode, String> {

        let strip;
        // Find the right-most, lowest priority operation in the string
        match lowest_priority_operation({
            // If the string starts and ends with parens then strip them
            if s.ends_with(")") && s.starts_with("(") {
                strip = true;
                s.trim_matches(|c| c == '(' || c ==')')
            } else {
                strip = false;
                s
            }
        }) {
            // If there are no operations, the string should be a numeric value;
            // return this immediately as a leaf.
            // TODO this will panic on incorrect input formatting, handle more gracefully?
            None => {
                let value: Decimal = match s.try_into() {
                    Ok(d) => { d },
                    Err(_) => { return Err(format!("Couldn't convert {} to Decimal", s)) },
                };
                return Ok(EquationTreeNode {
                    content: NodeType::Decimal(value),
                    first_child: None,
                    second_child: None,
                })
            },
            // Construct a node with an operation and children if the value is Some
            Some(t) => {
                return Ok(EquationTreeNode {
                    content: NodeType::Operation(t.1),
                    first_child: Some(Box::new(parse_node({
                        // If parentheses were stripped, this must be accounted for
                        if strip { &s[1..(t.0 + 1)] }
                        else { &s[..(t.0)] }
                    })?)),
                    second_child: Some(Box::new(parse_node({
                        if strip { &s[(t.0 + 2)..(s.len() - 1)] }
                        else { &s[(t.0 + 1)..] }
                    })?))
                })
            }
        };
    }

    // Find the index and value of the right-most, lowest-priority operation in a string slice
    fn lowest_priority_operation(s: &str) -> Option<(usize, Operation)> {

        // Variables to record data that must be tracked during the parsing process
        let mut candidate: Option<(usize, Operation)> = None;
        let mut candidate_precedence: usize = 0;
        let mut parenthese_depth: usize = 0;

        // Iterate over every character in the string to be parsed, starting from the back
        for c in s.chars().rev().enumerate() {

            // Process parentheses first, specially
            if c.1 == ')' {
                parenthese_depth += 1;
                continue;
            } else if c.1 == '(' {
                parenthese_depth -= 1;
                continue;
            } else if parenthese_depth > 0 {
                continue;
            }

            // Get the order of operations precedence of this character
            let c_precedence = get_precedence(&c.1);

            // If this character is not an operation, continue - Important base case
            if c_precedence == 0 { continue }
            // Return on the first instance of addition or subtraction,
            // as these are the lowest priority operations
            else if c_precedence == 1 { return Some((s.len() - c.0 - 1, get_operation(&c.1)?)) }
            // If there is no assigned candidate, assign it with this operation
            // If there is a candidate, reassign if the current operation has a lower precedence
            else if candidate.is_none() || c_precedence < candidate_precedence {
                candidate = Some((s.len() - c.0 - 1, get_operation(&c.1)?));
                candidate_precedence = c_precedence;
            }
        }
        // Return the current candidate
        candidate
    }

    // Helper function to reuse a match block
    fn get_operation(c: &char) -> Option<Operation> {
        match c {
            '+' => Some(Operation::Add),
            '-' => Some(Operation::Subtract),
            '*' => Some(Operation::Multiply),
            '/' => Some(Operation::Divide),
            '^' => Some(Operation::Exponent),
            _ => None
        }
    }

    // Helper function to reuse a match block
    // Get the PEMDAS precedence of an operation character (higher is more important)
    // Defaults to 0 if c does not match any patterns
    fn get_precedence(c: &char) -> usize {
        match c {
            '+' | '-' => 1,
            '*' | '/' => 2,
            '^' => 3,
            _ => 0
        }
    }
}

#[cfg(test)]
mod tests {
    use rust_decimal::Decimal;

    use super::equations::process;

    #[test]
    fn basic_operations() {
        assert_eq!(process("2.4+8").unwrap(), Decimal::new(104, 1));
        assert_eq!(process("8-3.82").unwrap(), Decimal::new(418, 2));
        assert_eq!(process("5*3.6").unwrap(), Decimal::new(18, 0));
        assert_eq!(process("9.8/3.2").unwrap(), Decimal::new(30625, 4));
        assert_eq!(process("4^3.5").unwrap(), Decimal::new(128, 0));
    }

    #[test]
    fn order_of_operations() {
        assert_eq!(process("8+2-7*9/2^3").unwrap(), Decimal::new(2125, 3));
        assert_eq!(process("2^7/8*2-9+3").unwrap(), Decimal::new(26, 0));
    }

    #[test]
    fn parentheses() {
        assert_eq!(process("2+8*(5.2/1.3-9/2)+4^2").unwrap(), Decimal::new(14, 0));
        assert_eq!(process("4*(3.4+5*(8-3^2))+2*(8.3-4)").unwrap(), Decimal::new(22, 1));
    }

    #[test]
    fn parentheses_special() {
        assert_eq!(process("sqrt(4)").unwrap(), Decimal::new(2, 0));
        assert_eq!(process("4.2(5)").unwrap(), Decimal::new(21, 0));
        assert_eq!(process("(3+4.6)(20.8/5)").unwrap(), Decimal::new(31616, 3));
    }
}