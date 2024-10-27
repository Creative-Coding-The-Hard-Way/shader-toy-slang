module value_noise;

import interpolation;

/// Computes a consistent hash of the provided numbers.
///
/// This hash seems fairly well-behaved for integer values in ranges 0-1000 or
/// so. It produces decent looking value noise.
///
public float value_noise_hash(float2 ij) {
    static const float2 DIR = 33.890/sqrt(2);
    let v = 55.9871 * fract((ij+DIR) * DIR);
    return fract(v.x - v.y + v.x*v.y*(ij.x + ij.y));
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
public float vnoise(float2 p) {
    let ij = no_diff int2(floor(p));
    let off = no_diff int2(0, 1);
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
