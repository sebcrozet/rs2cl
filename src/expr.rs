use std::num::{Zero, One};
use nalgebra::traits::scalar_op::ScalarMul; // FIXME: implement other traits
use nalgebra::traits::dot::Dot;
use kernel::Kernel;
use cl_logic::{ClEq, ClOrd};

pub enum Location
{
  Global,
  Local,
  Private,
  Nowhere
}

pub trait Expr
{ }

// XXX: make all the constructors private!
pub enum UntypedExpr<T>
{
  Declaration(~str, Location),
  Assign(LValue<T>, @TypedExpr<T>),
  WildCardAssign(~TypedExpr<T>),
  StrExpr(~str)
}

pub enum TypedExpr<T>
{
  RValue(RValue<T>),
  LValue(LValue<T>)
}

pub enum LValue<T>
{
  // LValue
  LVariable(~str, Location, @mut Kernel),
  LIndexed(@Expr, @TypedExpr<uint>),
  LStrExpr(~str), // NOTE: unsafe
}

pub enum RValue<T>
{
  RIndexed(@Expr, @TypedExpr<uint>),
  RLiteral(T),
  RStrExpr(~str), // NOTE: unsafe
  ParenthesedOp(@Expr)
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

impl<T> Expr for UntypedExpr<T>
{ }

impl<T> Expr for TypedExpr<T>
{ }

impl<N1, N2, N3> Expr for BinaryOperation<N1, N2, N3>
{ }

impl<N1, N2, N3, N4> Expr for TernaryOperation<N1, N2, N3, N4>
{ }

impl<T: 'static> Index<@TypedExpr<uint>, @TypedExpr<T>> for @TypedExpr<~[T]>
{
  fn index(&self, idx: &@TypedExpr<uint>) -> @TypedExpr<T>
  {
    if self.is_lvalue()
    { @LValue(LIndexed(*self as @Expr, *idx)) }
    else
    { @RValue(RIndexed(*self as @Expr, *idx)) }
  }
}

impl<T> TypedExpr<T>
{
  pub fn assign(@self, val: @TypedExpr<T>) -> @UntypedExpr<T>
  {
    match *self
    {
      LValue(ref l) => @Assign(copy* l, val),
      RValue(_)     => fail!("Cannot assign an rvalue.")
    }
  }

  pub fn is_lvalue(&self) -> bool
  {
    match *self
    {
      RValue(_) => false,
      LValue(_) => true,
    }
  }
}

pub unsafe fn untyped_str(string: ~str) -> @UntypedExpr<()>
{ @StrExpr(string) }

pub unsafe fn lval_str<T>(string: ~str) -> @TypedExpr<T>
{ @LValue(LStrExpr(string)) }

pub unsafe fn rval_str<T>(string: ~str) -> @TypedExpr<T>
{ @RValue(RStrExpr(string)) }

pub fn literal<T>(val: T) -> @TypedExpr<T>
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

impl<N: One> One for @TypedExpr<N>
{
  fn one() -> @TypedExpr<N>
  { @RValue(RLiteral(One::one())) }
}

impl<N1: 'static + Add<N2, N3>, N2: 'static, N3: 'static>
Add<@TypedExpr<N2>, @TypedExpr<N3>> for @TypedExpr<N1>
{
  pub fn add(&self, other: &@TypedExpr<N2>) -> @TypedExpr<N3>
  { @RValue(ParenthesedOp(@BinaryOperation::new::<N1, N2, N3>(*self, *other, Plus) as @Expr)) }
}

impl<N1: 'static + Sub<N2, N3>, N2: 'static, N3: 'static>
Sub<@TypedExpr<N2>, @TypedExpr<N3>> for @TypedExpr<N1>
{
  pub fn sub(&self, other: &@TypedExpr<N2>) -> @TypedExpr<N3>
  { @RValue(ParenthesedOp(@BinaryOperation::new::<N1, N2, N3>(*self, *other, Minus) as @Expr)) }
}

impl<N1: 'static + Mul<N2, N3>, N2: 'static, N3: 'static>
Mul<@TypedExpr<N2>, @TypedExpr<N3>> for @TypedExpr<N1>
{
  pub fn mul(&self, other: &@TypedExpr<N2>) -> @TypedExpr<N3>
  { @RValue(ParenthesedOp(@BinaryOperation::new::<N1, N2, N3>(*self, *other, Multiply) as @Expr)) }
}

impl<N1: 'static + Div<N2, N3>, N2: 'static, N3: 'static>
Div<@TypedExpr<N2>, @TypedExpr<N3>> for @TypedExpr<N1>
{
  pub fn div(&self, other: &@TypedExpr<N2>) -> @TypedExpr<N3>
  { @RValue(ParenthesedOp(@BinaryOperation::new::<N1, N2, N3>(*self, *other, Divide) as @Expr)) }
}

impl<N: 'static, V: 'static> ScalarMul<@TypedExpr<N>> for @TypedExpr<V>
{
  pub fn scalar_mul(&self, val: &@TypedExpr<N>) -> @TypedExpr<V>
  { @RValue(ParenthesedOp(@BinaryOperation::new::<V, N, V>(*self, *val, Multiply) as @Expr)) }

  pub fn scalar_mul_inplace(&mut self, _: &@TypedExpr<N>)
  {
    fail!("Not yet implemented.");
    // @RValue(ParenthesedOp(@BinaryOperation::new::<V, N, V>(*self, *val, Multiply) as @Expr))
  }
}

impl<N: 'static + Eq> ClEq<@TypedExpr<bool>> for @TypedExpr<N>
{
  pub fn cl_eq(&self, other: &@TypedExpr<N>) -> @TypedExpr<bool>
  { @RValue(ParenthesedOp(@BinaryOperation::new::<N, N, bool>(*self, *other, Estrict) as @Expr)) }

  pub fn cl_ne(&self, other: &@TypedExpr<N>) -> @TypedExpr<bool>
  { @RValue(ParenthesedOp(@BinaryOperation::new::<N, N, bool>(*self, *other, NEstrict) as @Expr)) }
}

impl<N: 'static + Ord> ClOrd<@TypedExpr<bool>> for @TypedExpr<N>
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

impl<V: 'static + Dot<N>, N: 'static> Dot<@TypedExpr<N>> for @TypedExpr<V>
{
  fn dot(&self, other: &@TypedExpr<V>) -> @TypedExpr<N>
  { @RValue(ParenthesedOp(@BinaryOperation::new::<V, V, N>(*self, *other, Dot) as @Expr)) }
}

impl<N: 'static + Orderable> Orderable for @TypedExpr<N>
{
  fn min(&self, other: &@TypedExpr<N>) -> @TypedExpr<N>
  { @RValue(ParenthesedOp(@BinaryOperation::new::<N, N, N>(*self, *other, Min) as @Expr)) }

  fn max(&self, other: &@TypedExpr<N>) -> @TypedExpr<N>
  { @RValue(ParenthesedOp(@BinaryOperation::new::<N, N, N>(*self, *other, Max) as @Expr)) }

  fn clamp(&self, mn: &@TypedExpr<N>, mx: &@TypedExpr<N>) -> @TypedExpr<N>
  { @RValue(ParenthesedOp(@TernaryOperation::new::<N, N, N, N>(*self, *mn, *mx, Clamp) as @Expr)) }
}
