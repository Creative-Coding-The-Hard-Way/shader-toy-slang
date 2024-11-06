struct FrameData {
    float2 mouse_pos;
    float2 screen_size;
    float dt;
    float time;
};

struct Particle {
    float2 pos;
    float2 vel;
    float size;
    float mass;
}

/// Computes a deterministic random number based on the input value.
///
float hash(float u) {
    static const float DIR = 33.890/sqrt(2);
    let v = 55.9871 * fract(u * DIR);
    let r = sin(v*v*79.91987);
    return r;
}

[vk_binding(0, 0)] FrameData frame;

[vk_binding(1, 0)]
RWStructuredBuffer<Particle, Std430DataLayout> particles;

[shader("compute")]
[numthreads(32, 1, 1)]
void computeMain(uint3 dispatch_thread_id : SV_DispatchThreadID) {
    let world_height = 20.0;
    let world_width = world_height * (frame.screen_size.x / frame.screen_size.y);
    let mp = frame.mouse_pos * 0.5 * float2(world_width, world_height);

    let index = dispatch_thread_id.x;

    let hi = index+1;
    var particle = Particle();
    particle.pos = float2(1.0, 1.0) * index * 10.0 / 3_200_000;
    particle.vel = float2(hash(hi), hash(hi*hi));
    particle.size = 0.06;
    particles[index] = particle;
}
