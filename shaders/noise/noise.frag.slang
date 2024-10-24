struct FrameData {
    float2 mouse_pos;
    float2 screen_size;
    float dt;
    float time;
};

[[vk_binding(0, 0)]]
ConstantBuffer<FrameData> frame;

/// Computes a consistent 'hash' of the provided numbers.
float hash(float i, float j) {
    let u = 37 * fract(i / sqrt(2));
    let v = 91 * fract(j / sqrt(2));
    return (2.0 * fract(u*v*(u+v))) - 1.0;
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

float noise(float2 p) {
    let i = trunc(p.x);
    let j = trunc(p.y);
    return bicubic(
        fract(p),
        float4(
            hash(i+0,j+0),
            hash(i+0,j+1),
            hash(i+1,j+0),
            hash(i+1,j+1)
        )
    );
}

static const float2x2 R = float2x2(
    3.0/5.0, -4.0/5.0,
    4.0/5.0, 3.0/5.0
);

float multinoise(float2 p) {
    p = abs(p);
    float total = 0.0;
    let base_scale = 3.1415;
    var rotate = float2x2(1.0, 0.0, 0.0, 1.0);
    var scale = 1.0;
    for (int i = 0; i < 5; i++) {
        let at = scale * mul(p, rotate);
        total += (noise(at) / scale);

        rotate *= R;
        scale *= base_scale;
    }
    return total;
}

[shader("fragment")]
float4 main(
    float2 pos: SmoothOutput_pos,
    float2 uv: SmoothOutput_uv
) : SV_Target {
    pos.x *= (frame.screen_size.x / frame.screen_size.y);
    let color = float3(
        (multinoise(pos*5 + frame.time) * 0.5) + 0.5
    );
    return float4(color, 1.0);
}
