# rs2cl

**rs2cl** is a DSL to write OpenCL kernels in Rust: you write some rust-like code, and it is then
converted (at runtime) to a string containing valid OpenCL code. Of course, you kernel written in
rust is type checked at compile time.

Note that **rs2cl** is pure Rust code. OpenCL code generation is done with the `to_str` method.
You do not need any kind preprocessor.

If you happen to make it generate invalid OpenCL code, this is a bug.

## Features
**rs2cl** is covers a very small subset of OpenCL functionalities at the moment.
Here is what the addition of two vectors look like:

```rust
extern mod rs2cl;

use rs2cl::pragma;
use rs2cl::expr;
use rs2cl::kernel::Kernel;

fn addition_kernel()
{
    let k = @mut Kernel::new(~"add");

    k.enable_extension(pragma::cl_khr_fp64);

    let in_a = k.param::<~[f64]>(expr::Global);
    let in_b = k.param::<~[f64]>(expr::Global);
    let out  = k.param::<~[f64]>(expr::Global);

    let id = k.var();

    id.assign(k.get_global_id(0));

    out[id].assign(in_a[id] + in_b[id]);

    println(k.to_str());
}
```

Note that this is much more verbose than the equivalent OpenCL code. **rs2cl** becomes more
handy for bigger kernels. See the **roft** project for examples of more advanced kernels for
soft-body physics simulation.

We intend to be able to generate every examples from the OpenCL Green book on the future.


## rs2cl in use
  * https://github.com/natal/roft (see the file src/kernels.rs)
