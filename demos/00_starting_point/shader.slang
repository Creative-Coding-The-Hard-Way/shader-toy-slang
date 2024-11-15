struct FrameData {
    float2 mouse_pos;
    float2 screen_size;
    float dt;
    float time;
};

[vk_binding(0, 0)] SamplerState samplers[3];
[vk_binding(1, 0)] texture2D textures[];

[vk_binding(0, 1)] ConstantBuffer<FrameData> frame;

[shader("fragment")]
float4 main(
    float2 pos: SmoothOutput_pos,
    float2 uv: SmoothOutput_uv
) : SV_Target {
    return float4(0.1, 0.1, 0.5, 1.0);
}
