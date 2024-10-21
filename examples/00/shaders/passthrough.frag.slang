struct FrameData {
    float2 mouse_pos;
    float2 screen_size;
    float dt;
    float time;
};

[[vk_binding(0, 0)]]
ConstantBuffer<FrameData> frame;

[shader("fragment")]
float4 main(float2 frag_pos: SmoothOutput) : SV_Target {
    let pi = 3.1415;
    let t = frame.time;
    var color = float4(0.0, 0.0, 0.0, 1.0);
    let pos = frag_pos * frame.screen_size.y;

    let h = 0
        + sin(frag_pos.x * pi * 1.0 + t*2.00) * 100
        + sin(frag_pos.x * pi * 0.5 + t*3.00) * 50
        + sin(frag_pos.x * pi * 0.2 + t*0.50) * frame.screen_size.y * 0.75;

    let d = abs(pos.y - h);

    color.xyz += float3(pow(10/d, 1.3), pow(20/d, 2.0), pow(40/d, 1.3));


    return color;
}
