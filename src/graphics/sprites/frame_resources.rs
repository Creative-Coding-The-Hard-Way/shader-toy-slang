use {
    super::{LayerData, SpriteBatch},
    crate::graphics::{
        vulkan::{raii, DescriptorBumpAllocator, UniformBuffer, VulkanContext},
        Sprite,
    },
    anyhow::Result,
    ash::vk::{self},
    std::{collections::HashMap, sync::Arc},
};

/// Per-frame resources for drawing sprites.
pub struct FrameResources {
    layer_descriptor_set: vk::DescriptorSet,
    cached_batch_descriptors: HashMap<vk::Buffer, vk::DescriptorSet>,
    ctx: Arc<VulkanContext>,
}

impl FrameResources {
    /// Create a new instance of the Sprite Layer's per-frame resources.
    pub fn new(
        ctx: Arc<VulkanContext>,
        descriptor_allocator: &mut DescriptorBumpAllocator,
        layer_descriptor_set_layout: &raii::DescriptorSetLayout,
        layer_buffer: &UniformBuffer<LayerData>,
        index: usize,
    ) -> Result<Self> {
        let layer_descriptor_set = descriptor_allocator
            .allocate_descriptor_set(layer_descriptor_set_layout)?;

        unsafe {
            let buffer_info = vk::DescriptorBufferInfo {
                buffer: layer_buffer.buffer(),
                offset: layer_buffer.offset_for_index(index),
                range: size_of::<LayerData>() as u64,
            };
            ctx.update_descriptor_sets(
                &[vk::WriteDescriptorSet {
                    dst_set: layer_descriptor_set,
                    dst_binding: 0,
                    dst_array_element: 0,
                    descriptor_count: 1,
                    descriptor_type: vk::DescriptorType::UNIFORM_BUFFER,
                    p_image_info: std::ptr::null(),
                    p_buffer_info: &buffer_info,
                    p_texel_buffer_view: std::ptr::null(),
                    ..Default::default()
                }],
                &[],
            )
        };

        Ok(Self {
            layer_descriptor_set,
            cached_batch_descriptors: HashMap::with_capacity(10),
            ctx,
        })
    }

    pub fn get_layer_descriptor(&self) -> vk::DescriptorSet {
        self.layer_descriptor_set
    }

    pub fn get_batch_descriptor(
        &mut self,
        batch: &impl SpriteBatch,
        descriptor_allocator: &mut DescriptorBumpAllocator,
        batch_descriptor_set_layout: &raii::DescriptorSetLayout,
    ) -> Result<vk::DescriptorSet> {
        if let Some(&descriptor_set) =
            self.cached_batch_descriptors.get(&batch.buffer())
        {
            return Ok(descriptor_set);
        }

        let batch_descriptor = descriptor_allocator
            .allocate_descriptor_set(batch_descriptor_set_layout)?;

        let buffer_info = vk::DescriptorBufferInfo {
            buffer: batch.buffer(),
            offset: 0,
            range: batch.count() as u64 * size_of::<Sprite>() as u64,
        };
        unsafe {
            self.ctx.update_descriptor_sets(
                &[vk::WriteDescriptorSet {
                    dst_set: batch_descriptor,
                    dst_binding: 0,
                    dst_array_element: 0,
                    descriptor_count: 1,
                    descriptor_type: vk::DescriptorType::STORAGE_BUFFER,
                    p_image_info: std::ptr::null(),
                    p_buffer_info: &buffer_info,
                    p_texel_buffer_view: std::ptr::null(),
                    ..Default::default()
                }],
                &[],
            );
        }

        self.cached_batch_descriptors
            .insert(batch.buffer(), batch_descriptor);

        Ok(batch_descriptor)
    }
}
