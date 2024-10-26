import "../include/value_noise";

struct FrameData {
    float2 mouse_pos;
    float2 screen_size;
    float dt;
    float time;
};

[[vk_binding(0, 0)]]
ConstantBuffer<FrameData> frame;

/// Computes "Fractal Brownian Motion" based noise.
///
/// The 'Fractal' part of the noise is accomplished by composing multiple
/// octaves of a more simple noise function.
[Differentiable]
public float fbm(float2 p) {
    static const float2x2 ROTATE = float2x2(
        3.0/5.0, -4.0/5.0,
        4.0/5.0, 3.0/5.0
    );
    static const float SCALE = radians(180) / 2;

    var total = 0.0;
    var amplitude = 0.5;

    [unroll]
    for (int i = 0; i < 8; i++) {
        total += amplitude*vnoise(p);

        p = SCALE * mul(p + float2(5.0, 7.0), ROTATE);
        amplitude *= 0.5;
    }

    return total;
}

[shader("fragment")]
float4 main(
    float2 pos: SmoothOutput_pos,
    float2 uv: SmoothOutput_uv
) : SV_Target {
    // adjust the input position based on the screen's aspect ratio
    let a =(frame.screen_size.x / frame.screen_size.y);
    pos.x *= a;
    let mouse = frame.mouse_pos * float2(a, 1.0);
    let dir = pos - mouse;
    let d = length(dir);
    let sq_d = dot(dir, dir);

    let result = fwd_diff(fbm)(DifferentialPair<float2>(
        float2(pos * 10.0),
        dir / d,
    ));

    let ambient = float3(165.0/255.0, 42.0/255.0, 42.0/255.0) * 0.1 * result.p;

    let diffuse_material = float3(0.7);
    let diffuse = diffuse_material * result.d * 0.2/clamp(sq_d, 1.0, 10000.0);
    let spotlight = float3(1.0, 1.0, 1.1)*(0.025/clamp(d, 0.05, 10000.0));

    let c = ambient + diffuse + spotlight;

    return float4(c, 1.0);
}
