struct FrameData {
    float2 mouse_pos;
};

[[vk_binding(0, 0)]]
ConstantBuffer<FrameData> frame;

[shader("fragment")]
float4 main() : SV_Target {
    return float4(frame.mouse_pos.x, 0.0, frame.mouse_pos.y, 1.0);
}
