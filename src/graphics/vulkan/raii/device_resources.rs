use {
    crate::{graphics::vulkan::raii, trace},
    anyhow::{Context, Result},
    ash::vk::{self},
    std::sync::Arc,
};

macro_rules! resource {
    (
        $name: ident,
        $raw_type: ty,
        $create_info_type: ty,
        $create: ident,
        $destroy: ident
    ) => {
        /// RAII wrapper that destroys itself when Dropped.
        ///
        /// The owner is responsible for dropping Vulkan resources in the
        /// correct order.
        pub struct $name {
            pub raw: $raw_type,
            pub device: Arc<raii::Device>,
        }

        impl $name {
            pub fn new(
                device: Arc<raii::Device>,
                create_info: &$create_info_type,
            ) -> Result<Self> {
                let raw = unsafe { device.$create(&create_info, None)? };
                Ok(Self { device, raw })
            }
        }

        impl std::fmt::Debug for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_struct(stringify!($name))
                    .field("raw", &self.raw)
                    .field("device", &self.device)
                    .finish()
            }
        }

        impl std::ops::Deref for $name {
            type Target = $raw_type;

            fn deref(&self) -> &Self::Target {
                &self.raw
            }
        }

        impl Drop for $name {
            fn drop(&mut self) {
                unsafe { self.device.$destroy(self.raw, None) }
            }
        }
    };
}

resource!(
    Sampler,
    vk::Sampler,
    vk::SamplerCreateInfo,
    create_sampler,
    destroy_sampler
);

resource!(
    Image,
    vk::Image,
    vk::ImageCreateInfo,
    create_image,
    destroy_image
);

resource!(
    Fence,
    vk::Fence,
    vk::FenceCreateInfo,
    create_fence,
    destroy_fence
);

resource!(
    DeviceMemory,
    vk::DeviceMemory,
    vk::MemoryAllocateInfo,
    allocate_memory,
    free_memory
);

resource!(
    Buffer,
    vk::Buffer,
    vk::BufferCreateInfo,
    create_buffer,
    destroy_buffer
);

resource!(
    DescriptorPool,
    vk::DescriptorPool,
    vk::DescriptorPoolCreateInfo,
    create_descriptor_pool,
    destroy_descriptor_pool
);

resource!(
    DescriptorSetLayout,
    vk::DescriptorSetLayout,
    vk::DescriptorSetLayoutCreateInfo,
    create_descriptor_set_layout,
    destroy_descriptor_set_layout
);

resource!(
    ImageView,
    vk::ImageView,
    vk::ImageViewCreateInfo,
    create_image_view,
    destroy_image_view
);

resource!(
    Semaphore,
    vk::Semaphore,
    vk::SemaphoreCreateInfo,
    create_semaphore,
    destroy_semaphore
);

resource!(
    CommandPool,
    vk::CommandPool,
    vk::CommandPoolCreateInfo,
    create_command_pool,
    destroy_command_pool
);

resource!(
    RenderPass,
    vk::RenderPass,
    vk::RenderPassCreateInfo,
    create_render_pass,
    destroy_render_pass
);

resource!(
    Framebuffer,
    vk::Framebuffer,
    vk::FramebufferCreateInfo,
    create_framebuffer,
    destroy_framebuffer
);

resource!(
    ShaderModule,
    vk::ShaderModule,
    vk::ShaderModuleCreateInfo,
    create_shader_module,
    destroy_shader_module
);

resource!(
    PipelineLayout,
    vk::PipelineLayout,
    vk::PipelineLayoutCreateInfo,
    create_pipeline_layout,
    destroy_pipeline_layout
);

/// RAII wrapper that destroys itself when Dropped.
///
/// The owner is responsible for dropping Vulkan resources in the
/// correct order.
pub struct Pipeline {
    pub raw: vk::Pipeline,
    pub device: Arc<raii::Device>,
}

impl Pipeline {
    pub fn new_graphics_pipeline(
        device: Arc<raii::Device>,
        create_info: &vk::GraphicsPipelineCreateInfo,
    ) -> Result<Self> {
        let result = unsafe {
            device.create_graphics_pipelines(
                vk::PipelineCache::null(),
                &[*create_info],
                None,
            )
        };
        let raw = match result {
            Ok(pipelines) => pipelines[0],
            Err((_, result)) => {
                return Err(result).with_context(trace!(
                    "Error while creating graphics pipeline!"
                ));
            }
        };
        Ok(Self { device, raw })
    }

    pub fn new_compute_pipeline(
        device: Arc<raii::Device>,
        create_info: &vk::ComputePipelineCreateInfo,
    ) -> Result<Self> {
        let result = unsafe {
            device.create_compute_pipelines(
                vk::PipelineCache::null(),
                &[*create_info],
                None,
            )
        };
        let raw = match result {
            Ok(pipelines) => pipelines[0],
            Err((_, result)) => {
                return Err(result).with_context(trace!(
                    "Error while creating compute pipeline!"
                ));
            }
        };
        Ok(Self { device, raw })
    }
}

impl std::fmt::Debug for Pipeline {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!($name))
            .field("raw", &self.raw)
            .field("device", &self.device)
            .finish()
    }
}

impl std::ops::Deref for Pipeline {
    type Target = vk::Pipeline;

    fn deref(&self) -> &Self::Target {
        &self.raw
    }
}

impl Drop for Pipeline {
    fn drop(&mut self) {
        unsafe { self.device.destroy_pipeline(self.raw, None) }
    }
}
