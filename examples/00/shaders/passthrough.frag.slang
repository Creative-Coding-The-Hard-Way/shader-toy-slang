struct FrameData {
    float2 mouse_pos;
};

[[vk_binding(0, 0)]]
ConstantBuffer<FrameData> frame;

[shader("fragment")]
float4 main(float2 frag_pos: SmoothOutput) : SV_Target {
    var color = float4(0.0, 0.0, 0.0, 1.0);

    float d = distance(frag_pos, frame.mouse_pos);
    color.xyz += float3(pow(0.05/d, 10));

    return color;
}
