/// binding = 0, set = 0
[vk_binding(0, 0)] SamplerState samplers[3];

/// binding = 1, set = 0
[vk_binding(1, 0)] Texture2D textures[];

[shader("fragment")]
float4 main(
    float4 tint: Sprite_Tint,
    float2 uv: Sprite_UV,
    int32_t texture: Sprite_Texture,
    uint32_t sampler_index: Sprite_Sampler
) : SV_Target {
    printf("%i\n", texture);
    if (texture < 0) {
        return tint;
    } else {
        let tex = textures[NonUniformResourceIndex(texture)];
        let sampler = samplers[NonUniformResourceIndex(sampler_index)];
        return tex.Sample(sampler, uv) * tint;
    }
}
