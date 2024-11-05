
[shader("fragment")]
float4 main(
    float2 uv: SmoothOutput_uv
) : SV_Target {
    return float4(0.0, 0.0, 1.0, 1.0);
}
