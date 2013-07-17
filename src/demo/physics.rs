#[test]
use std::num::Zero;
#[test]
use nalgebra::vec::Vec3;
#[test]
use nalgebra::traits::scalar_op::ScalarMul;
#[test]
use nalgebra::traits::dot::Dot;
#[test]
use kernel::Kernel;
#[test]
use cl_logic::ClOrd;
#[test]
use expr;

// test
#[test]
fn integration_kernel() -> ()
{
  let k          = @mut Kernel::new();

  let velocities = k.param::<~[Vec3<f64>]>(expr::Global);
  let positions  = k.param::<~[Vec3<f64>]>(expr::Global);
  let fext       = k.param::<Vec3<f64>>(expr::Global);
  let dt         = k.param::<f64>(expr::Global);

  // FIXME: let id         = k.global_id();
  let id = expr::literal(0u);

  velocities[id].assign(velocities[id] + fext.scalar_mul(&dt));
  positions[id].assign(positions[id] + velocities[id].scalar_mul(&dt));
}

#[test]
fn lin_pgs_kernel()
{
  let k = @mut Kernel::new();

  // FIXME: let id         = k.global_id();
  let id = expr::literal(0u);

  /*
   * Params
   */
  let id1s       = k.param::<~[uint]>(expr::Global);
  let id2s       = k.param::<~[uint]>(expr::Global);
  let normals    = k.param::<~[Vec3<f64>]>(expr::Global);
  let inv_masses = k.param::<~[f64]>(expr::Global);
  let impulses   = k.param::<~[f64]>(expr::Global);
  let lobounds   = k.param::<~[f64]>(expr::Global);
  let hibounds   = k.param::<~[f64]>(expr::Global);
  let objectives = k.param::<~[f64]>(expr::Global);
  let pmasses    = k.param::<~[f64]>(expr::Global);
  let MJLambdas  = k.param::<~[Vec3<f64>]>(expr::Global);

  /*
   * Locals
   */
  let mut d_lambda_i = k.var::<f64>();
  let     id1        = k.var::<uint>();
  let     id2        = k.var::<uint>();

  id1.assign(id1s[id]);
  id2.assign(id2s[id]);

  /*
   * The solver itself
   */
  d_lambda_i.assign(objectives[id]);

  do k.if_(id1.cl_ge(&Zero::zero()))
  { d_lambda_i.assign(d_lambda_i + normals[id].dot(&MJLambdas[id1])); }

  do k.if_(id2.cl_ge(&Zero::zero()))
  { d_lambda_i.assign(d_lambda_i - normals[id].dot(&MJLambdas[id2])); }

  d_lambda_i.assign(d_lambda_i / pmasses[id]);

  let lambda_i_0 = k.var::<f64>();

  lambda_i_0.assign(impulses[id]);

  impulses[id].assign((lambda_i_0 + d_lambda_i).clamp(&lobounds[id], &hibounds[id]));

  d_lambda_i = impulses[id] - lambda_i_0;

  do k.if_(id1.cl_ge(&Zero::zero()))
  { MJLambdas[id1].assign(MJLambdas[id1] - normals[id].scalar_mul(&(inv_masses[id1] * d_lambda_i))); }

  do k.if_(id2.cl_ge(&Zero::zero()))
  { MJLambdas[id2].assign(MJLambdas[id2] + normals[id].scalar_mul(&(inv_masses[id2] * d_lambda_i))); }
}
