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

[vk_binding(0, 0)] FrameData frame;

[vk_binding(1, 0)]
RWStructuredBuffer<Particle, Std430DataLayout> particles;

[shader("compute")]
[numthreads(32, 1, 1)]
void computeMain(uint3 dispatch_thread_id : SV_DispatchThreadID) {
    let index = dispatch_thread_id.x;
    let world_height = 20.0;
    let world_width = world_height * (frame.screen_size.x / frame.screen_size.y);
    let mp = frame.mouse_pos * float2(world_width/2, world_height/2);

    var particle = particles[index];

    let a = mp-particle.pos;
    let d = length(a);
    let na = a / d;
    particle.vel *= 0.99999;
    particle.vel += na * clamp(50.0/(d*d),0.1, 100.0) * frame.dt;
    particle.pos += frame.dt * particle.vel;

    if (particle.pos.y < -world_height/2) {
        particle.vel.y *= -1;
    }
    if (particle.pos.y > world_height/2) {
        particle.vel.y *= -1;
    }
    if (particle.pos.x < -world_width/2) {
        particle.vel.x *= -1;
    }
    if (particle.pos.x > world_width/2) {
        particle.vel.x *= -1;
    }

    particles[index] = particle;
}

