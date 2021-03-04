trait Arity {
    fn arity(&self) -> usize {
        0
    }
}
trait LogicElement: Arity {
    fn symbol(&self) -> char;
}

struct Operator {
    symbol: char,
    arity: usize,
}
impl Operator {
    fn new(symbol: char, arity: usize) -> Self {
        Self { symbol, arity }
    }
    fn neg() -> Self {
        Operator::new('¬', 1)
    }
    fn or() -> Self {
        Operator::new('∨', 2)
    }
    fn and() -> Self {
        Operator::new('∧', 2)
    }
    fn imp() -> Self {
        Operator::new('→', 2)
    }
}
impl Arity for Operator {
    fn arity(&self) -> usize {
        self.arity
    }
}
impl LogicElement for Operator {
    fn symbol(&self) -> char {
        self.symbol
    }
}

struct Variable {
    symbol: char,
}
impl Variable {
    fn new(symbol: char) -> Self {
        Self { symbol }
    }
}
impl Arity for Variable {}
impl LogicElement for Variable {
    fn symbol(&self) -> char {
        self.symbol
    }
}

struct Wff {
    formula: Vec<Box<dyn LogicElement>>,
}
impl Wff {
    fn new_variable(element: Variable) -> Self {
        Self {
            formula: vec![Box::new(element)],
        }
    }
    fn new(wff: Wff) -> Self {
        let mut formula = Vec::<Box<dyn LogicElement>>::new();
        formula.extend(wff.formula.into_iter());
        Self { formula }
    }
    fn add_1arity(&mut self, operator: Operator) {
        // check arity
        self.formula.push(Box::new(operator));
    }
    fn add_2arity(&mut self, operator: Operator, wff: Wff) {
        // check arity
        self.formula.extend(wff.formula.into_iter());
        self.formula.push(Box::new(operator));
    }
    fn rpn(&self) -> String {
        self.formula
            .iter()
            .map(|element| element.symbol())
            .collect::<String>()
    }
}
impl Arity for Wff {}
impl LogicElement for Wff {
    fn symbol(&self) -> char {
        ' ' // return
    }
}

fn main() {
    let or = Operator::or();
    let imp = Operator::imp();
    let a = Variable::new('A');
    let b = Variable::new('B');
    let b_wff = Wff::new_variable(b);
    let c = Variable::new('C');
    let c_wff = Wff::new_variable(c);

    let mut disj = Wff::new_variable(a);
    disj.add_2arity(or, b_wff);

    let mut impli = Wff::new(disj);
    impli.add_2arity(imp, c_wff);

    println!("Formula: {}", impli.rpn());
}
