use expr::{Location, Expr, TypedExpr};
use expr;

pub struct Kernel
{
  priv last_param_id: uint,
  priv last_var_id:   uint,
  priv params:        ~[@Expr],
  priv exprs:         ~[@Expr]
}

impl Kernel
{
  pub fn new() -> Kernel
  {
    Kernel {
      last_param_id: 0,
      last_var_id:   0,
      params:        ~[],
      exprs:         ~[]

    }
  }

  pub fn push_expr<T: 'static + Expr>(@mut self, expr: @T)
  {
    self.exprs.push(expr as @Expr);
  }

  pub fn param<T: 'static>(@mut self, location: Location) -> @TypedExpr<T>
  {
    let res = self.named_param(~"rs2cl_p" + self.last_param_id.to_str(), location);

    self.last_param_id = self.last_param_id + 1;

    res
  }

  pub fn var<T: 'static>(@mut self) -> @TypedExpr<T>
  {
    let res = self.named_var(~"rs2cl_v" + self.last_var_id.to_str());

    self.last_var_id = self.last_var_id + 1;

    res
  }

  pub fn named_param<T: 'static>(@mut self, name: ~str, location: Location) -> @TypedExpr<T>
  {
    self.params.push(@expr::Declaration::<T>(name.clone(), location) as @Expr);

    // FIXME: return an rvalue if the location is const?
    @expr::LValue(expr::LVariable(name, location, self))
  }

  pub fn named_var<T: 'static>(@mut self, name: ~str) -> @TypedExpr<T>
  {
    self.exprs.push(@expr::Declaration::<T>(name.clone(), expr::Nowhere) as @Expr);

    @expr::LValue(expr::LVariable(name, expr::Nowhere, self))
  }

  // FIXME: implement else_ and elif_
  pub fn if_(@mut self, _: @TypedExpr<bool>, f: &fn())
  {
    // XXX: push constructs to start an if
    f();
    // XXX: push constructs to end an if
    fail!("Not yet implemented.")
  }
}

impl ToStr for Kernel
{
  fn to_str(&self) -> ~str
  {
    // XXX: transform this to a valid OpenCL Kernel!
    fail!("Not yet implemented.");
  }
}
