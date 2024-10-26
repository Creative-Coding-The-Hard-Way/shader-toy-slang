module interpolation;

/// Computes bilinear interpolation at point p.
///
/// The domain of p is [(0, 0), (1, 1)].
///
/// The values of U control the interpolation such that:
///   - bilinear(float2(0,0)) == u.x
///   - bilinear(float2(0,1)) == u.y
///   - bilinear(float2(1,0)) == u.z
///   - bilinear(float2(1,1)) == u.w
///
[Differentiable]
public float interpolate_bilinear(float2 p, no_diff float4 u) {
    let u0 = u.x;
    let u1 = u.y;
    let u2 = u.z;
    let u3 = u.w;
    let x = p.x;
    let y = p.y;
    return (u3+u0-u1-u2)*x*y + (u1-u0)*y + (u2-u0)*x + u0;
}

/// Computes bicubic interpolation at point p.
///
/// The vector u describes the values of the function at each of the
/// corners of the unit square such that:
///   - bicubic(float2(0,0)) == u.x
///   - bicubic(float2(0,1)) == u.y
///   - bicubic(float2(1,0)) == u.z
///   - bicubic(float2(1,1)) == u.w
///
[Differentiable]
public float interpolate_bicubic(float2 p, no_diff float4 u) {
    let u0 = u.x;
    let u1 = u.y;
    let u2 = u.z;
    let u3 = u.w;
    let x = p.x;
    let y = p.y;
    let P = u1 - u0;
    let Q = u3 - u1 - u2 + u0;
    let R = u2 - u0;
    let x2 = x*x;
    let x3 = x*x*x;
    let A = P + 3*Q*x2 - 2*Q*x3;
    return u0 + 3*R*x2 - 2*R*x3 + 3*y*y*A - 2*y*y*y*A;
}
