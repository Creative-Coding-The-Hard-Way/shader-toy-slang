struct FrameData {
    float2 mouse_pos;
    float2 screen_size;
    float dt;
    float time;
};

[[vk_binding(0, 0)]]
ConstantBuffer<FrameData> frame;

/// Computes a deterministic random number based on the input value.
///
/// Creates weird little circle-in-square patterns when given integer values.
float weird_little_circles_hash(float2 uv) {
    static const float2 DIR = 33.890/sqrt(2);
    let v = 55.9871 * fract(uv * DIR);
    return fract(v.x*v.y);
}

/// Computes a deterministic random number based on the input value.
///
float hash(float2 uv) {
    static const float2 DIR = 33.890/sqrt(2);
    let v = 55.9871 * fract(uv * DIR);
    return fract(v.x*v.y*79.91987);
}

[shader("fragment")]
float4 main(
    float2 pos: SmoothOutput_pos,
    float2 uv: SmoothOutput_uv
) : SV_Target {
    pos.x *= frame.screen_size.x / frame.screen_size.y;
    pos = floor(pos*500.0);
    return float4(float3(hash(pos)), 1.0);
}
