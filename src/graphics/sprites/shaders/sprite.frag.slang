[shader("fragment")]
float4 main(
    float4 tint: Sprite_Tint,
    float2 uv: Sprite_UV,
    int32_t texture: Sprite_Texture
) : SV_Target {
    printf("%i\n", texture);
    return float4(uv.x, 0.0, uv.y, 1.0) * tint;
}
