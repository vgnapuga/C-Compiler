#[derive(Debug, Clone)]
pub struct VarDecl {
    pub ty: Type,
    pub vars: Vec<(String, Option<Expr>)>,
}

#[derive(Debug, Clone)]
pub struct Param {
    pub ty: Type,
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct FuncDecl {
    pub return_ty: Type,
    pub name: String,
    pub params: Vec<Param>,
    pub body: Vec<Stmt>,
}

#[derive(Debug, Clone)]
pub enum TopLevel {
    Func(FuncDecl),
    Var(VarDecl),
}

#[derive(Debug, Clone)]
pub struct Program {
    pub items: Vec<TopLevel>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Int,
    Char,
    Bool,
    Void,
    Array(Box<Type>, Option<usize>),
}

#[derive(Debug, Clone)]
pub enum Expr {
    IntLit(i64),
    CharLit(char),
    BoolLit(bool),
    Ident(String),
    Binary {
        op: BinaryOp,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
    Unary {
        op: UnaryOp,
        operand: Box<Expr>,
    },
    Call {
        name: String,
        args: Vec<Expr>,
    },
}

#[derive(Debug, Clone)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Eq,
    Ne,
    Lt,
    Gt,
    Le,
    Ge,
    Assign,
}

#[derive(Debug, Clone)]
pub enum UnaryOp {
    Not,
}

#[derive(Debug, Clone)]
pub enum ForInit {
    Expr(Expr),
    Empty,
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Block(Vec<Stmt>),
    If {
        cond: Expr,
        then_br: Box<Stmt>,
        else_br: Option<Box<Stmt>>,
    },
    While {
        cond: Expr,
        body: Box<Stmt>,
    },
    DoWhile {
        body: Box<Stmt>,
        cond: Expr,
    },
    For {
        init: ForInit,
        cond: Option<Expr>,
        update: Option<Expr>,
        body: Box<Stmt>,
    },
    Return(Option<Expr>),
    Expr(Expr),
    Empty,
    VarDecl(VarDecl),
}

impl Type {
    pub fn display(&self) -> String {
        match self {
            Type::Int => "int".into(),
            Type::Char => "char".into(),
            Type::Bool => "bool".into(),
            Type::Void => "void".into(),
            Type::Array(elem, size) => {
                let sz = size.map_or("?".to_owned(), |n| n.to_string());
                format!("{}[{}]", elem.display(), sz)
            }
        }
    }
}

fn pad(indent: usize) -> String {
    "  ".repeat(indent)
}

impl Expr {
    pub fn print_tree(&self, i: usize) {
        let p = pad(i);

        match self {
            Expr::IntLit(v) => println!("{p}IntLit({v})"),
            Expr::CharLit(c) => println!("{p}CharLit({c:?})"),
            Expr::BoolLit(b) => println!("{p}BoolLit({b})"),
            Expr::Ident(name) => println!("{p}Ident({name})"),
            Expr::Binary { op, lhs, rhs } => {
                println!("{p}Binary({op:?})");
                lhs.print_tree(i + 1);
                rhs.print_tree(i + 1);
            }
            Expr::Unary { op, operand } => {
                println!("{p}Unary({op:?})");
                operand.print_tree(i + 1);
            }
            Expr::Call { name, args } => {
                println!("{p}Call({name})");
                for arg in args {
                    arg.print_tree(i + 1);
                }
            }
        }
    }
}

impl VarDecl {
    pub fn print_tree(&self, i: usize) {
        let p = pad(i);
        println!("{p}VarDecl [{}]", self.ty.display());

        for (name, init) in &self.vars {
            println!("{}Var: {name}", pad(i + 1));
            if let Some(e) = init {
                e.print_tree(i + 2);
            }
        }
    }
}

impl Stmt {
    pub fn print_tree(&self, i: usize) {
        let p = pad(i);

        match self {
            Stmt::Block(stmts) => {
                println!("{p}Block");

                for s in stmts {
                    s.print_tree(i + 1);
                }
            }
            Stmt::If {
                cond,
                then_br,
                else_br,
            } => {
                println!("{p}If");

                println!("{}Cond:", pad(i + 1));
                cond.print_tree(i + 2);

                println!("{}Then:", pad(i + 1));
                then_br.print_tree(i + 2);

                if let Some(e) = else_br {
                    println!("{}Else:", pad(i + 1));
                    e.print_tree(i + 2);
                }
            }
            Stmt::While { cond, body } => {
                println!("{p}While");
                cond.print_tree(i + 1);
                body.print_tree(i + 1);
            }
            Stmt::DoWhile { body, cond } => {
                println!("{p}DoWhile");
                body.print_tree(i + 1);
                cond.print_tree(i + 1);
            }
            Stmt::For {
                init,
                cond,
                update,
                body,
            } => {
                println!("{p}For");
                let p2 = pad(i + 1);

                match init {
                    ForInit::Expr(e) => {
                        println!("{p2}Init:");
                        e.print_tree(i + 2);
                    }
                    ForInit::Empty => println!("{p2}Init: <empty>"),
                }

                if let Some(c) = cond {
                    println!("{p2}Cond:");
                    c.print_tree(i + 2);
                }
                if let Some(u) = update {
                    println!("{p2}Update:");
                    u.print_tree(i + 2);
                }

                println!("{p2}Body:");
                body.print_tree(i + 2);
            }
            Stmt::Return(e) => {
                println!("{p}Return");
                if let Some(e) = e {
                    e.print_tree(i + 1);
                }
            }
            Stmt::Expr(e) => {
                println!("{p}ExprStmt");
                e.print_tree(i + 1);
            }
            Stmt::VarDecl(d) => d.print_tree(i),
            Stmt::Empty => println!("{p}Empty"),
        }
    }
}

impl Program {
    pub fn print_tree(&self) {
        println!("Program");

        for item in &self.items {
            match item {
                TopLevel::Func(f) => {
                    println!("{}Func: {} -> {}", pad(1), f.name, f.return_ty.display());

                    println!("{}Params:", pad(2));
                    for p in &f.params {
                        println!("{}{} : {}", pad(3), p.name, p.ty.display());
                    }

                    println!("{}Body:", pad(2));
                    for s in &f.body {
                        s.print_tree(3);
                    }
                }
                TopLevel::Var(v) => v.print_tree(1),
            }
        }
    }
}
