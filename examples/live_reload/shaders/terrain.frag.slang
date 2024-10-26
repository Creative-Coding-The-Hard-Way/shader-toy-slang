import lib.value_noise;

// ---------------------------------------------------------------------------
// Global per-frame data
// ---------------------------------------------------------------------------
struct FrameData {
    float2 mouse_pos;
    float2 screen_size;
    float dt;
    float time;
};

[[vk_binding(0, 0)]]
ConstantBuffer<FrameData> frame;


// ---------------------------------------------------------------------------
// Configuration
// ---------------------------------------------------------------------------

// Camera constants
static const float FOV = radians(30);
static const float NEAR_CLIP_DISTANCE = 1 / tan(FOV/2);

// Ray behavior constants
static const int MAX_STEPS = 300;
static const float SURFACE_DISTANCE = 0.01;
static const float MAX_DISTANCE = 100.0;


// ---------------------------------------------------------------------------
// Scene Defenition using signed distance functions
// ---------------------------------------------------------------------------

float sphere(float3 point, float radius) {
    return length(point) - radius; // distance to a sphere of radius 1
}

float horizontal_plane(float3 point) {
    return point.y;
}

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

[Differentiable]
float distance_to_scene(float3 point) {
    let scale = clamp((frame.mouse_pos.y + 1.0) * 0.5, 0.1, 1.0) * 5.0;
    let terrain_height = scale*fbm(point.xz / 3.0) - 4.0;
    return (point.y - terrain_height) / scale;
}


// ---------------------------------------------------------------------------
// The Ray Marching and lighting implementation.
// ---------------------------------------------------------------------------

/// Marches a given point along the ray direction and returns the distance
/// travelled.
float ray_march(float3 ro, float3 rd) {
    var total_distance = 0.0;

    var step = 0;
    for (; step < MAX_STEPS; step++) {
        var p = ro + rd * total_distance;

        var d = distance_to_scene(p);
        total_distance += d;

        if (d < SURFACE_DISTANCE || total_distance > MAX_DISTANCE) {
            break;
        }
    }

    /// If we ran out of steps, then simply indicate that the ray missed the
    /// scene.
    if (step == MAX_STEPS) {
        total_distance = MAX_DISTANCE;
    }

    return total_distance;
}

/// Computes the normal vector for the surface at P.
float3 compute_normal(float3 p) {
    float d = distance_to_scene(p);
    let e = float2(0.01, 0.0);

    let n = d - float3(
        distance_to_scene(p - e.xyy),
        distance_to_scene(p - e.yxy),
        distance_to_scene(p - e.yyx)
    );

    return normalize(n);
}

/// Computes the diffuse lighting
float terrain_diffuse_lighting(float3 p) {
    let light_pos = float3(5.0, 1.0, 10.0);
    let light_dir = normalize(light_pos - p);
    let n = compute_normal(p);

    var diffuse = clamp(dot(n, light_dir), 0.0, 1.0);

    float d = ray_march(p+(n*SURFACE_DISTANCE*2.0), light_dir);
    let max_d = length(light_pos - p);
    if (d < max_d) {
        diffuse *= (d / max_d);
    }

    return diffuse;
}

/// Computes the ray from the origin through the clipping plane.
/// The ray is based on the fragment position and camera fov.
float3 compute_camera_ray(float2 pos) {
    let aspect = frame.screen_size.x / frame.screen_size.y;
    pos.x *= aspect;

    return normalize(float3(pos, NEAR_CLIP_DISTANCE));
}

[shader("fragment")]
float4 main(
    float2 pos: SmoothOutput_pos,
    float2 uv: SmoothOutput_uv
) : SV_Target {
    var color = float3(0.0);

    let ro = float3(0.0);
    let rd: float3 = compute_camera_ray(pos);
    float d = ray_march(ro, rd);

    if (d < MAX_DISTANCE) {
        let p = ro + rd*d;
        color = float3(103.0/255, 133.0/255, 56.0/255) * terrain_diffuse_lighting(p);
    }

    return float4(color, 1.0);
}
