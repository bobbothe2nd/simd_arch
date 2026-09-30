# `simd_arch`

Simple SIMD types. These types are very low-level, but completely safe.

They are intentionally only exported when the target supports them, but with mostly safe functions where possible.

## Usage

Basic usage is shown in the matrix multiplication shown below:

```rust
use simd_arch::f32x8;

fn simd_matmul<const M: usize, const N: usize, const K: usize>(
    a: &[[f32; K]; M],
    b: &[[f32; N]; K],
) -> [[f32; N]; M] {
    const {
        assert!(N % 8 == 0);
    }

    let mut c = [[0.0f32; N]; M];

    for i in 0..M {
        for j in (0..N).step_by(8) {
            let mut sum = f32x8::splat(0.0);

            for k in 0..K {
                let a_scalar = a[i][k];

                let b_vec = unsafe {
                    f32x8::loadu(b[k].as_ptr().add(j))
                };

                sum = f32x8::splat(a_scalar).mul_add(b_vec, sum);
            }

            unsafe {
                sum.storeu(c[i].as_mut_ptr().add(j));
            }
        }
    }

    c
}
```

But you could take it a step further and optimize for cache locality using a tiled algorithm:

```rust
use simd_arch::f32x8;

fn tiled_matmul<const M: usize, const N: usize, const K: usize>(
    a: &[[f32; K]; M],
    b: &[[f32; N]; K],
) -> [[f32; N]; M] {
    const {
        assert!(M % 4 == 0);
        assert!(N % 32 == 0);
    }

    let mut c = [[0.0f32; N]; M];

    for i in (0..M).step_by(4) {
        for j in (0..N).step_by(32) {
            let mut c00 = f32x8::splat(0.0);
            let mut c01 = f32x8::splat(0.0);
            let mut c02 = f32x8::splat(0.0);
            let mut c03 = f32x8::splat(0.0);

            let mut c10 = f32x8::splat(0.0);
            let mut c11 = f32x8::splat(0.0);
            let mut c12 = f32x8::splat(0.0);
            let mut c13 = f32x8::splat(0.0);

            let mut c20 = f32x8::splat(0.0);
            let mut c21 = f32x8::splat(0.0);
            let mut c22 = f32x8::splat(0.0);
            let mut c23 = f32x8::splat(0.0);

            let mut c30 = f32x8::splat(0.0);
            let mut c31 = f32x8::splat(0.0);
            let mut c32 = f32x8::splat(0.0);
            let mut c33 = f32x8::splat(0.0);

            for k in 0..K {
                let b0 = unsafe {
                    f32x8::loadu(b[k].as_ptr().add(j))
                };

                let b1 = unsafe {
                    f32x8::loadu(b[k].as_ptr().add(j + 8))
                };

                let b2 = unsafe {
                    f32x8::loadu(b[k].as_ptr().add(j + 16))
                };

                let b3 = unsafe {
                    f32x8::loadu(b[k].as_ptr().add(j + 24))
                };

                let a0 = f32x8::splat(a[i][k]);
                let a1 = f32x8::splat(a[i + 1][k]);
                let a2 = f32x8::splat(a[i + 2][k]);
                let a3 = f32x8::splat(a[i + 3][k]);

                c00 = a0.mul_add(b0, c00);
                c01 = a0.mul_add(b1, c01);
                c02 = a0.mul_add(b2, c02);
                c03 = a0.mul_add(b3, c03);

                c10 = a1.mul_add(b0, c10);
                c11 = a1.mul_add(b1, c11);
                c12 = a1.mul_add(b2, c12);
                c13 = a1.mul_add(b3, c13);

                c20 = a2.mul_add(b0, c20);
                c21 = a2.mul_add(b1, c21);
                c22 = a2.mul_add(b2, c22);
                c23 = a2.mul_add(b3, c23);

                c30 = a3.mul_add(b0, c30);
                c31 = a3.mul_add(b1, c31);
                c32 = a3.mul_add(b2, c32);
                c33 = a3.mul_add(b3, c33);
            }

            unsafe {
                c00.storeu(c[i].as_mut_ptr().add(j));
                c01.storeu(c[i].as_mut_ptr().add(j + 8));
                c02.storeu(c[i].as_mut_ptr().add(j + 16));
                c03.storeu(c[i].as_mut_ptr().add(j + 24));

                c10.storeu(c[i + 1].as_mut_ptr().add(j));
                c11.storeu(c[i + 1].as_mut_ptr().add(j + 8));
                c12.storeu(c[i + 1].as_mut_ptr().add(j + 16));
                c13.storeu(c[i + 1].as_mut_ptr().add(j + 24));

                c20.storeu(c[i + 2].as_mut_ptr().add(j));
                c21.storeu(c[i + 2].as_mut_ptr().add(j + 8));
                c22.storeu(c[i + 2].as_mut_ptr().add(j + 16));
                c23.storeu(c[i + 2].as_mut_ptr().add(j + 24));

                c30.storeu(c[i + 3].as_mut_ptr().add(j));
                c31.storeu(c[i + 3].as_mut_ptr().add(j + 8));
                c32.storeu(c[i + 3].as_mut_ptr().add(j + 16));
                c33.storeu(c[i + 3].as_mut_ptr().add(j + 24));
            }
        }
    }

    c
}
```
