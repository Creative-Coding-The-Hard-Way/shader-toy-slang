searchState.loadedDescShard("sts", 0, "The GLFW application implementation.\nAdd context to an anyhow error which includes a formatted …\nImplementations of this trait can be run with app_main to …\nA helper for toggling fullscreen on a glfw window.\nThe entrypoint for implementations of the App trait.\nReturns the argument unchanged.\nHandles a single GLFW event.\nCalls <code>U::from(self)</code>.\nCreates a new instance of the application. The application …\nCreate a new fullscreen toggle helper.\nSwitch to fullscreen if windowed, switch back to windowed …\nCalled in a loop after all pending events have been …\nMaintains descriptors for a list of texture images. …\nRender a single fullscreen quad using the provided …\nAutomatically recompiles a slang source file any time it …\nA pipeline + resources for rendering sprites.\nA sprite-batch implementation that supports streaming new …\nA utility for managing a Renderpass and Framebuffers for …\nA Texture is a Vulkan Image paired with backing GPU memory …\nAdd a sprite to be rendered by the next call to flush().\nAdds a texture to the atlas. The returned texture number …\nBegin rendering a layer to the frame.\nBegin the render pass in the current frame targetting the …\nBinds the descriptor for the given frame.\nCreates a new fullscreen quad that uses the provided …\nCreates a new sprite layer for use with the provided …\nCreates a new texture with the given dimensions.\nChecks for an updated copy of the compiled source code.\nBorrows the descriptor set layout for the atlas. Useful …\nUpdate the frame’s data and add draw commands to the …\nEnds the render pass for the current frame.\nFlush all current sprites into device memory for rendering …\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nA non-owning handle for the underlying Vulkan image …\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nA non-owning handle for the underlying GPU memory.\nCreates a new recompiler that attempts to compile the …\nCreates a new Renderpass and Framebuffers that target the …\nCreates a new atlas that can hold up to max_textures total …\nCreates the texture loader and underlying resources.\nRebuilds the graphics pipeline using the provided fragment …\nUpdate any swapchain-dependent internal resources.\nBorrow the underlying Vulkan renderpass.\nReset the Sprite Layer’s internal resources.\nSets the layer’s projection matrix. This can be called …\nReturns the most up-to-date copy of the shader’s …\nA non-owning handle for the underlying Vulkan image view.\nThis module defines traits, structs, and functions to …\nThe Vulkan device memory allocator.\nAllocators return blocks of memory that can be used for …\nA CPU accessible buffer with some convenience functions …\nA grow-only descriptor pool that can be reset.\nA Frame is guaranteed to be synchronized such that no two …\nIndicates that the frame is started.\nThe primary synchronization mechanism for managing …\nThe logical Vulkan instance.\nA block allocation that frees itself when dropped.\nThe relative amount of each descriptor type to allocate in …\nThe Vulkan swapchain and associated resources.\nIndicates that the swapchain needs to be rebuilt.\nA utility for synchronously submitting commands to the GPU.\nA CPU accessible buffer with some convenience functions …\nThe Vulkan context is the logical handle for all Vulkan …\nAcquires the index for the next swapchain image.\nAllocates a new buffer and GPU memory for holding data.\nAllocates a buffer with enough space for count copies of …\nCreates a buffer and allocates memory to back it.\nAllocates a new descriptor set with the provided layout.\nCreates an image and allocates memory to back it.\nAllocates device memory according to the given …\nAllocates a new buffer and GPU memory for holding …\nThe device memory allocator.\nReturns a non-owning copy of the Vulkan buffer handle.\nReturns a non-owning copy of the Vulkan buffer handle.\nThe maximum number of items that can be saved in this …\nCompiles the shader file into usable SPIRV.\nReturns the Swapchain’s current extent.\nCreate a new Vulkan instance for the given GLFW window.\nReturns the Swapchain’s image format.\nGet the total number of configured frames in flight.\nFree the allocated block.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nThe graphics queue supports GRAPHICS and presentation …\nThe queue family index for the graphics + present queue.\nReturns the Swapchain’s image views.\nReturns the Swapchain’s image handles.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nReturns true when self is entirely contained by other, …\nReturns the memory-mapped pointer for the DeviceMemory.\nGets a non-owning copy of the underlying DeviceMemory …\nReturns the memory type index for the Block’s device …\nCreate a new Vulkan instance.\nCreates a new Vulkan Context for the first suitable device …\nCreates a new instance with <code>frame_count</code> frames.\nCreates a new Vulkan swapchain.\nCreates a new bump allocator instance.\nReturns the start of the block as a byte offset into …\nReturns the byte-offset into the buffer for the …\nQueues the Frame’s command buffer and swapchain …\nPresents the swapchain image.\nRAII wrappers for Vulkan objects.\nReturns the non-owning Vulkan swapchain handle.\nReset all descriptor pools and descriptor sets.\nReturns the size of the block in bytes.\nThe size of the buffer in bytes.\nCreates a Vulkan shader module from the provided SPIR-V …\nConvert an unaligned slice of bytes into an aligned chunk …\nStarts the next frame in flight.\nReturns a subregion of the current Block.\nUpdates GPU memory with the provided data for the current …\nBlocks until all submitted commands for all frames have …\nWrites data into the GPU memory at the given index.\nWrites data into the GPU memory at the given index.\nRAII wrapper that destroys itself when Dropped.\nRAII wrapper that destroys itself when Dropped.\nRAII wrapper that destroys itself when Dropped.\nRAII wrapper that destroys itself when Dropped.\nRAII wrapper that destroys itself when Dropped.\nA RAII wrapper for the Vulkan Logical Device.\nRAII wrapper that destroys itself when Dropped.\nRAII wrapper that destroys itself when Dropped.\nRAII wrapper that destroys itself when Dropped.\nRAII wrapper that destroys itself when Dropped.\nRAII wrapper that destroys itself when Dropped.\nA RAII wrapper for the ash library entry and instance that …\nRAII wrapper that destroys itself when Dropped.\nRAII wrapper that destroys itself when Dropped.\nRAII wrapper that destroys itself when Dropped.\nRAII wrapper that destroys itself when Dropped.\nRAII wrapper that destroys itself when Dropped.\nRAII wrapper that destroys itself when Dropped.\nRAII wrapper that destroys itself when Dropped.\nRAII wrapper that destroys itself when Dropped.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.")