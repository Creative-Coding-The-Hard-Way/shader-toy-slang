static const float2 vertices[] = {
    float2(-0.5, -0.5),
    float2(0.0, 0.5),
    float2(0.5, -0.5),
};

struct VertexStageOutput {
    float4 sv_position : SV_Position;
};

[shader("vertex")]
VertexStageOutput main(int index: SV_VertexID) {
    VertexStageOutput output;

    let pos = vertices[index];
    output.sv_position = float4(pos.x, pos.y, 0.0, 1.0);

    return output;
}

