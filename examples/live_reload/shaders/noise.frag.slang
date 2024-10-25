import lib.noise;

struct FrameData {
    float2 mouse_pos;
    float2 screen_size;
    float dt;
    float time;
};

[[vk_binding(0, 0)]]
ConstantBuffer<FrameData> frame;

[shader("fragment")]
float4 main(
    float2 pos: SmoothOutput_pos,
    float2 uv: SmoothOutput_uv
) : SV_Target {
    pos.x *= (frame.screen_size.x / frame.screen_size.y);
    return float4(float3(fbm(pos * 20.0 - frame.time)), 1.0);
}
