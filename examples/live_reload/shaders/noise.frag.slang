
struct FrameData {
    float2 mouse_pos;
    float2 screen_size;
    float dt;
    float time;
};

[[vk_binding(0, 0)]]
ConstantBuffer<FrameData> frame;

/// Computes the bilinear interpolation at point p.
///
/// The vector u describes the values of the function at each of the
/// corners of the unit square such that:
///   - bilerp(float2(0,0)) == u.x
///   - bilerp(float2(0,1)) == u.y
///   - bilerp(float2(1,0)) == u.z
///   - bilerp(float2(1,1)) == u.w
///
float bilerp(float2 p, float4 u) {
    let u0 = u.x;
    let u1 = u.y;
    let u2 = u.z;
    let u3 = u.w;
    let x = p.x;
    let y = p.y;
    return (
        (u3 + u0 - u1 - u2)*x*y +
        (u1 - u0) * y +
        (u2 - u0) * x +
        u0
    );
}

[shader("fragment")]
float4 main(
    float2 pos: SmoothOutput_pos,
    float2 uv: SmoothOutput_uv
) : SV_Target {
    return float4(float3(bilerp(uv, float4(0.0, 1.0, 1.0, -1.0))), 1.0);
}
