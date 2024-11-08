use {
    crate::{
        graphics::vulkan::{raii, VulkanContext},
        trace,
    },
    anyhow::{Context, Result},
    ash::vk,
    std::sync::Arc,
};

/// The relative amount of each descriptor type to allocate in each Descriptor
/// Pool.
pub type PoolRatio = (vk::DescriptorType, u32);

/// A grow-only descriptor pool that can be reset.
///
/// Internally, there can actually be many DescriptorPools. When a pool is out
/// of space for allocations, then a new pool will be created, and so on...
#[derive(Debug)]
pub struct DescriptorBumpAllocator {
    max_sets_per_pool: u32,
    pool_sizes: Vec<vk::DescriptorPoolSize>,
    free_pools: Vec<raii::DescriptorPool>,
    filled_pools: Vec<raii::DescriptorPool>,
    ctx: Arc<VulkanContext>,
}

impl DescriptorBumpAllocator {
    /// Creates a new bump allocator instance.
    ///
    /// # Params
    ///
    /// * `max_sets_per_pool` - controls the size of each DescriptorPool. Larger
    ///   values mean fewer calls to vkCreateDescriptorPool, but may be wasteful
    ///   in memory usage.
    /// * `pool_ratios` - controls the relative ratios of how many descriptor
    ///   types can be allocated from each pool. This doesn't need to be perfect
    ///   (when in doubt, overestimate), but inaccuraces can result in wasted
    ///   pools.
    pub fn new<I>(
        ctx: Arc<VulkanContext>,
        max_sets_per_pool: u32,
        pool_ratios: I,
    ) -> Result<Self>
    where
        I: IntoIterator<Item = PoolRatio>,
    {
        let pool_sizes: Vec<vk::DescriptorPoolSize> = pool_ratios
            .into_iter()
            .map(|(descriptor_type, relative_size)| vk::DescriptorPoolSize {
                ty: descriptor_type,
                descriptor_count: max_sets_per_pool * relative_size,
            })
            .collect();
        let free_pools = vec![raii::DescriptorPool::new(
            ctx.device.clone(),
            &vk::DescriptorPoolCreateInfo {
                max_sets: max_sets_per_pool,
                pool_size_count: pool_sizes.len() as u32,
                p_pool_sizes: pool_sizes.as_ptr(),
                ..Default::default()
            },
        )?];
        Ok(Self {
            max_sets_per_pool,
            pool_sizes,
            free_pools,
            filled_pools: vec![],
            ctx,
        })
    }

    /// Allocates a new descriptor set with the provided layout.
    pub fn allocate_descriptor_set(
        &mut self,
        descriptor_set_layout: &raii::DescriptorSetLayout,
    ) -> Result<vk::DescriptorSet> {
        let first_attempt = unsafe {
            self.ctx
                .allocate_descriptor_sets(&vk::DescriptorSetAllocateInfo {
                    descriptor_pool: self.current_pool().raw,
                    descriptor_set_count: 1,
                    p_set_layouts: &descriptor_set_layout.raw,
                    ..Default::default()
                })
        };
        if let Ok(descriptor_sets) = first_attempt {
            return Ok(descriptor_sets[0]);
        }

        // The first attempt failed, so get a new current pool and try again

        self.mark_current_pool_filled().with_context(trace!(
            "Error while updating the current descriptor pool!\n\n{:#?}",
            self
        ))?;

        let second_attempt = unsafe {
            self.ctx
                .allocate_descriptor_sets(&vk::DescriptorSetAllocateInfo {
                    descriptor_pool: self.current_pool().raw,
                    descriptor_set_count: 1,
                    p_set_layouts: &descriptor_set_layout.raw,
                    ..Default::default()
                })
        };

        match second_attempt {
            Ok(descriptor_sets) => Ok(descriptor_sets[0]),
            Err(err) => Err(err).with_context(trace!(
                "Second attempt to allocate descriptor set failed!"
            )),
        }
    }

    /// Reset all descriptor pools and descriptor sets.
    ///
    /// # Safety
    ///
    /// This method is unsafe because:
    /// * the caller must synchronize access to the descriptor sets. e.g. the
    ///   caller must ensure that no descriptor set is in-use when this method
    ///   is called.
    pub unsafe fn reset(&mut self) -> Result<()> {
        self.free_pools.extend(self.filled_pools.drain(0..));

        for pool in &self.free_pools {
            unsafe {
                self.ctx
                    .reset_descriptor_pool(
                        pool.raw,
                        vk::DescriptorPoolResetFlags::empty(),
                    )
                    .with_context(trace!("Error resetting command pool!"))?;
            }
        }
        Ok(())
    }

    // Private API ------------------------------------------------------------

    /// Returns the current free pool that can be used for allocation.
    fn current_pool(&self) -> &raii::DescriptorPool {
        self.free_pools.last().unwrap()
    }

    /// Indicate that the current pool can no longer be used for allocations.
    ///
    /// Allocates a new free pool if there are no remaining free pools.
    fn mark_current_pool_filled(&mut self) -> Result<()> {
        self.filled_pools.push(self.free_pools.pop().unwrap());
        if self.free_pools.is_empty() {
            self.free_pools.push(
                raii::DescriptorPool::new(
                    self.ctx.device.clone(),
                    &vk::DescriptorPoolCreateInfo {
                        max_sets: self.max_sets_per_pool,
                        pool_size_count: self.pool_sizes.len() as u32,
                        p_pool_sizes: self.pool_sizes.as_ptr(),
                        ..Default::default()
                    },
                )
                .with_context(trace!(
                    "Error while creating new descriptor pool for {:#?}",
                    self
                ))?,
            );
        }
        Ok(())
    }
}
