pub enum Constraint {
    Text,
    Number,
    Choice(Vec<String>)
}

pub struct Argument {
    name: String,
    constraint: Constraint,
    multi: bool
}

pub struct Command {
    args: Vec<Argument>,
    script: String
}
