struct FrameData {
    float2 mouse_pos;
    float2 screen_size;
    float dt;
    float time;
};

// Per-frame data is provided in descriptor set 0
[vk_binding(0, 0)] ConstantBuffer<FrameData> frame;

// samplers and textures are provided in descriptor set 1
[vk_binding(0, 1)] SamplerState sampler;
[vk_binding(1, 1)] texture2D background;
[vk_binding(2, 1)] texture2D brush;


[shader("fragment")]
float4 main(
    float2 pos: SmoothOutput_pos,
    float2 uv: SmoothOutput_uv
) : SV_Target {
    uv = float2(uv.x, 1.0 - uv.y);

    return background.Sample(sampler, uv) * brush.Sample(sampler, uv);
}
