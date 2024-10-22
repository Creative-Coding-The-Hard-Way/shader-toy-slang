struct FrameData {
    float2 mouse_pos;
    float2 screen_size;
    float dt;
    float time;
};

[[vk_binding(0, 0)]]
ConstantBuffer<FrameData> frame;

float3 palette(float t)
{
    let a = float3(0.328, 0.408, 0.488);
    let b = float3(0.150, 0.227, 0.227);
    let c = float3(1.330, 1.208, 1.568);
    let d = float3(-0.303, -0.103, 0.197);
    return a + b*cos( 6.283185*(c*t+d) );
}

[shader("fragment")]
float4 main(float2 uv: SmoothOutput) : SV_Target {
    let t = frame.time ;
    var color = float3(1.0, 2.0, 0.8);

    float2 uv0 = uv;

    for (float i = 0; i < 3; i++) {
        uv = fract(uv * 2.0) - 0.5;

        float d = length(uv);
        d = sin(d * 8 + t) / 8;
        d = abs(d);

        color *= (0.05/d) * palette(d*5 + t*0.25);
    }


    return float4(color, 1.0);
}
