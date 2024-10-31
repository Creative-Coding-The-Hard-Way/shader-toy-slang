struct FrameData {
    float2 mouse_pos;
    float2 screen_size;
    float dt;
    float time;
};

[vk_binding(0, 0)] ConstantBuffer<FrameData> frame;

[vk_binding(1, 0)] SamplerState sampler;
[vk_binding(2, 0)] texture2D texture;


[shader("fragment")]
float4 main(
    float2 pos: SmoothOutput_pos,
    float2 uv: SmoothOutput_uv
) : SV_Target {
    uint w;
    uint h;
    texture.GetDimensions(w, h); // whatt??
    if (frame.time < 0.1) {
        printf("%d, %d", w, h);
    }
    return texture.Sample(sampler, float2(uv.x, 1.0 - uv.y));
}
