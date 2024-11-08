[shader("fragment")]
float4 main(
    float4 tint: Sprite_Tint,
    float2 uv: Sprite_UV,
) : SV_Target {
    //return float4(uv.x, 0.0, uv.y, 1.0) * tint;
    return float4(1.0);
}
