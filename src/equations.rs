// TODO Equation parsing and processing!
// Start with basic arithmetic with two operands and move on from there
pub mod equations {
    use rust_decimal::{Decimal, MathematicalOps};

    enum Operation {
        Add,
        Subtract,
        Multiply,
        Divide,
        Exponent,
        SquareRoot
    }

    // Data types permitted in the equation tree
    enum NodeType {
        Operation(Operation),
        Decimal(Decimal),
    }

    struct EquationTreeNode {
        content: NodeType,
        first_child: Option<Box<EquationTreeNode>>,
        second_child: Option<Box<EquationTreeNode>>
    }

    // Public function to take a string slice and present a Decimal
    // This is the main external entry point!
    pub fn process(s: &str) -> Decimal {
        // This will recursively build the entire operations tree
        let root = parse_node(s);
        // Recursively solve the equation from the tree
        collapse_node(root)
    }

    // Solve the node's children recursively, then perform the node's operation
    fn collapse_node(n: EquationTreeNode) -> Decimal{
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
    fn parse_node(s: &str) -> EquationTreeNode {
        // Find the right-most, lowest priority operation in the string
        match lowest_priority_operation(s) {
            // If there are no operations, the string should be a numeric value;
            // return this immediately as a leaf.
            // TODO this will panic on incorrect input formatting, handle more gracefully?
            // Defaulting could be nice, but if the user types something with incorrect
            // formatting I'd rather abort and alert them
            None => {
                let value: Decimal = s.try_into().expect("Could not convert &str to Decimal");
                return EquationTreeNode {
                    content: NodeType::Decimal(value),
                    first_child: None,
                    second_child: None,
                }
            },
            // Construct a node if the value is Some
            Some(t) => {
                return EquationTreeNode {
                    content: NodeType::Operation(t.1),
                    first_child: Some(Box::new(parse_node(&s[..(t.0)]))),
                    second_child: Some(Box::new(parse_node(&s[(t.0 + 1)..])))
                }
            }
        };
    }

    // Find the index and value of the right-most, lowest-priority operation in a string slice
    fn lowest_priority_operation(s: &str) -> Option<(usize, Operation)> {
        let mut candidate: Option<(usize, Operation)> = None;
        let mut candidate_precedence: usize = 0;

        for c in s.chars().rev().enumerate() {
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
        // Return the candidate
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
    fn test_equation() {
        assert_eq!(process("2+8*(5.2/1.3-9/2)+4^2"), Decimal::from(14));
    }
}