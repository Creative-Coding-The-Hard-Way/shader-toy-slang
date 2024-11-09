static const float2 vertices[] = {
    // first triangle
    float2(-1.0, 1.0),  // bottom left
    float2(-1.0, -1.0), // top left
    float2(1.0, 1.0),   // bottom right

    // second triangle
    float2(-1.0, -1.0), // top left
    float2(1.0, 1.0),   // bottom right
    float2(1.0, -1.0),  // top right
};

static const float2 uvs[] = {
    // first triangle
    float2(0.0, 0.0),  // bottom left
    float2(0.0, 1.0),  // top left
    float2(1.0, 0.0),  // bottom right

    // second triangle
    float2(0.0, 1.0), // top left
    float2(1.0, 0.0), // bottom right
    float2(1.0, 1.0), // top right
};

struct Sprite {
    float2 pos;
    float2 size;
    float2 uv_pos;
    float2 uv_size;
    float4 tint;
    float angle;
    int32_t texture;
    uint32_t sampler;
}

struct Layer {
    float4x4 projection;
}

// set = 0 - texture atlas used in fragment shader

/// binding = 0, set = 1
[vk_binding(0, 1)] ConstantBuffer<Layer> layer;

/// binding = 0, set = 2
[vk_binding(0, 2)] StructuredBuffer<Sprite, Std430DataLayout> sprites;

struct VertexStageOutput {
    float4 sv_position : SV_Position;
    float4 tint: Sprite_Tint;
    float2 uv: Sprite_UV;
    int32_t texture: Sprite_Texture;
    int32_t sampler_index: Sprite_Sampler;
};

[shader("vertex")]
VertexStageOutput main(int vertex_id: SV_VertexID, int sprite_id: SV_InstanceID) {
    // TODO: rotation
    let sprite = sprites[sprite_id];
    let pos = sprite.pos + sprite.size * vertices[vertex_id];

    VertexStageOutput output;
    output.sv_position = mul(layer.projection, float4(pos, 0.0, 1.0));
    output.tint = sprite.tint;
    output.uv = sprite.uv_pos + sprite.uv_size * uvs[vertex_id];
    output.texture = sprite.texture;
    output.sampler_index = sprite.sampler;
    return output;
}

