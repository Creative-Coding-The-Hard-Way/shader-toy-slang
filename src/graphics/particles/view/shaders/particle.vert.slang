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

struct VertexStageOutput {
    float4 sv_position : SV_Position;
    float2 uv: SmoothOutput_uv;
};

struct Particle {
    float2 pos;
    float2 vel;
    float size;
    float mass;
}

/// The projection matrix for the scene.
[vk_binding(0, 0)]
float4x4 projection;

[vk_binding(1, 0)]
StructuredBuffer<Particle, Std430DataLayout> particles;

[shader("vertex")]
VertexStageOutput main(int index: SV_VertexID, int instance: SV_InstanceID) {
    VertexStageOutput output;

    let particle = particles[instance];
    let pos = particle.size * vertices[index] + particle.pos;
    output.sv_position = mul(projection, float4(pos, 0.0, 1.0));
    output.uv = uvs[index];

    return output;
}

