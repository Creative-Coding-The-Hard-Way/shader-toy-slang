struct FrameData {
    float2 mouse_pos;
    float2 screen_size;
};

static const float2 vertices[] = {
    // first triangle
    float2(-1.0, 1.0), // bottom left
    float2(-1.0, -1.0),  // top left
    float2(1.0, 1.0),  // bottom right

    // second triangle
    float2(-1.0, -1.0), // top left
    float2(1.0, 1.0), // bottom right
    float2(1.0, -1.0), // top right
};


[[vk_binding(0, 0)]]
ConstantBuffer<FrameData> frame;

struct VertexStageOutput {
    float4 sv_position : SV_Position;
    float2 frag_pos : SmoothOutput;
};

[shader("vertex")]
VertexStageOutput main(int index: SV_VertexID) {
    VertexStageOutput output;

    let aspect = frame.screen_size.x / frame.screen_size.y;
    let pos = vertices[index];
    output.sv_position = float4(pos.x, pos.y, 0.0, 1.0);
    output.frag_pos = float2(pos.x * aspect, -pos.y);

    return output;
}

