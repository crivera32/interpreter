//
//  Author: Christian Rivera
//  Semantics Interpreter in Rust
//

#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

#[derive(Copy, Clone)]
enum ArithmeticOperator {
  PLUS,
  MINUS,
  TIMES,
  DIV,
  NONE,
}
#[derive(Copy, Clone)]
enum ComparisonOperator {
  EQ,
  NONE,
}

//=======================//
// Expression Definition //
//=======================//
// Tag
#[derive(Copy, Clone)]
enum ExpressionTag {
  INT_CONST,
  BOOL_CONST,
  BIN_OP,
  COMP,
  IF,
  LET,
  VARIABLE,
  FUNC_DECLARATION,
  FUNC_CALL,
}
#[derive( Clone)]
struct Expression {
  tag: ExpressionTag, // Expression type
  body: Vec<Expression>, // The operands or the rest of the program
  intConst: i32, // For integer constants
  boolConst: bool, // For boolean constants
  op: ArithmeticOperator, // For arithmetic operations
  comp: ComparisonOperator, // For comparisons
  name: String, // For variable reads and writes
  argNames: Vec<String>, // Argument names for functions
}
impl Expression {
    // Constructors
    fn integerConstant(value: i32) -> Expression {
      Expression {
        tag: ExpressionTag::INT_CONST,
        body: Vec::new(),
        intConst: value,
        boolConst: false,
        op: ArithmeticOperator::NONE,
        comp: ComparisonOperator::NONE,
        name: String::new(),
        argNames: Vec::new(),
      }
    }
    fn booleanConstant(value: bool) -> Expression {
      Expression {
        tag: ExpressionTag::BOOL_CONST,
        body: Vec::new(),
        intConst: 0,
        boolConst: value,
        op: ArithmeticOperator::NONE,
        comp: ComparisonOperator::NONE,
        name: String::new(),
        argNames: Vec::new(),
      }
    }
    fn binaryOperation(operator: ArithmeticOperator, left: Expression, right: Expression) -> Expression {
      let mut operands : Vec<Expression> = Vec::new();
      operands.push(left);
      operands.push(right);

      Expression {
        tag: ExpressionTag::BIN_OP,
        body: operands,
        intConst: 0,
        boolConst: false,
        op: operator,
        comp: ComparisonOperator::NONE,
        name: String::new(),
        argNames: Vec::new(),
      }
    }
    fn comparison(comparison: ComparisonOperator, left: Expression, right: Expression) -> Expression {
      let mut operands : Vec<Expression> = Vec::new();
      operands.push(left);
      operands.push(right);

      Expression {
        tag: ExpressionTag::COMP,
        body: operands,
        intConst: 0,
        boolConst: false,
        op: ArithmeticOperator::NONE,
        comp: comparison,
        name: String::new(),
        argNames: Vec::new(),
      }
    }
    fn ifBranch(condition : Expression, thenSide: Expression, elseSide: Expression ) -> Expression {
      let mut operands : Vec<Expression> = Vec::new();
      operands.push(condition);
      operands.push(thenSide);
      operands.push(elseSide);

      Expression {
        tag: ExpressionTag::IF,
        body: operands,
        intConst: 0,
        boolConst: false,
        op: ArithmeticOperator::NONE,
        comp: ComparisonOperator::NONE,
        name: String::new(),
        argNames: Vec::new(),
      }
    }
    fn assignVariable(varName: String, val: Expression, body: Expression) -> Expression {
      let mut operands : Vec<Expression> = Vec::new();
      operands.push(val);
      operands.push(body);

      Expression {
        tag: ExpressionTag::LET,
        body: operands,
        intConst: 0,
        boolConst: false,
        op: ArithmeticOperator::NONE,
        comp: ComparisonOperator::NONE,
        name: varName,
        argNames: Vec::new(),
      }
    }
    fn readVariable(varName: String) -> Expression {
      Expression {
        tag: ExpressionTag::VARIABLE,
        body: Vec::new(),
        intConst: 0,
        boolConst: false,
        op: ArithmeticOperator::NONE,
        comp: ComparisonOperator::NONE,
        name: varName,
        argNames: Vec::new(),
      }
    }
    fn functionDeclaration(func: Expression, formalArgNames: Vec<String>) -> Expression {
      let mut body : Vec<Expression> = Vec::new();
      body.push(func);

      Expression {
        tag: ExpressionTag::FUNC_DECLARATION,
        body: body,
        intConst: 0,
        boolConst: false,
        op: ArithmeticOperator::NONE,
        comp: ComparisonOperator::NONE,
        name: String::new(),
        argNames: formalArgNames,
      }
    }
    fn functionCall(name: String, actualArgs: Vec<Expression>) -> Expression {
      let mut body : Vec<Expression> = Vec::new();

      for i in 0..actualArgs.len() {
        body.push(Expression::clone(&actualArgs[i]));
      }

      Expression {
        tag: ExpressionTag::FUNC_CALL,
        body: body,
        intConst: 0,
        boolConst: false,
        op: ArithmeticOperator::NONE,
        comp: ComparisonOperator::NONE,
        name: name,
        argNames: Vec::new(),
      }
    }
    // Methods
    fn getValue(&self) -> Value {
      match self.tag {
        ExpressionTag::INT_CONST => Value::integer(self.intConst),
        ExpressionTag::BOOL_CONST => Value::boolean(self.boolConst),
        ExpressionTag::FUNC_CALL => Value::function(Expression::clone(&self.body[0]), Vec::clone(&self.argNames)),
        _ => Value::null(),
      }
    }
}
// Print Function
impl std::fmt::Display for Expression {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match self.tag {
      ExpressionTag::INT_CONST => write!(f, "INT_CONST:{}", self.intConst),
      ExpressionTag::BOOL_CONST => write!(f, "BOOL_CONST:{}", self.boolConst),
      ExpressionTag::BIN_OP => {
        let op = self.op;
        match op {
          ArithmeticOperator::PLUS  => write!(f, "BIN_OP:PLUS"),
          ArithmeticOperator::MINUS => write!(f, "BIN_OP:MINUS"),
          ArithmeticOperator::TIMES => write!(f, "BIN_OP:TIMES"),
          ArithmeticOperator::DIV   => write!(f, "BIN_OP:DIV"),
          ArithmeticOperator::NONE  => write!(f, "BIN_OP:NONE"),
        }
      },
      ExpressionTag::COMP => {
        let comp = self.comp;
        match comp {
          ComparisonOperator::EQ   => write!(f, "COMP:EQ"),
          ComparisonOperator::NONE => write!(f, "COMP:NONE"),
          //_   => write!(f, "COMP:unknown"),
        }
      },
      ExpressionTag::IF => write!(f, "IF"),
      ExpressionTag::LET => write!(f, "LET:{}",&self.name),
      ExpressionTag::VARIABLE => write!(f, "VARIABLE:{}",&self.name),
      ExpressionTag::FUNC_DECLARATION => write!(f, "FUNC_DECLARATION"),
      ExpressionTag::FUNC_CALL => write!(f, "FUNC_CALL:{}", &self.name),
    }
  }
}

//==================//
// Value Definition //
//==================//
// Tag
#[derive(PartialEq)]
#[derive(Copy, Clone)]
enum ValueTag {
  INT,
  BOOL,
  FUNC,
  NULL
}
#[derive( Clone)]
struct Value {
  tag: ValueTag, // Value type
  intVal: i32, // For integer values
  boolVal: bool, // For boolean values
  funcVal: Expression, // For function values
  formalFuncArgNames: Vec<String>, // For function values
}
impl Value {
  // Constructors
  fn integer(value: i32) -> Value {
      Value {
        tag: ValueTag::INT,
        intVal: value,
        boolVal: false,
        funcVal: Expression::booleanConstant(false),
        formalFuncArgNames: Vec::new(),
      }
  }
  fn boolean(value: bool) -> Value {
      Value {
        tag: ValueTag::BOOL,
        intVal: 0,
        boolVal: value,
        funcVal: Expression::booleanConstant(false),
        formalFuncArgNames: Vec::new()
      }
  }
  fn null() -> Value {
      Value {
        tag: ValueTag::NULL,
        intVal: 0,
        boolVal: false,
        funcVal: Expression::booleanConstant(false),
        formalFuncArgNames: Vec::new()
      }
  }
  fn function(body: Expression, argNames: Vec<String>) -> Value {
    Value {
        tag: ValueTag::FUNC,
        intVal: 0,
        boolVal: false,
        funcVal: body,
        formalFuncArgNames: argNames,
      }
  }
  fn isNull(&self) -> bool {
    return self.tag == ValueTag::NULL;
  }
}
// Print Function
impl std::fmt::Display for Value {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match self.tag {
      ValueTag::INT  => write!(f, "INT:{}", self.intVal),
      ValueTag::BOOL => write!(f, "BOOL:{}", self.boolVal),
      ValueTag::FUNC => write!(f, "FUNC"),
      ValueTag::NULL => write!(f, "NULL"),
    }
  }
}

//====================//
// Binding Definition //
//====================//
#[derive(Clone)]
struct Binding {
  name: String,
  value: Value,
}
impl Binding {
  // Constructors
  fn value(n: String, v: Value) -> Binding {
    Binding{
      name: n,
      value: v,
    }
  }
  fn integer(n: String, integerVal: i32) -> Binding {
    Binding{
      name: n,
      value: Value::integer(integerVal),
    }
  }
  fn boolean(n: String, booleanVal: bool) -> Binding {
    Binding{
      name: n,
      value: Value::boolean(booleanVal),
    }
  }
  fn null(n: String) -> Binding {
    Binding{
      name: n,
      value: Value::null(),
    }
  }
}
// Print Function
impl std::fmt::Display for Binding {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    return write!(f, "[NAME({})|VALUE({})]", self.name, self.value);
  }
}

//========================//
// Environment Definition //
//========================//
#[derive( Clone)]
struct Environment {
  bindings: Vec<Binding>,
}
impl Environment {
  // Constructors
  fn new() -> Environment {
    Environment {
      bindings: Vec::new(),
    }
  }
  // Methods
  fn bind(&self, name: String, value: Value) -> Environment {
    let b : Binding = Binding::value(name, value);
    let mut newBindings : Vec<Binding> = self.bindings.clone();
    newBindings.push(b);
    Environment {
      bindings: newBindings,
    }
  }
  fn lookup(&self, name: String) -> Value {
    for i in (0..self.bindings.len()).rev() {
      if self.bindings[i].name.eq(&name) {
        return Value::clone(&self.bindings[i].value);
      }
    }
    return Value::null();
  }
  fn print(&self) {
    print!("ENVIRONMENT:\n");
    for b in &self.bindings {
      print!("\t{}\n", b);
    }
  }
}

//=====================//
// Evaluate Definition //
//=====================//
fn evaluate(program : &Expression, mut pc : usize, e : &Environment) -> (Value, usize) {
  let exp : &Expression = &program;
  print!("PC={} -> {}\n", pc, exp);
  pc = pc + 1;

  match exp.tag {
    ExpressionTag::INT_CONST => {
      let intVal : Value = exp.getValue();
      (intVal, pc)
    },
    ExpressionTag::BOOL_CONST => {
      let boolVal : Value = exp.getValue();
      (boolVal, pc)
    },
    ExpressionTag::BIN_OP => {
      let op : ArithmeticOperator =  exp.op;
      let (leftVal, pc) = evaluate(&exp.body[0], pc, e);
      let (rightVal, pc) = evaluate(&exp.body[1], pc, e);

      let left = leftVal.intVal;
      let right = rightVal.intVal;

      let res : i32;
      match op {
        ArithmeticOperator::PLUS  => (Value::integer(left+right), pc),
        ArithmeticOperator::MINUS => (Value::integer(left-right), pc),
        ArithmeticOperator::TIMES => (Value::integer(left*right), pc),
        ArithmeticOperator::DIV   => (Value::integer(left/right), pc),
        ArithmeticOperator::NONE  => (Value::null(), pc)
      }
    },
    ExpressionTag::COMP => {
      let comp : ComparisonOperator =  exp.comp;
      let (leftVal, pc) = evaluate(&exp.body[0], pc, e);
      let (rightVal, pc) = evaluate(&exp.body[1], pc, e);
 
      // Comparison between different types returns false
      if leftVal.tag != rightVal.tag {
        (Value::boolean(false), pc)
      }
      else {
        match leftVal.tag {
          ValueTag::INT => {
            let left : i32 = leftVal.intVal;
            let right : i32 = rightVal.intVal;   
            (Value::boolean(left==right), pc)
          }
          ValueTag::BOOL => {
            let left : bool = leftVal.boolVal;
            let right : bool = rightVal.boolVal;   
            (Value::boolean(left==right), pc)
          }
          ValueTag::NULL => {
            // NULL==NULL evaluates to true
            (Value::boolean(true), pc)
          }
          ValueTag::FUNC => {
            (Value::boolean(false), pc)
          }
        }
      }
    },
    ExpressionTag::IF => {
      let (condVal, pc) = evaluate(&exp.body[0], pc, e);
      if condVal.boolVal==true {
        let (leftVal, pc) = evaluate(&exp.body[1], pc, e);
        (leftVal, pc)
      }
      else {
        let (rightVal, pc) = evaluate(&exp.body[2], pc, e);
        (rightVal, pc)
      }
    },
    ExpressionTag::LET => {
      let name : &String = &exp.name;
      let (value, pc) = evaluate(&exp.body[0], pc, e);

      let newE = e.bind(name.to_string(), value);

      let (res, pc) = evaluate(&exp.body[1], pc, &newE);
      (res, pc)
    },
    ExpressionTag::VARIABLE => {
      let name : &String = &exp.name;
      let value = e.lookup(name.to_string());
      (value, pc)
    },
    ExpressionTag::FUNC_DECLARATION => {
      let formalArgNames : Vec<String> = Vec::clone(&exp.argNames);
      let funcBody : Expression = Expression::clone(&exp.body[0]);

      let retVal : Value = Value::function(funcBody, formalArgNames);
      (retVal, pc)
    },
    ExpressionTag::FUNC_CALL => {
      let name : &String = &exp.name;
      let functionValue = e.lookup(name.to_string());
      let theFunction : Expression = functionValue.funcVal;
      let formalArgNames : Vec<String> = functionValue.formalFuncArgNames;

      let mut evalEnv : Environment = Environment::clone(e);
      for i in 0..exp.body.len() {
        let (thisArgValue, pc) = evaluate(&exp.body[i], pc, &e);
        evalEnv = evalEnv.bind(formalArgNames[i].to_string(),thisArgValue);
      }

      let (ret, pc) = evaluate(&theFunction, pc, &evalEnv);
      (ret, pc)
    },
  }
}

//========//
//  Main  //
//========//
fn main() {

  // p1
  let p1 : Expression = Expression::integerConstant(474);

  // p2
  let p2 : Expression = Expression::binaryOperation(
    ArithmeticOperator::DIV,
    Expression::binaryOperation(
      ArithmeticOperator::PLUS,
      Expression::integerConstant(400),
      Expression::integerConstant(74)
    ),
    Expression::integerConstant(3)
  );

  // p3
  let p3 : Expression = Expression::comparison(
    ComparisonOperator::EQ,
    Expression::binaryOperation(
      ArithmeticOperator::DIV,
      Expression::binaryOperation(
        ArithmeticOperator::PLUS,
        Expression::integerConstant(400),
        Expression::integerConstant(74)
      ),
      Expression::integerConstant(3)
    ),
    Expression::integerConstant(158)
  );

  // p4
  // if (((400 + 74) / 3) == 158) then 474
  // else 474/0
  let p4 : Expression = Expression::ifBranch(
    Expression::comparison(
      ComparisonOperator::EQ,
      Expression::binaryOperation(
        ArithmeticOperator::DIV,
        Expression::binaryOperation(
          ArithmeticOperator::PLUS,
          Expression::integerConstant(400),
          Expression::integerConstant(74)
        ),
        Expression::integerConstant(3)
      ),
      Expression::integerConstant(158)
    ),
    Expression::integerConstant(474),
    Expression::binaryOperation(
      ArithmeticOperator::DIV,
      Expression::integerConstant(474),
      Expression::integerConstant(0)
    )
  );

  //let bot = 3 in
  //  (let bot = 2 in bot)
  //  +
  //  (if (bot == 0) then 474/0 else (400+74)/bot)
  let p5 : Expression = Expression::assignVariable(
    "bot".to_string(),
    Expression::integerConstant(3),
    Expression::binaryOperation(
      ArithmeticOperator::PLUS,
      Expression::assignVariable(
        "bot".to_string(),
        Expression::integerConstant(2),
        Expression::readVariable("bot".to_string())
      ),
      Expression::ifBranch(
        Expression::comparison(
          ComparisonOperator::EQ,
          Expression::readVariable("bot".to_string()),
          Expression::integerConstant(0)
        ),
        Expression::binaryOperation(
          ArithmeticOperator::DIV,
          Expression::integerConstant(474),
          Expression::integerConstant(0)
        ),
        Expression::binaryOperation(
          ArithmeticOperator::DIV,
          Expression::binaryOperation(
            ArithmeticOperator::PLUS,
            Expression::integerConstant(400),
            Expression::integerConstant(74)
          ),
          Expression::readVariable("bot".to_string())
        )
      )
    )
  );

  // p6
  /*
  function f(top,bot) :
    if (bot == 0) then 0 else top/bot

    let bot = 3 in
      (let bot = 2 in bot)
      +
      (f(400+74,bot) + f(470+4,0))
  */
  let p6 : Expression = Expression::assignVariable(
    "f".to_string(),
    Expression::functionDeclaration(
      Expression::ifBranch(
        Expression::comparison(
          ComparisonOperator::EQ,
          Expression::readVariable("bot".to_string()),
          Expression::integerConstant(0)
        ),
        Expression::integerConstant(0),
        Expression::binaryOperation(
          ArithmeticOperator::DIV,
          Expression::readVariable("top".to_string()),
          Expression::readVariable("bot".to_string())
        )
      ),
      vec![
        "top".to_string(),
        "bot".to_string()
      ]
    ),
    Expression::assignVariable(
      "bot".to_string(),
      Expression::integerConstant(3),
      Expression::binaryOperation(
        ArithmeticOperator::PLUS,
        Expression::assignVariable(
          "bot".to_string(),
          Expression::integerConstant(2),
          Expression::readVariable("bot".to_string())
        ),
        Expression::binaryOperation(
          ArithmeticOperator::PLUS,
          Expression::functionCall(
            "f".to_string(),
            vec![
              Expression::binaryOperation(
                ArithmeticOperator::PLUS,
                Expression::integerConstant(400),
                Expression::integerConstant(74)
              ),
              Expression::readVariable("bot".to_string())
            ]
          ),
          Expression::functionCall(
            "f".to_string(),
            vec![
              Expression::binaryOperation(
                ArithmeticOperator::PLUS,
                Expression::integerConstant(470),
                Expression::integerConstant(4)
              ),
              Expression::integerConstant(0)
            ]
          )
        )
      )
    )
  );

  let (res, pc) = evaluate(&p6, 0, &Environment::new());
  print!(">>> Result: {} | PC: {}\n", res, pc);
}