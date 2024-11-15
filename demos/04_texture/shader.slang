struct FrameData {
    float2 mouse_pos;
    float2 screen_size;
    float dt;
    float time;
};

[vk_binding(0, 0)] SamplerState samplers[3];
[vk_binding(1, 0)] texture2D textures[];

[vk_binding(0, 1)] ConstantBuffer<FrameData> frame;

// background is 0 because it is listed first in the cli args
const static int background_index = 0;

// brush is 1 because it is listed second in the cli args
const static int brush_index = 1;

[shader("fragment")]
float4 main(
    float2 pos: SmoothOutput_pos,
    float2 uv: SmoothOutput_uv
) : SV_Target {
    let d = length(pos - frame.mouse_pos);

    uv = float2(uv.x, 1.0 - uv.y);

    let b = textures[background_index].Sample(samplers[0], uv);
    let c = textures[brush_index].Sample(samplers[0], uv);
    let c2 = textures[brush_index].Sample(samplers[0], (uv * 0.5));

    let c3 = c2 * c / d;

    return b * c3;
}
