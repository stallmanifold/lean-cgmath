extern crate cglinalg;
extern crate criterion;
extern crate rand;
extern crate rand_isaac;


use cglinalg::{
    Matrix2x2,
    Matrix3x3,
    Matrix4x4,
};

use rand::{
    Rng, 
    prelude::Distribution,
    distributions::Standard,
};

use rand_isaac::{
    IsaacRng,
};

use criterion::{
    black_box,
    criterion_group,
    criterion_main,
};

fn gen_scalar<S>() -> S
where
    Standard: Distribution<S>
{
    use rand::SeedableRng;
    let mut rng = IsaacRng::seed_from_u64(0);

    rng.gen()
}

fn gen_matrix2x2<S>() -> Matrix2x2<S> 
where 
    Standard: Distribution<S> 
{
    use rand::SeedableRng;
    let mut rng = IsaacRng::seed_from_u64(0);
    
    Matrix2x2::new(
        rng.gen(), rng.gen(),
        rng.gen(), rng.gen()
    )
}

fn gen_matrix3x3<S>() -> Matrix3x3<S> 
where 
    Standard: Distribution<S> 
{
    use rand::SeedableRng;
    let mut rng = IsaacRng::seed_from_u64(0);
    
    Matrix3x3::new(
        rng.gen(), rng.gen(), rng.gen(),
        rng.gen(), rng.gen(), rng.gen(),
        rng.gen(), rng.gen(), rng.gen()
    )
}

fn gen_matrix4x4<S>() -> Matrix4x4<S> 
where 
    Standard: Distribution<S> 
{
    use rand::SeedableRng;
    let mut rng = IsaacRng::seed_from_u64(0);
    
    Matrix4x4::new(
        rng.gen(), rng.gen(), rng.gen(), rng.gen(),
        rng.gen(), rng.gen(), rng.gen(), rng.gen(),
        rng.gen(), rng.gen(), rng.gen(), rng.gen(),
        rng.gen(), rng.gen(), rng.gen(), rng.gen()
    )
}

macro_rules! bench_binop(
    ($name: ident, $scalar_type:ty, $type1:ty, $type2:ty, $generator_t1:ident, $generator_t2:ident, $binop:ident) => {
        fn $name(bh: &mut criterion::Criterion) {
            let a = $generator_t1::<$scalar_type>();
            let b = $generator_t2::<$scalar_type>();

            bh.bench_function(stringify!($name), move |bh| bh.iter(|| {
                a.$binop(b)
            }));
        }
    }
);

macro_rules! bench_unop(
    ($name:ident, $scalar_type:ty, $ty:ty, $generator:ident, $unop:expr) => {
        fn $name(bh: &mut criterion::Criterion) {
            let v = $generator::<$scalar_type>();

            bh.bench_function(stringify!($name), move |bh| bh.iter(|| {
                $unop(&v)
            }));
        }
    }
);

fn swap_rows2x2(bh: &mut criterion::Criterion) {
    let mut m: Matrix2x2<f32> = gen_matrix2x2();

    bh.bench_function(stringify!("finite2"), move |bh| bh.iter(|| {
        black_box(m.is_finite())
    }));
}

fn swap_rows3x3(bh: &mut criterion::Criterion) {
    let mut m: Matrix3x3<f32> = gen_matrix3x3();

    bh.bench_function(stringify!("finite3"), move |bh| bh.iter(|| {
        black_box(m.is_finite())
    }));
}

fn swap_rows4x4(bh: &mut criterion::Criterion) {
    let mut m: Matrix4x4<f32> = gen_matrix4x4();

    bh.bench_function(stringify!("finite4"), move |bh| bh.iter(|| {
        black_box(m.is_finite())
    }));
}

// bench_binop!(matrix2x2_add_matrix2x2_f32, f32, Matrix2x2<f32>, Matrix2x2<f32>, gen_matrix2x2, gen_matrix2x2, add);

// bench_unop!(matrix2x2_cast_f32_to_f64, f32, Matrix2x2<f32>, gen_matrix2x2, Matrix2x2::swap_columns::<f64>);
// bench_unop!(matrix3x3_cast_f32_to_f64, f32, Matrix3x3<f32>, gen_matrix3x3, Matrix3x3::swap::<f64>);
// bench_unop!(matrix4x4_cast_f32_to_f64, f32, Matrix4x4<f32>, gen_matrix4x4, Matrix4x4::cast::<f64>);


criterion_group!(
    matrix_swap_rows_benchmarks,
    swap_rows2x2,
    swap_rows3x3,
    swap_rows4x4
);
criterion_main!(matrix_swap_rows_benchmarks);

