use leo_ast::{ExpressionVisitor, GroupValue};
use snarkvm_bytecode::{
    instructions::*,
    parsers::{BinaryOperation, Operand, UnaryOperation},
    register::Register,
    Program, Value,
};
use snarkvm_circuits::{
    Address, Boolean, Field, Group, Literal, Parser, StringType, I128, I16, I32, I64, I8, U128, U16, U32, U64, U8,
};

use crate::IrEmitter;

impl<'a, P: Program> IrEmitter<'a, P> {
    pub fn emit_expr(&mut self, expr: &'a leo_ast::Expression) -> Operand<P> {
        match expr {
            leo_ast::Expression::Identifier(r) => Operand::Register(self.find_register(r)),
            leo_ast::Expression::Value(v) => Operand::Value(self.get_value(v)),
            leo_ast::Expression::Binary(e) => Operand::Register(self.emit_binary(e)),
            leo_ast::Expression::Unary(e) => Operand::Register(self.emit_unary(e)),
            leo_ast::Expression::Ternary(e) => Operand::Register(self.emit_ternary(e)),
            leo_ast::Expression::Call(_) => unimplemented!("call not implemented in IR"),
            leo_ast::Expression::Err(e) => unreachable!("errors shouldn't exist at the IR level, encountered {e:?}"),
        }
    }

    fn find_register(&self, input: &'a leo_ast::Identifier) -> Register<P> {
        self.registers
            .iter()
            .find(|i| **i == input.name)
            .unwrap()
            .register
            .clone()
    }

    fn get_value(&self, input: &'a leo_ast::ValueExpression) -> Value<P> {
        Value::Literal(match input {
            leo_ast::ValueExpression::Address(v, _) => Literal::Address(Address::parse(v).unwrap().1),
            leo_ast::ValueExpression::Boolean(v, _) => Literal::Boolean(Boolean::parse(v).unwrap().1),
            leo_ast::ValueExpression::Char(_) => unimplemented!("char not implemented in IR"),
            leo_ast::ValueExpression::Field(v, _) => Literal::Field(Field::parse(v).unwrap().1),
            leo_ast::ValueExpression::Group(g) => match &**g {
                GroupValue::Single(v, _) => Literal::Group(Group::parse(v).unwrap().1),
                // todo: is this right?
                GroupValue::Tuple(v) => Literal::Group(Group::parse(&v.to_string()).unwrap().1),
            },
            leo_ast::ValueExpression::Integer(t, s, _) => match t {
                leo_ast::IntegerType::U8 => Literal::U8(U8::parse(s).unwrap().1),
                leo_ast::IntegerType::U16 => Literal::U16(U16::parse(s).unwrap().1),
                leo_ast::IntegerType::U32 => Literal::U32(U32::parse(s).unwrap().1),
                leo_ast::IntegerType::U64 => Literal::U64(U64::parse(s).unwrap().1),
                leo_ast::IntegerType::U128 => Literal::U128(U128::parse(s).unwrap().1),
                leo_ast::IntegerType::I8 => Literal::I8(I8::parse(s).unwrap().1),
                leo_ast::IntegerType::I16 => Literal::I16(I16::parse(s).unwrap().1),
                leo_ast::IntegerType::I32 => Literal::I32(I32::parse(s).unwrap().1),
                leo_ast::IntegerType::I64 => Literal::I64(I64::parse(s).unwrap().1),
                leo_ast::IntegerType::I128 => Literal::I128(I128::parse(s).unwrap().1),
            },
            leo_ast::ValueExpression::String(s, _) => Literal::String(
                StringType::parse(
                    // todo: doesnt take into account non-scalar values, but those cant exist when read from a file anyways
                    &s.iter().map(|s| s.to_string()).collect::<Vec<_>>().join(""),
                )
                .unwrap()
                .1,
            ),
        })
    }

    fn emit_binary(&mut self, expr: &'a leo_ast::BinaryExpression) -> Register<P> {
        let first = self.emit_expr(&expr.left);
        let second = self.emit_expr(&expr.right);
        let dest = self.spawn_reg(None);

        let operation = BinaryOperation {
            first,
            second,
            destination: dest.clone(),
        };
        let instruction = match expr.op {
            leo_ast::BinaryOperation::Add => Instruction::Add(Add { operation }),
            leo_ast::BinaryOperation::Sub => Instruction::Sub(Sub { operation }),
            leo_ast::BinaryOperation::Mul => Instruction::Mul(Mul { operation }),
            leo_ast::BinaryOperation::Div => Instruction::Div(Div { operation }),
            leo_ast::BinaryOperation::Pow => Instruction::Pow(Pow { operation }),
            leo_ast::BinaryOperation::Or => Instruction::Or(Or { operation }),
            leo_ast::BinaryOperation::And => Instruction::And(And { operation }),
            leo_ast::BinaryOperation::Eq => Instruction::Equal(Equal { operation }),
            leo_ast::BinaryOperation::Ne => Instruction::NotEqual(NotEqual { operation }),
            leo_ast::BinaryOperation::Ge => Instruction::GreaterThanOrEqual(GreaterThanOrEqual { operation }),
            leo_ast::BinaryOperation::Gt => Instruction::GreaterThan(GreaterThan { operation }),
            leo_ast::BinaryOperation::Le => Instruction::LessThanOrEqual(LessThanOrEqual { operation }),
            leo_ast::BinaryOperation::Lt => Instruction::LessThan(LessThan { operation }),
        };

        self.emit(instruction);
        dest
    }

    fn emit_unary(&mut self, expr: &'a leo_ast::UnaryExpression) -> Register<P> {
        let first = self.emit_expr(&expr.inner);
        let dest = self.spawn_reg(None);

        let operation = UnaryOperation {
            first,
            destination: dest.clone(),
        };
        let instruction = match expr.op {
            leo_ast::UnaryOperation::Not => Instruction::Not(Not { operation }),
            leo_ast::UnaryOperation::Negate => Instruction::Neg(Neg { operation }),
        };

        self.emit(instruction);
        dest
    }

    fn emit_ternary(&mut self, _expr: &'a leo_ast::TernaryExpression) -> Register<P> {
        unimplemented!("ternary not implemented in IR")
    }
}

impl<'a, P: Program> ExpressionVisitor<'a> for IrEmitter<'a, P> {
    fn visit_call(&mut self, _input: &'a leo_ast::CallExpression) -> leo_ast::VisitResult {
        unimplemented!("call not implemented in IR")
    }
}
