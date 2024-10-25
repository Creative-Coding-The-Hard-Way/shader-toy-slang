module value_noise;

import interpolation;

/// Computes a consistent hash of the provided numbers.
///
/// The main idea with this 'hash' is to abuse the apparent randomness observed
/// by large-valued floating point numbers when multiplied by a reasonably
/// varied output of the sin function.
///
public float value_noise_hash(float2 ij) {
    static const float pi = radians(180);

    // these constants are arbitrary. Tune until the results look good to you.
    let x = 51397.7915953 * sin(dot(ij, float2(12.95137, 134.15997)));
    return fract(x);
}

/// Computes Value Noise at the point P.
///
/// Value noise is composed by generating a lattice of random numbers at each
/// integer-valued grid point of the input domain. So each input coordinate,
/// (0, 0), (1, 0), (2, 0), (3, 5), etc... is assigned a deterministic value
/// between 0.0 and 1.0. The values are interpolated with bicubic interpolation
/// so there are no sharp transitions between neighboring values.
///
[Differentiable]
public float noise(float2 p) {
    int2 ij = no_diff floor(p);
    int2 off = int2(0, 1);
    return interpolate_bicubic(
        no_diff fract(p),
        float4(
            no_diff value_noise_hash(ij + off.xx),
            no_diff value_noise_hash(ij + off.xy),
            no_diff value_noise_hash(ij + off.yx),
            no_diff value_noise_hash(ij + off.yy)
        )
    );
}
