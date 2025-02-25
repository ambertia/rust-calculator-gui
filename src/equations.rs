// TODO Equation parsing and processing!
// Start with basic arithmetic with two operands and move on from there
pub mod equations {
    use rust_decimal::Decimal;
    use rust_decimal_macros::dec;

    enum Operation {
        Add,
        Subtract,
        Multiply,
        Divide,
        Exponent,
        SquareRoot
    }

    enum NodeType {
        Operation(Operation),
        Decimal(Decimal),
    }

    struct EquationTreeNode {
        content: NodeType,
        first_child: Option<Box<EquationTreeNode>>,
        second_child: Option<Box<EquationTreeNode>>
    }

    pub fn process(s: &str) -> Decimal {
        // This will recursively build the entire operations tree
        let root = parse_node(s);
        // Recursively solve the equation from the tree
        collapse_node(root)
    }

    fn collapse_node(n: EquationTreeNode) -> Decimal{
        let operation = match n.content {
            NodeType::Decimal(d) => return d,
            NodeType::Operation(o) => o
        };

        let first_operand = collapse_node(*n.first_child.expect("Operation node must have at least one operand"));
        let second_operand = collapse_node(*n.second_child.expect("Operation node has only one operand, but operation requires two operands"));

        match operation {
            Operation::Add => first_operand + second_operand,
            Operation::Subtract => first_operand - second_operand,
            _ => 0.into(),
        }
    }

    // Recursively break the tree down into nodes
    fn parse_node(s: &str) -> EquationTreeNode {
        // Find the lowest priority operation in the string
        let current_operation = match lowest_priority_operation(s) {
            // If there are no operations, the string should be a value
            None => {
                let value: Decimal = s.try_into().expect("Could not convert &str to Decimal");
                return EquationTreeNode {
                    content: NodeType::Decimal(value),
                    first_child: None,
                    second_child: None,
                }
            },
            // Assign the value if search result is Some
            Some(t) => t
        };

        if let '+' | '-' = current_operation.1 {
            return EquationTreeNode {
                content: NodeType::Operation(match current_operation.1 {
                    '+' => Operation::Add,
                    _ => Operation::Subtract
                }),
                first_child: Some(Box::new(parse_node(&s[..(current_operation.0)]))),
                second_child: Some(Box::new(parse_node(&s[(current_operation.0 + 1)..])))
            }
        }
        else {
            EquationTreeNode {
                content: NodeType::Decimal(0.into()),
                first_child: None,
                second_child: None,
            }
        }
    }

    // Find the index and value of the lowest-priority operation in a string slice
    fn lowest_priority_operation(s: &str) -> Option<(usize, char)> {
        for c in s.chars().rev().enumerate() {
            if let '+' | '-' = c.1 {
                return Some((s.len() - c.0 - 1, c.1));
            }
        }
        None
    }
}