use enum_iterator::Sequence;

use crate::{Function, Instr, Primitive, Shape, TempStack, Uiua, Value};

#[derive(Debug, Clone)]
enum Type {
    Num,
    Complex,
    Char,
    Box,
}

#[derive(Debug, Clone)]
struct Form {
    ty: Type,
    shape: Shape,
}

impl Form {
    fn new(ty: Type, shape: Shape) -> Self {
        Self { ty, shape }
    }
}

struct FormEnv {
    stack: Vec<Form>,
    temp_stacks: [Vec<Form>; TempStack::CARDINALITY],
    function_stack: Vec<Function>,
}

struct FormError;
type FormResult<T = ()> = Result<T, FormError>;

impl FormEnv {
    fn instrs(&mut self, instrs: &[Instr], env: &Uiua) -> FormResult {
        for instr in instrs {
            self.instr(instr, env)?;
        }
        Ok(())
    }
    fn instr(&mut self, instr: &Instr, env: &Uiua) -> FormResult {
        match instr {
            Instr::Comment(_) => {}
            Instr::Push(val) => {
                let ty = match val {
                    Value::Num(_) => Type::Num,
                    #[cfg(feature = "bytes")]
                    Value::Byte(_) => Type::Num,
                    Value::Char(_) => Type::Char,
                    Value::Complex(_) => Type::Complex,
                    Value::Box(_) => Type::Box,
                };
                self.stack.push(Form::new(ty, val.shape().into()));
            }
            Instr::CallGlobal { index, call, sig } => return Err(FormError),
            Instr::BindGlobal { name, span, index } => return Err(FormError),
            Instr::BeginArray => return Err(FormError),
            Instr::EndArray { boxed, span } => return Err(FormError),
            Instr::Prim(_, _) => return Err(FormError),
            Instr::ImplPrim(_, _) => return Err(FormError),
            Instr::Call(_) => return Err(FormError),
            Instr::PushFunc(f) => self.function_stack.push(f.clone()),
            Instr::Switch { count, sig, span } => return Err(FormError),
            Instr::Format(_, _) => return Err(FormError),
            Instr::Dynamic(_) => return Err(FormError),
            Instr::Unpack { count, span, unbox } => {
                let mut form = self.pop()?;
                if !form.shape.is_empty() {
                    form.shape.remove(0);
                }
                for _ in 0..*count {
                    self.stack.push(form.clone());
                }
            }
            Instr::PushTempFunctions(_) => return Err(FormError),
            Instr::PopTempFunctions(_) => return Err(FormError),
            Instr::GetTempFunction { offset, sig, span } => return Err(FormError),
            Instr::TouchStack { count, span } => return Err(FormError),
            Instr::PushTemp { stack, count, span } => return Err(FormError),
            Instr::PopTemp { stack, count, span } => return Err(FormError),
            Instr::CopyToTemp { stack, count, span } => return Err(FormError),
            Instr::CopyFromTemp {
                stack,
                offset,
                count,
                span,
            } => return Err(FormError),
            Instr::DropTemp { stack, count, span } => return Err(FormError),
            Instr::SetOutputComment { i, n } => return Err(FormError),
            Instr::PushSig(_) => {}
            Instr::PopSig => {}
        }
        Ok(())
    }
    fn prim(&mut self, prim: Primitive, env: &Uiua) -> FormResult {
        match prim {
            _ => return Err(FormError),
        }
        Ok(())
    }
    fn pop(&mut self) -> FormResult<Form> {
        self.stack.pop().ok_or(FormError)
    }
}
