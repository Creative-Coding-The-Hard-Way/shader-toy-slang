# Live Reload

A Vulkan demo that presents a fullscreen quad every frame. The fragment shader
source is provided as a command line argument and will be dynamically reloaded
any time the source is modified and compiles successfully.

## Usage

> cargo run --example live_reload -- <path-to-slang-src>

## Keybinds

- Esc: exit the application
- Space: toggle fullscreen

## Uniform Data

The fragment shader has access to per-frame uniform data. It can be accessed
by adding the following to the top of the fragment shader definition:

```
struct FrameData {
    float2 mouse_pos;
    float2 screen_size;
    float dt;
    float time;
};

[[vk_binding(0, 0)]]
ConstantBuffer<FrameData> frame;
```

## Per-Fragment Data

The Vertex shader emits the position and uv as varying float2s for access within
the fragment shader:

```
float2 pos : SmoothOutput_pos;
float2 uv: SmoothOutput_uv;
```

`pos` is in the range [-1, 1] and represents standard euclidian
coordinates. (e.g. (-1, -1) is the bottom left, (0, 0) is the center of the
screen, and (1, 1) is the top right).

`uv` is in the range [0, 1] and represents "UV" coordinates where (0, 0) is the
bottom left of the screen and (1, 1) is the top right of the screen.

## Starting Point

This is a bare-minimum fragment shader that turns the entire screen blue.

```
struct FrameData {
    float2 mouse_pos;
    float2 screen_size;
    float dt;
    float time;
};

[[vk_binding(0, 0)]]
ConstantBuffer<FrameData> frame;

[shader("fragment")]
float4 main(
    float2 pos: SmoothOutput_pos,
    float2 uv: SmoothOutput_uv
) : SV_Target {
    return float4(0.1, 0.1, 0.5, 1.0);
}
```
