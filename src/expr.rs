use std::num::{Zero, One};
use nalgebra::traits::scalar_op::ScalarMul; // FIXME: implement other traits
use nalgebra::traits::dot::Dot;
use nalgebra::traits::norm::Norm;
use kernel;
use indent::Indent;
use cl_logic::{ClEq, ClOrd};
use cl_type::CLType;

pub enum Location
{
  Global,
  Local,
  Private,
  Const, // FIXME: this is not really a location but a qualifier…
  Nowhere
}

impl ToStr for Location
{
  fn to_str(&self) -> ~str
  {
    match *self
    {
      Global  => ~"__global ",
      Local   => ~"__local ",
      Private => ~"__private ",
      Const   => ~"const ",
      Nowhere => ~""
    }
  }
}

pub trait Expr
{
  fn to_cl_str(&self, indent: &mut Indent) -> ~str;
}

// XXX: make all the constructors private!
pub enum UntypedExpr<T>
{
  Param(~str, Location),
  Declare(~str, Location),
  Assign(LValue<T>, @TypedExpr<T>),
  // FIXME: useful? WildCardAssign(~TypedExpr<T>),
  StrExpr(~str)
}

pub enum TypedExpr<T>
{
  RValue(RValue<T>),
  LValue(LValue<T>, @mut kernel::Kernel)
}

pub enum LValue<T>
{
  // LValue
  LVariable(~str, Location),
  LIndexed(@Expr, @TypedExpr<i32>),
  LStrExpr(~str), // NOTE: unsafe
}

impl<T> Clone for LValue<T>
{
  fn clone(&self) -> LValue<T>
  {
    match *self
    {
      LVariable(ref a, b) => LVariable(a.clone(), b),
      LIndexed(a, b)      => LIndexed(a, b),
      LStrExpr(ref a)     => LStrExpr(a.clone()), // NOTE: unsafe
    }
  }
}

pub enum RValue<T>
{
  RIndexed(@Expr, @TypedExpr<i32>),
  RLiteral(T),
  RStrExpr(~str), // NOTE: unsafe
  ParenthesedOp(@Expr)
}

struct UnaryOperation<N1, N2>
{
  val: @TypedExpr<N1>,
  op:  UnOp
}

impl<N1, N2> UnaryOperation<N1, N2>
{
  pub fn new(val: @TypedExpr<N1>, op: UnOp) -> UnaryOperation<N1, N2>
  {
    UnaryOperation {
      val: val,
      op:  op
    }
  }
}

struct BinaryOperation<N1, N2, N3>
{
  val1: @TypedExpr<N1>,
  val2: @TypedExpr<N2>,
  op:   BinOp
}

impl<N1, N2, N3> BinaryOperation<N1, N2, N3>
{
  pub fn new(val1: @TypedExpr<N1>, val2: @TypedExpr<N2>, op: BinOp) -> BinaryOperation<N1, N2, N3>
  {
    BinaryOperation {
      val1: val1,
      val2: val2,
      op:   op
    }
  }
}

struct TernaryOperation<N1, N2, N3, N4>
{
  val1: @TypedExpr<N1>,
  val2: @TypedExpr<N2>,
  val3: @TypedExpr<N3>,
  op:   TernOp
}

impl<N1, N2, N3, N4> TernaryOperation<N1, N2, N3, N4>
{
  pub fn new(val1: @TypedExpr<N1>,
             val2: @TypedExpr<N2>,
             val3: @TypedExpr<N3>,
             op: TernOp) -> TernaryOperation<N1, N2, N3, N4>
  {
    TernaryOperation {
      val1: val1,
      val2: val2,
      val3: val3,
      op:   op
    }
  }
}

enum UnOp
{
  Normalize,
  Length
}

enum BinOp
{
  Plus,
  Minus,
  Multiply,
  Divide,
  Leq,      // <=
  Geq,      // >=
  Lstrict,  // <
  Gstrict,  // >
  Estrict,  // ==
  NEstrict, // !=
  Dot,
  Min,
  Max
}

enum TernOp
{
  Clamp
}

impl<T: CLType> Expr for UntypedExpr<T>
{
  fn to_cl_str(&self, indent: &mut Indent) -> ~str
  {
    indent.to_str() +
    match *self
    {
      Param(ref name,   ref location) => location.to_str() + CLType::to_cl_type_str::<T>() + " " + *name,
      Declare(ref name, ref location) => location.to_str() + CLType::to_cl_type_str::<T>() + " " + *name + ";",
      Assign(ref left,  ref right)    => left.to_cl_str(indent) + " = " + right.to_cl_str(indent) + ";",
      StrExpr(ref s)                  => s.clone()
    }
  }
}

impl<T: CLType> Expr for TypedExpr<T>
{
  fn to_cl_str(&self, indent: &mut Indent) -> ~str
  {
    match *self
    {
      RValue(ref rval)    => rval.to_cl_str(indent),
      LValue(ref lval, _) => lval.to_cl_str(indent)
    }
  }
}

impl<T: CLType> Expr for RValue<T>
{
  fn to_cl_str(&self, indent: &mut Indent) -> ~str
  {
    match *self
    {
      RIndexed(ref val, ref idx) => val.to_cl_str(indent) + "[" + idx.to_cl_str(indent) + "]",
      RLiteral(ref val)          => val.to_cl_literal_str(),
      RStrExpr(ref expr)         => expr.clone(),
      ParenthesedOp(ref expr)    => "(" + expr.to_cl_str(indent) + ")"
    }
  }
}

impl<T> Expr for LValue<T>
{
  fn to_cl_str(&self, indent: &mut Indent) -> ~str
  {
    match *self
    {
      LIndexed(ref val, ref idx) => val.to_cl_str(indent) + "[" + idx.to_cl_str(indent) + "]",
      LVariable(ref name, _)     => name.clone(),
      LStrExpr(ref expr)         => expr.clone()
    }
  }
}

impl<N1: CLType, N2> Expr for UnaryOperation<N1, N2>
{
  fn to_cl_str(&self, indent: &mut Indent) -> ~str
  {
    let v1 = self.val.to_cl_str(indent);

    match self.op
    {
      Length    => "length("    + v1 + ")",
      Normalize => "normalize(" + v1 + ")",
    }
  }
}

impl<N1: CLType, N2: CLType, N3> Expr for BinaryOperation<N1, N2, N3>
{
  fn to_cl_str(&self, indent: &mut Indent) -> ~str
  {
    let v1 = self.val1.to_cl_str(indent);
    let v2 = self.val2.to_cl_str(indent);

    match self.op
    {
      Plus     => v1 + " + "  + v2,
      Minus    => v1 + " - "  + v2,
      Multiply => v1 + " * "  + v2,
      Divide   => v1 + " / "  + v2,
      Leq      => v1 + " <= " + v2,
      Geq      => v1 + " >= " + v2,
      Lstrict  => v1 + " < "  + v2,
      Gstrict  => v1 + " > "  + v2,
      Estrict  => v1 + " == " + v2,
      NEstrict => v1 + " != " + v2,
      Dot      => "dot(" + v1 + ", " + v2 + ")",
      Min      => "min(" + v1 + ", " + v2 + ")",
      Max      => "max(" + v1 + ", " + v2 + ")"
    }
  }
}

impl<N1: CLType, N2: CLType, N3: CLType, N4> Expr for TernaryOperation<N1, N2, N3, N4>
{
  fn to_cl_str(&self, indent: &mut Indent) -> ~str
  {
    let v1 = self.val1.to_cl_str(indent);
    let v2 = self.val2.to_cl_str(indent);
    let v3 = self.val3.to_cl_str(indent);

    match self.op
    {
      Clamp => "clamp(" + v1 + ", " + v2 + ", " + v3 + ")"
    }
  }
}

impl<T: 'static + CLType> Index<@TypedExpr<i32>, @TypedExpr<T>> for @TypedExpr<~[T]>
{
  fn index(&self, idx: &@TypedExpr<i32>) -> @TypedExpr<T>
  {
    match **self
    {
      LValue(_, parent) => @LValue(LIndexed(*self as @Expr, *idx), parent),
      RValue(_)         => @RValue(RIndexed(*self as @Expr, *idx))
    }
  }
}

impl<T: 'static + CLType> TypedExpr<T>
{
  pub fn assign(@self, val: @TypedExpr<T>) -> @UntypedExpr<T>
  {
    match *self
    {
      LValue(ref l, parent) => {
        let res = @Assign(l.clone(), val);

        parent.push_expr(res);

        res
      },
      RValue(_) => fail!("Cannot assign an rvalue.")
    }
  }
}

impl kernel::Kernel
{
  pub unsafe fn untyped_str(@mut self, string: ~str) -> @UntypedExpr<u32>
  {
    let res = @StrExpr(string);

    self.push_expr(res);

    res
  }

  pub unsafe fn lval_str<T: 'static>(@mut self, string: ~str) -> @TypedExpr<T>
  { @LValue(LStrExpr(string), self) }
  
  pub unsafe fn rval_str<T: 'static>(@mut self, string: ~str) -> @TypedExpr<T>
  { @RValue(RStrExpr(string)) }
}

pub fn literal<T: 'static>(val: T) -> @TypedExpr<T>
{ @RValue(RLiteral(val)) }

/*
 * Impl math operations
 */
impl<N: Zero> Zero for TypedExpr<N>
{
  fn zero() -> TypedExpr<N>
  { RValue(RLiteral(Zero::zero())) }

  fn is_zero(&self) -> bool
  { fail!("is_zero cannot be evaluated on an openCL cl-expression.") }
}

impl<N: 'static + One> One for @TypedExpr<N>
{
  fn one() -> @TypedExpr<N>
  { @RValue(RLiteral(One::one())) }
}

impl<N1: 'static + Add<N2, N3> + CLType, N2: 'static + CLType, N3: 'static>
Add<@TypedExpr<N2>, @TypedExpr<N3>> for @TypedExpr<N1>
{
  pub fn add(&self, other: &@TypedExpr<N2>) -> @TypedExpr<N3>
  { @RValue(ParenthesedOp(@BinaryOperation::new::<N1, N2, N3>(*self, *other, Plus) as @Expr)) }
}

impl<N1: 'static + Sub<N2, N3> + CLType, N2: 'static + CLType, N3: 'static>
Sub<@TypedExpr<N2>, @TypedExpr<N3>> for @TypedExpr<N1>
{
  pub fn sub(&self, other: &@TypedExpr<N2>) -> @TypedExpr<N3>
  { @RValue(ParenthesedOp(@BinaryOperation::new::<N1, N2, N3>(*self, *other, Minus) as @Expr)) }
}

impl<N1: 'static + Mul<N2, N3> + CLType, N2: 'static + CLType, N3: 'static>
Mul<@TypedExpr<N2>, @TypedExpr<N3>> for @TypedExpr<N1>
{
  pub fn mul(&self, other: &@TypedExpr<N2>) -> @TypedExpr<N3>
  { @RValue(ParenthesedOp(@BinaryOperation::new::<N1, N2, N3>(*self, *other, Multiply) as @Expr)) }
}

impl<N1: 'static + Div<N2, N3> + CLType, N2: 'static + CLType, N3: 'static>
Div<@TypedExpr<N2>, @TypedExpr<N3>> for @TypedExpr<N1>
{
  pub fn div(&self, other: &@TypedExpr<N2>) -> @TypedExpr<N3>
  { @RValue(ParenthesedOp(@BinaryOperation::new::<N1, N2, N3>(*self, *other, Divide) as @Expr)) }
}

impl<N: 'static + CLType, V: 'static + CLType + ScalarMul<N>> ScalarMul<@TypedExpr<N>> for @TypedExpr<V>
{
  pub fn scalar_mul(&self, val: &@TypedExpr<N>) -> @TypedExpr<V>
  { @RValue(ParenthesedOp(@BinaryOperation::new::<V, N, V>(*self, *val, Multiply) as @Expr)) }

  pub fn scalar_mul_inplace(&mut self, _: &@TypedExpr<N>)
  {
    fail!("Not yet implemented.");
    // @RValue(ParenthesedOp(@BinaryOperation::new::<V, N, V>(*self, *val, Multiply) as @Expr))
  }
}

impl<N: 'static + Eq + CLType> ClEq<@TypedExpr<bool>> for @TypedExpr<N>
{
  pub fn cl_eq(&self, other: &@TypedExpr<N>) -> @TypedExpr<bool>
  { @RValue(ParenthesedOp(@BinaryOperation::new::<N, N, bool>(*self, *other, Estrict) as @Expr)) }

  pub fn cl_ne(&self, other: &@TypedExpr<N>) -> @TypedExpr<bool>
  { @RValue(ParenthesedOp(@BinaryOperation::new::<N, N, bool>(*self, *other, NEstrict) as @Expr)) }
}

impl<N: 'static + Ord + CLType> ClOrd<@TypedExpr<bool>> for @TypedExpr<N>
{
  pub fn cl_ge(&self, other: &@TypedExpr<N>) -> @TypedExpr<bool>
  { @RValue(ParenthesedOp(@BinaryOperation::new::<N, N, bool>(*self, *other, Geq) as @Expr)) }

  pub fn cl_gt(&self, other: &@TypedExpr<N>) -> @TypedExpr<bool>
  { @RValue(ParenthesedOp(@BinaryOperation::new::<N, N, bool>(*self, *other, Gstrict) as @Expr)) }

  pub fn cl_le(&self, other: &@TypedExpr<N>) -> @TypedExpr<bool>
  { @RValue(ParenthesedOp(@BinaryOperation::new::<N, N, bool>(*self, *other, Leq) as @Expr)) }

  pub fn cl_lt(&self, other: &@TypedExpr<N>) -> @TypedExpr<bool>
  { @RValue(ParenthesedOp(@BinaryOperation::new::<N, N, bool>(*self, *other, Lstrict) as @Expr)) }
}

impl<V: 'static + Dot<N> + CLType, N: 'static + CLType> Dot<@TypedExpr<N>> for @TypedExpr<V>
{
  fn dot(&self, other: &@TypedExpr<V>) -> @TypedExpr<N>
  { @RValue(ParenthesedOp(@BinaryOperation::new::<V, V, N>(*self, *other, Dot) as @Expr)) }
}

impl<N: 'static + Orderable + CLType> Orderable for @TypedExpr<N>
{
  fn min(&self, other: &@TypedExpr<N>) -> @TypedExpr<N>
  { @RValue(ParenthesedOp(@BinaryOperation::new::<N, N, N>(*self, *other, Min) as @Expr)) }

  fn max(&self, other: &@TypedExpr<N>) -> @TypedExpr<N>
  { @RValue(ParenthesedOp(@BinaryOperation::new::<N, N, N>(*self, *other, Max) as @Expr)) }

  fn clamp(&self, mn: &@TypedExpr<N>, mx: &@TypedExpr<N>) -> @TypedExpr<N>
  { @RValue(ParenthesedOp(@TernaryOperation::new::<N, N, N, N>(*self, *mn, *mx, Clamp) as @Expr)) }
}

impl<V: 'static + Norm<N> + CLType, N: 'static + CLType> Norm<@TypedExpr<N>> for @TypedExpr<V>
{
  fn norm(&self) -> @TypedExpr<N>
  {
    @RValue(ParenthesedOp(@UnaryOperation::new::<V, N>(*self, Length) as @Expr))
  }

  fn sqnorm(&self) -> @TypedExpr<N>
  {
    fail!("Not yet implemented.");
  }

  fn normalized(&self) -> @TypedExpr<V>
  {
    @RValue(ParenthesedOp(@UnaryOperation::new::<V, V>(*self, Normalize) as @Expr))
  }

  fn normalize(&mut self) -> @TypedExpr<N>
  {
    fail!("Not yet implemented.");
  }
}
