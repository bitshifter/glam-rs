#![allow(non_snake_case)]

#[macro_use]
mod support;

use glam::{
    dvec2, dvec3, dvec4, DMat2, DMat3, DMat4, DVec2, DVec3, DVec4, Vec3Swizzles, Vec4Swizzles,
};

fn sorted<const N: usize>(mut x: [f64; N]) -> [f64; N] {
    x.sort_by(|a, b| a.total_cmp(&b));
    x
}

fn complex(x: f64) -> DVec2 {
    dvec2(x, 0.0)
}

fn csorted<const N: usize>(mut x: [DVec2; N]) -> [DVec2; N] {
    x.sort_by(|a, b| a.x.total_cmp(&b.x).then(a.y.total_cmp(&b.y)));
    x
}

fn csqrt(x: f64) -> DVec2 {
    if x.is_sign_positive() {
        dvec2(x.sqrt(), 0.0)
    } else {
        dvec2(0.0, (-x).sqrt())
    }
}

// Direct solving of characteristic polynomial
fn eigvals2_symmetric(m: DMat2) -> [f64; 2] {
    let t = m.trace();
    let d = m.determinant();
    let center = 0.5 * t;
    let gap = (0.25 * t * t - d).sqrt();
    [center - gap, center + gap]
}

fn eigvals2(m: DMat2) -> [DVec2; 2] {
    let t = m.trace();
    let d = m.determinant();
    let center = complex(0.5 * t);
    let gap = csqrt(0.25 * t * t - d);
    [center - gap, center + gap]
}

fn qr3(A: DMat3) -> (DMat3, DMat3) {
    let mut A = A;
    let mut Q = DMat3::IDENTITY;

    let H = make_householder3(A.x_axis);
    Q = Q * H;
    A = H * A;

    let H = make_householder2(A.y_axis.yz());
    let H = DMat3::from_cols_array(&[
        1.0, 0.0, 0.0, 0.0, H.x_axis.x, H.y_axis.x, 0.0, H.x_axis.y, H.y_axis.y,
    ]);
    Q = Q * H;
    A = H * A;

    return (Q, A);
}

// These make_householder functions assume that a.x + a.length() is nonzero
fn make_householder2(a: DVec2) -> DMat2 {
    let d = a.x + a.length().copysign(a.x);
    let mut v = a / d;
    v.x = 1.0;
    let mut H = DMat2::IDENTITY;
    H -= (2.0 / v.dot(v)) * DMat2::from_cols(v * v.x, v * v.y);
    H
}

fn make_householder3(a: DVec3) -> DMat3 {
    let d = a.x + a.length().copysign(a.x);
    let mut v = a / d;
    v.x = 1.0;
    let mut H = DMat3::IDENTITY;
    H -= (2.0 / v.dot(v)) * DMat3::from_cols(v * v.x, v * v.y, v * v.z);
    H
}

fn make_householder4(a: DVec4) -> DMat4 {
    let d = a.x + a.length().copysign(a.x);
    let mut v = a / d;
    v.x = 1.0;
    let mut H = DMat4::IDENTITY;
    H -= (2.0 / v.dot(v)) * DMat4::from_cols(v * v.x, v * v.y, v * v.z, v * v.w);
    H
}

fn qr4(A: DMat4) -> (DMat4, DMat4) {
    let mut A = A;
    let mut Q = DMat4::IDENTITY;

    let H = make_householder4(A.x_axis);
    Q = Q * H;
    A = H * A;

    let H = make_householder3(A.y_axis.yzw());
    let H = DMat4::from_cols_array(&[
        1.0, 0.0, 0.0, 0.0, 0.0, H.x_axis.x, H.y_axis.x, H.z_axis.x, 0.0, H.x_axis.y, H.y_axis.y,
        H.z_axis.y, 0.0, H.x_axis.z, H.y_axis.z, H.z_axis.z,
    ]);
    Q = Q * H;
    A = H * A;

    let H = make_householder2(A.z_axis.zw());
    let H = DMat4::from_cols_array(&[
        1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, H.x_axis.x, H.y_axis.x, 0.0, 0.0,
        H.x_axis.y, H.y_axis.y,
    ]);
    Q = Q * H;
    A = H * A;

    return (Q, A);
}

fn eigvals3_symmetric(A: DMat3) -> [f64; 3] {
    let [a, b, c] = eigvals3(A);
    [a.x, b.x, c.x]

    // TODO:
    // let p1 = A.y_axis.x.powi(2) + A.z_axis.x.powi(2) + A.z_axis.y.powi(2);

    // if p1 == 0.0 {
    //     // A is diagonal
    //     [A.x_axis.x, A.y_axis.y, A.z_axis.z]
    // } else {
    //     let tr = A.trace();
    //     let q = tr / 3.0;
    //     let p2 = (A.x_axis.x - q).powi(2)
    //         + (A.y_axis.y - q).powi(2)
    //         + (A.z_axis.z - q).powi(2)
    //         + 2.0 * p1;
    //     let p = (p2 / 6.0).sqrt();
    //     let B = (1.0 / p) * (A - q * DMat3::IDENTITY);
    //     let r = B.determinant() / 2.0;
    //     // let r = -1.0_f64;

    //     // In exact arithmetic for a symmetric matrix  -1 <= r <= 1
    //     // but computation error can leave it slightly outside this range.
    //     let phi = r.clamp(-1.0, 1.0).acos();

    //     // the eigenvalues satisfy c <= b <= a
    //     let a = q + 2.0 * p * phi.cos();
    //     let c = q + 2.0 * p * (phi + (2.0 * std::f64::consts::PI / 3.0)).cos();
    //     let b = tr - a - c;

    //     dbg!(a, b, c, phi, r);

    //     [a, b, c]
    // }
}

#[inline]
fn lmul2(v: DVec2, M: DMat2) -> DVec2 {
    dvec2(v.dot(M.x_axis), v.dot(M.y_axis))
}

#[inline]
fn outer_product2(a: DVec2, b: DVec2) -> DMat2 {
    DMat2::from_cols(a * b.x, a * b.y)
}

#[inline]
fn lmul3(v: DVec3, M: DMat3) -> DVec3 {
    dvec3(v.dot(M.x_axis), v.dot(M.y_axis), v.dot(M.z_axis))
}

#[inline]
fn outer_product3(a: DVec3, b: DVec3) -> DMat3 {
    DMat3::from_cols(a * b.x, a * b.y, a * b.z)
}

fn hessenberg3(A: DMat3) -> DMat3 {
    let mut v = A.x_axis.yz();
    let alpha = -v.length();
    let alpha = if v.x < 0.0 { -alpha } else { alpha };
    v.x -= alpha;
    v = v.normalize_or_zero();

    let Q = DMat2::from_mat3_minor(A, 0, 0);
    let Q = Q - 2.0 * outer_product2(v, lmul2(v, Q));

    let mut A = DMat3::from_cols_array(&[
        A.x_axis.x, alpha, 0.0, A.y_axis.x, Q.x_axis.x, Q.x_axis.y, A.z_axis.x, Q.y_axis.x,
        Q.y_axis.y,
    ]);

    let V = dvec3(
        dvec2(A.y_axis.x, A.z_axis.x).dot(v),
        dvec2(A.y_axis.y, A.z_axis.y).dot(v),
        dvec2(A.y_axis.z, A.z_axis.z).dot(v),
    );
    A.y_axis = A.y_axis - 2.0 * V * v.x;
    A.z_axis = A.z_axis - 2.0 * V * v.y;

    // Q2D(1:n,k+1:n) = Q2D(1:n,k+1:n) - 2 * (Q2D(1:n,k+1:n) * v) * v.';

    A
}

fn hessenberg4(A: DMat4) -> DMat4 {
    // dbg!(A);

    let mut v = A.x_axis.yzw();
    let alpha = -v.length();
    let alpha = if v.x < 0.0 { -alpha } else { alpha };
    v.x -= alpha;
    v = v.normalize_or_zero();

    let Q = DMat3::from_mat4_minor(A, 0, 0);
    let Q = Q - 2.0 * outer_product3(v, lmul3(v, Q));

    let mut A = DMat4::from_cols_array(&[
        A.x_axis.x, alpha, 0.0, 0.0, A.y_axis.x, Q.x_axis.x, Q.x_axis.y, Q.x_axis.z, A.z_axis.x,
        Q.y_axis.x, Q.y_axis.y, Q.y_axis.z, A.w_axis.x, Q.z_axis.x, Q.z_axis.y, Q.z_axis.z,
    ]);

    let V = dvec4(
        dvec3(A.y_axis.x, A.z_axis.x, A.w_axis.x).dot(v),
        dvec3(A.y_axis.y, A.z_axis.y, A.w_axis.y).dot(v),
        dvec3(A.y_axis.z, A.z_axis.z, A.w_axis.z).dot(v),
        dvec3(A.y_axis.w, A.z_axis.w, A.w_axis.w).dot(v),
    );
    A.y_axis = A.y_axis - 2.0 * V * v.x;
    A.z_axis = A.z_axis - 2.0 * V * v.y;
    A.w_axis = A.w_axis - 2.0 * V * v.z;

    // let Z = hessenberg3(DMat3::from_mat4_minor(A, 0, 0));
    // let A = DMat4::from_cols_array(&[
    //     A.x_axis.x, A.x_axis.y, A.x_axis.z, A.x_axis.w, A.y_axis.x, Z.x_axis.x, Z.x_axis.y,
    //     Z.x_axis.z, A.z_axis.x, Z.y_axis.x, Z.y_axis.y, Z.y_axis.z, A.w_axis.x, Z.z_axis.x,
    //     Z.z_axis.y, Z.z_axis.z,
    // ]);

    let mut v = A.y_axis.zw();
    let alpha = -v.length();
    let alpha = if v.x < 0.0 { -alpha } else { alpha };
    v.x -= alpha;
    v = v.normalize_or_zero();

    let Q = DMat2::from_cols(A.z_axis.zw(), A.w_axis.zw());
    let Q = Q - 2.0 * outer_product2(v, lmul2(v, Q));

    // let mut AA = DMat3::from_cols_array(&[
    //     A.y_axis.y, alpha, 0.0, A.z_axis.y, Q.x_axis.x, Q.x_axis.y, A.w_axis.y, Q.y_axis.x,
    //     Q.y_axis.y,
    // ]);

    let mut A = DMat4::from_cols_array(&[
        A.x_axis.x, A.x_axis.y, A.x_axis.z, A.x_axis.w, A.y_axis.x, A.y_axis.y, alpha, 0.0,
        A.z_axis.x, A.z_axis.y, Q.x_axis.x, Q.x_axis.y, A.w_axis.x, A.w_axis.y, Q.y_axis.x,
        Q.y_axis.y,
    ]);

    // let V = dvec3(
    //     dvec2(A.y_axis.x, A.z_axis.x).dot(v),
    //     dvec2(A.y_axis.y, A.z_axis.y).dot(v),
    //     dvec2(A.y_axis.z, A.z_axis.z).dot(v),
    // );
    // A.y_axis = A.y_axis - 2.0 * V * v.x;
    // A.z_axis = A.z_axis - 2.0 * V * v.y;

    let V = dvec4(
        dvec2(A.z_axis.x, A.w_axis.x).dot(v),
        dvec2(A.z_axis.y, A.w_axis.y).dot(v),
        dvec2(A.z_axis.z, A.w_axis.z).dot(v),
        dvec2(A.z_axis.w, A.w_axis.w).dot(v),
    );
    A.z_axis = A.z_axis - 2.0 * V * v.x;
    A.w_axis = A.w_axis - 2.0 * V * v.y;

    // Q2D(1:n,k+1:n) = Q2D(1:n,k+1:n) - 2 * (Q2D(1:n,k+1:n) * v) * v.';

    // dbg!(A);
    // todo!();
    A
}

fn eigvals3(A: DMat3) -> [DVec2; 3] {
    const CUTOFF: f64 = 1e-14;
    dbg!(A);

    let mut A = hessenberg3(A);
    // dbg!(A);
    // panic!();

    let mut k = 0;
    loop {
        dbg!(A);

        if A.x_axis.y.abs() <= CUTOFF * (A.x_axis.x.abs() + A.y_axis.y.abs()) {
            let [a, b] = eigvals2(DMat2::from_mat3_minor(A, 0, 0));
            return [a, b, complex(A.x_axis.x)];
        }
        if A.y_axis.z.abs() <= CUTOFF * (A.y_axis.y.abs() + A.z_axis.z.abs()) {
            let [a, b] = eigvals2(DMat2::from_mat3(A));
            return [a, b, complex(A.z_axis.z)];
        }

        // let sigma = A.z_axis.z;

        // let d = 0.5 * (A.y_axis.y - A.z_axis.z);
        // let sigma = A.z_axis.z + d - d.signum() * (d.powi(2) + A.y_axis.z.powi(2)).sqrt();

        // let (Q, R) = qr(A - sigma * DMat3::IDENTITY);
        // A = R * Q + sigma * DMat3::IDENTITY;

        let s = A.y_axis.y + A.z_axis.z;
        let t = A.y_axis.y * A.z_axis.z - A.y_axis.z * A.z_axis.y;

        let M = A * A - s * A + t * DMat3::IDENTITY;
        let (Z, _) = qr3(M);

        A = Z.transpose() * A * Z;

        k += 1;

        // We shouldn't have more than a couple dozen iterations
        if k > 1000 {
            unreachable!();
        }
    }
}

fn eigvals4(A: DMat4) -> [DVec2; 4] {
    const CUTOFF: f64 = 1e-14;

    let mut A = hessenberg4(A);
    // let mut A = A;
    dbg!(A);

    let mut k = 0;
    loop {
        // if k > 100 {
        //     return [
        //         complex(A.x_axis.x),
        //         complex(A.y_axis.y),
        //         complex(A.z_axis.z),
        //         complex(A.w_axis.w),
        //     ];
        // }

        // TODO: Don't recompute tridiagonizalation for smaller problems
        if A.x_axis.y.abs() <= CUTOFF * (A.x_axis.x.abs() + A.y_axis.y.abs()) {
            let [a, b, c] = eigvals3(DMat3::from_mat4_minor(A, 0, 0));
            return [complex(A.x_axis.x), a, b, c];
        }
        if A.y_axis.z.abs() <= CUTOFF * (A.y_axis.y.abs() + A.z_axis.z.abs()) {
            let [a, b] = eigvals2(DMat2::from_cols(A.x_axis.xy(), A.y_axis.xy()));
            let [c, d] = eigvals2(DMat2::from_cols(A.z_axis.zw(), A.w_axis.zw()));
            return [a, b, c, d];
        }
        if A.z_axis.w.abs() <= CUTOFF * (A.z_axis.z.abs() + A.w_axis.w.abs()) {
            let [a, b, c] = eigvals3(DMat3::from_mat4(A));
            return [a, b, c, complex(A.w_axis.w)];
        }

        // let sigma = A.w_axis.w;
        // // let d = 0.5 * (A.y_axis.y - A.z_axis.z);
        // // let sigma = A.z_axis.z + d - d.signum() * (d.powi(2) + A.y_axis.z.powi(2)).sqrt();

        // let (Q, R) = qr4(A - sigma * DMat4::IDENTITY);
        // A = R * Q + sigma * DMat4::IDENTITY;

        let s = A.y_axis.z + A.z_axis.w;
        let t = A.y_axis.z * A.z_axis.w - A.y_axis.w * A.z_axis.z;

        let M = A * A - s * A + t * DMat4::IDENTITY;
        let (Z, _) = qr4(M);

        A = Z.transpose() * A * Z;

        k += 1;

        // We shouldn't have more than a couple dozen iterations
        if k > 1000 {
            unreachable!();
        }
    }
}

const EPS: f32 = 1e-12;
const SYMMETRIC_EPS: f32 = 1e-7;

#[cfg(test)]
/// The tests here are from https://math.stackexchange.com/a/894641
fn assert_valid(eigvals: &[DVec2], trace: f64, trace_sq: f64, det: f64, eps: f32) {
    // The sum of the eigenvalues should be equal to the trace
    let sum: DVec2 = eigvals.iter().copied().sum();
    assert_approx_eq!(sum, complex(trace), eps);

    // The sum of the eigenvalues squared should be equal to the trace of the matrix times itself
    let sum: f64 = eigvals
        .iter()
        .map(|&lambda| lambda.x.powi(2) - lambda.y.powi(2))
        .sum();
    assert_approx_eq!(sum, trace_sq, eps);

    // The product of the eigenvalues should be equal to the determinant
    let sign: f64 = eigvals.iter().map(|&lambda| lambda.x.signum()).product();
    let prod: f64 = eigvals.iter().map(|&lambda| lambda.length()).product();
    assert_approx_eq!(sign * prod, det, eps);
}

glam_test!(test_eigvals2, {
    let [a, b] = csorted(eigvals2(DMat2::from_cols_array(&[1.0, -1.0, -1.0, 1.0])));
    assert_approx_eq!(a, dvec2(0.0, 0.0), EPS);
    assert_approx_eq!(b, dvec2(2.0, 0.0), EPS);

    let [a, b] = csorted(eigvals2(DMat2::from_cols_array(&[1.0, 2.0, 3.0, 4.0])));
    assert_approx_eq!(a, dvec2(-0.3722813232690143, 0.0), EPS);
    assert_approx_eq!(b, dvec2(5.372281323269014, 0.0), EPS);

    let [a, b] = csorted(eigvals2(DMat2::from_cols_array(&[1.0, -1.0, 1.0, 1.0])));
    assert_approx_eq!(a, dvec2(1.0, -1.0), EPS);
    assert_approx_eq!(b, dvec2(1.0, 1.0), EPS);

    #[cfg(feature = "rand")]
    {
        use rand::{Rng, SeedableRng};
        use rand_xoshiro::Xoshiro256Plus;
        let mut rng = Xoshiro256Plus::seed_from_u64(0);

        for _ in 0..1000 {
            let A = DMat2::from_cols_array(&rng.gen::<[f64; 4]>());
            let eigvals = eigvals2(A);
            assert_valid(&eigvals, A.trace(), (A * A).trace(), A.determinant(), EPS);
        }
    }

    // These two examples are the symmetric test cases from above
    let [a, b] = sorted(eigvals2_symmetric(DMat2::from_cols_array(&[
        1.0, -1.0, -1.0, 1.0,
    ])));
    assert_approx_eq!(a, 0.0, EPS);
    assert_approx_eq!(b, 2.0, EPS);

    let [a, b] = sorted(eigvals2_symmetric(DMat2::from_cols_array(&[
        1.0, 2.0, 3.0, 4.0,
    ])));
    assert_approx_eq!(a, -0.3722813232690143, EPS);
    assert_approx_eq!(b, 5.372281323269014, EPS);
});

glam_test!(test_eigvals3, {
    let [a, b, c] = csorted(eigvals3(DMat3::from_cols_array(&[
        1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0,
    ])));
    assert_approx_eq!(a, dvec2(-1.1168439698070436, 0.0), EPS);
    assert_approx_eq!(b, dvec2(0.0, 0.0), EPS);
    assert_approx_eq!(c, dvec2(16.116843969807064, 0.0), EPS);

    let [a, b, c] = csorted(eigvals3(DMat3::from_cols_array(&[
        1.0, 0.0, 1.0, 1.0, 1.0, 0.0, 0.0, 1.0, 1.0,
    ])));
    assert_approx_eq!(a, dvec2(0.5, -0.8660254037844392), EPS);
    assert_approx_eq!(b, dvec2(0.5, 0.8660254037844392), EPS);
    assert_approx_eq!(c, dvec2(2.0, 0.0), EPS);

    // A matrix that the base QR algorithm cannot solve
    let [a, b, c] = csorted(eigvals3(DMat3::from_cols_array(&[
        0.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 0.0,
    ])));
    assert_approx_eq!(a, dvec2(-1.0, 0.0), EPS);
    assert_approx_eq!(b, dvec2(1.0, 0.0), EPS);
    assert_approx_eq!(c, dvec2(1.0, 0.0), EPS);

    // A matrix with a low condition number, which is a bad case for the base QR algorithm
    let delta = 1e-5;
    let [a, b, c] = csorted(eigvals3(DMat3::from_cols_array(&[
        1.0, 0.0, delta, 0.0, 1.0, 0.0, delta, 0.0, 1.0,
    ])));
    assert_approx_eq!(a, dvec2(1.0 - delta, 0.0), EPS);
    assert_approx_eq!(b, dvec2(1.0, 0.0), EPS);
    assert_approx_eq!(c, dvec2(1.0 + delta, 0.0), EPS);

    // Another matrix with a low condition number, rearranged
    let [a, b, c] = csorted(eigvals3(DMat3::from_cols_array(&[
        1.0, 0.0, 0.0, 0.0, 1.0, delta, 0.0, delta, 1.0,
    ])));
    assert_approx_eq!(a, dvec2(1.0 - delta, 0.0), EPS);
    assert_approx_eq!(b, dvec2(1.0, 0.0), EPS);
    assert_approx_eq!(c, dvec2(1.0 + delta, 0.0), EPS);

    let [a, b, c] = csorted(eigvals3(DMat3::from_cols_array(&[
        1.0, 0.0, 0.0, 0.0, 1.0, delta, 0.0, -delta, 1.0,
    ])));
    assert_approx_eq!(a, dvec2(1.0, -delta), EPS);
    assert_approx_eq!(b, dvec2(1.0, 0.0), EPS);
    assert_approx_eq!(c, dvec2(1.0, delta), EPS);

    // A bad case for the non-shifted algorithm
    let [a, b, c] = csorted(eigvals3(DMat3::from_cols_array(&[
        0.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 1.0, 0.0,
    ])));
    assert_approx_eq!(a, dvec2(-1.0, 0.0), EPS);
    assert_approx_eq!(b, dvec2(1.0, 0.0), EPS);
    assert_approx_eq!(c, dvec2(1.0, 0.0), EPS);

    // A test case that did non converge with just the Rayleigh Quotient shift
    let [a, b, c] = csorted(eigvals3(DMat3::from_cols_array(&[
        -0.3213354775549597,
        0.02169014204590863,
        0.16693006894182016,
        -0.06496747561198643,
        0.1566324195390305,
        -0.21279295908748386,
        0.0,
        0.5943067054739077,
        -0.08187628642878006,
    ])));
    assert_approx_eq!(a, dvec2(-0.3448497698916063, 0.0), EPS);
    assert_approx_eq!(b, dvec2(0.0491352127234484, -0.3500020316105234), EPS);
    assert_approx_eq!(c, dvec2(0.0491352127234484, 0.3500020316105234), EPS);

    // Another test case that did non converge with just the Rayleigh Quotient shift
    let [a, b, c] = csorted(eigvals3(DMat3::from_cols_array(&[
        0.16321690858303795,
        0.016804839607712952,
        0.0,
        -0.08822904497924268,
        -0.5184780437091311,
        0.46726403838636654,
        -0.33628128518432093,
        -0.4632840573942136,
        -0.6725939268951061,
    ])));
    assert_approx_eq!(a, dvec2(-0.593049577421523, -0.4563618481950429), EPS);
    assert_approx_eq!(b, dvec2(-0.593049577421523, 0.4563618481950429), EPS);
    assert_approx_eq!(c, dvec2(0.15824409282184598, 0.0), EPS);

    let [a, b, c] = csorted(eigvals3(DMat3::from_cols_array(&[
        -0.26228172092024793,
        -0.03608976367373878,
        -2.0905308540472976e-17,
        0.03080946367110743,
        -0.06701789957369739,
        -0.5167701017184002,
        -0.5131608320367018,
        0.13828914509415038,
        -0.05187357048737509,
    ])));

    assert_approx_eq!(a, dvec2(-0.3270582792266884, 0.0), EPS);
    assert_approx_eq!(b, dvec2(-0.027057455877315686, -0.29800047501630117), EPS);
    assert_approx_eq!(c, dvec2(-0.027057455877315686, 0.29800047501630117), EPS);

    #[cfg(feature = "rand")]
    {
        use rand::{Rng, SeedableRng};
        use rand_xoshiro::Xoshiro256Plus;
        let mut rng = Xoshiro256Plus::seed_from_u64(0);

        for _ in 0..1000 {
            let A = DMat3::from_cols_array(&rng.gen::<[f64; 9]>());
            let eigvals = eigvals3(A);
            assert_valid(&eigvals, A.trace(), (A * A).trace(), A.determinant(), EPS);
        }
    }

    // These three examples are the symmetric test cases from above
    let [a, b, c] = sorted(eigvals3_symmetric(DMat3::from_cols_array(&[
        0.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 0.0,
    ])));
    assert_approx_eq!(a, -1.0, SYMMETRIC_EPS);
    assert_approx_eq!(b, 1.0, SYMMETRIC_EPS);
    assert_approx_eq!(c, 1.0, SYMMETRIC_EPS);

    let delta = 1e-5;
    let [a, b, c] = sorted(eigvals3_symmetric(DMat3::from_cols_array(&[
        1.0, 0.0, delta, 0.0, 1.0, 0.0, delta, 0.0, 1.0,
    ])));
    assert_approx_eq!(a, 1.0 - delta, SYMMETRIC_EPS);
    assert_approx_eq!(b, 1.0, SYMMETRIC_EPS);
    assert_approx_eq!(c, 1.0 + delta, SYMMETRIC_EPS);

    let [a, b, c] = sorted(eigvals3_symmetric(DMat3::from_cols_array(&[
        1.0, 0.0, 0.0, 0.0, 1.0, delta, 0.0, delta, 1.0,
    ])));
    assert_approx_eq!(a, 1.0 - delta, SYMMETRIC_EPS);
    assert_approx_eq!(b, 1.0, SYMMETRIC_EPS);
    assert_approx_eq!(c, 1.0 + delta, SYMMETRIC_EPS);

    #[cfg(feature = "rand")]
    {
        use rand::{Rng, SeedableRng};
        use rand_xoshiro::Xoshiro256Plus;
        let mut rng = Xoshiro256Plus::seed_from_u64(0);

        for _ in 0..10_000 {
            let A = DMat3::from_cols_array(&rng.gen::<[f64; 9]>());
            let A = A + A.transpose();
            let eigvals: Vec<DVec2> = eigvals3_symmetric(A).into_iter().map(complex).collect();
            assert_valid(
                &eigvals,
                A.trace(),
                (A * A).trace(),
                A.determinant(),
                SYMMETRIC_EPS,
            );
        }
    }
});

glam_test!(test_eigvals4, {
    let [a, b, c, d] = csorted(eigvals4(DMat4::from_cols_array(&[
        1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0,
    ])));
    assert_approx_eq!(a, dvec2(-2.2093727122985456, 0.0), EPS);
    assert_approx_eq!(b, dvec2(0.0, 0.0), EPS);
    assert_approx_eq!(c, dvec2(0.0, 0.0), EPS);
    assert_approx_eq!(d, dvec2(36.20937271229853, 0.0), EPS);

    let [a, b, c, d] = csorted(eigvals4(DMat4::from_cols_array(&[
        1.0, 0.0, 1.0, 1.0, 1.0, 1.0, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
    ])));
    assert_approx_eq!(a, dvec2(-0.4476229868548985, 0.0), EPS);
    assert_approx_eq!(b, dvec2(0.6976613211204787, -0.4951597570817051), EPS);
    assert_approx_eq!(c, dvec2(0.6976613211204787, 0.4951597570817051), EPS);
    assert_approx_eq!(d, dvec2(3.0523003446139394, 0.0), EPS);

    // Hilbert matrix
    let [a, b, c, d] = csorted(eigvals4(DMat4::from_cols_array(
        &[
            1f64, 2f64, 3f64, 4f64, 2f64, 3f64, 4f64, 5f64, 3f64, 4f64, 5f64, 6f64, 4f64, 5f64,
            6f64, 7f64,
        ]
        .map(|x| x.recip()),
    )));
    assert_approx_eq!(a, dvec2(9.670230402261436e-5, 0.0), EPS);
    assert_approx_eq!(b, dvec2(0.006738273605760762, 0.0), EPS);
    assert_approx_eq!(c, dvec2(0.16914122022145014, 0.0), EPS);
    assert_approx_eq!(d, dvec2(1.5002142800592426, 0.0), EPS);

    // A matrix with a low condition number, which is a bad case for the base QR algorithm
    let delta = 1e-5;
    let [a, b, c, d] = csorted(eigvals4(DMat4::from_cols_array(&[
        1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, -delta, 0.0, 0.0, delta, 1.0,
    ])));
    assert_approx_eq!(a, dvec2(1.0, -delta), EPS);
    assert_approx_eq!(b, dvec2(1.0, 0.0), EPS);
    assert_approx_eq!(c, dvec2(1.0, 0.0), EPS);
    assert_approx_eq!(d, dvec2(1.0, delta), EPS);

    #[cfg(feature = "rand")]
    {
        use rand::{Rng, SeedableRng};
        use rand_xoshiro::Xoshiro256Plus;
        let mut rng = Xoshiro256Plus::seed_from_u64(0);

        for _ in 0..1_000_000 {
            let A = DMat4::from_cols_array(&rng.gen::<[f64; 16]>());
            dbg!(A);
            let eigvals = eigvals4(A);
            assert_valid(&eigvals, A.trace(), (A * A).trace(), A.determinant(), EPS);
        }
    }
});
