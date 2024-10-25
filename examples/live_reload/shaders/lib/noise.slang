module noise;

/// Computes a consistent 'hash' of the provided numbers.
public float hash(float2 ij) {
    static const float pi = radians(180);
    let x = 51397.7915953 * sin(dot(ij, float2(12.95137, 134.15997)));
    return fract(x);
}

/// Computes bilinear interpolation at point p.
float bilinear(float2 p, float4 u) {
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
float bicubic(float2 p, float4 u) {
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

public float noise(float2 p) {
    int2 ij = floor(p);
    int2 off = int2(0, 1);
    return bicubic(
        fract(p),
        float4(
            hash(ij + off.xx),
            hash(ij + off.xy),
            hash(ij + off.yx),
            hash(ij + off.yy)
        )
    );
}

public float fbm(float2 p) {
    static const float2x2 ROTATE = float2x2(
        3.0/5.0, -4.0/5.0,
        4.0/5.0, 3.0/5.0
    );
    static const float SCALE = sqrt(2);

    var total = 0.0;
    var amplitude = 0.5;

    [unroll]
    for (int i = 0; i < 4; i++) {
        total += amplitude*noise(p);

        p = SCALE * mul(p, ROTATE);
        amplitude *= 0.5;
    }

    return total;
}
