struct Particle {
    float2 pos;
    float2 vel;
    float size;
    float mass;
}

[vk_binding(0, 0)]
RWStructuredBuffer<Particle, Std430DataLayout> particles;

[shader("compute")]
void computeMain(int index : SV_GroupIndex) {
    var particle = particles[index];
    particle.pos += 0.001 * particle.vel;
    particles[index] = particle;
}
