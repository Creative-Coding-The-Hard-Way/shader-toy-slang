# Shader-Toy-Slang

The library + example in this binary exist because I wanted to experiment with
[shader-slang](https://github.com/shader-slang/slang) which is now included in
the Vulkan SDK (see the [1.3.296.0 release notes](https://vulkan.lunarg.com/doc/sdk/1.3.296.0/windows/release_notes.html)).

## Demos

Working shader demos are placed in the [demos](./demos/) directory. Demos use
the [Live Reload](./examples/live_reload/) binary.

## Dependencies

- `slangc`: The [shader-slang](https://github.com/shader-slang/slang) compiler
  is expected to be on the system PATH. The compiler binary is
  available as part of the Vulkan SDK.
