use {
    super::ComposableAllocator,
    crate::graphics::vulkan::{
        allocator::{AllocationRequirements, HumanizedSize},
        Block,
    },
    anyhow::Result,
};

pub trait LabelledAllocatorBuilder {
    /// Replace self with an allocator that reports metrics at exit.
    fn description(
        self,
        label: impl Into<String>,
        description: impl Into<String>,
    ) -> ReportingAllocator<Self>
    where
        Self: Sized + ComposableAllocator,
    {
        ReportingAllocator::with_description(label, description, self)
    }
}
impl<T> LabelledAllocatorBuilder for T where T: ComposableAllocator {}

#[derive(Copy, Clone, Default, PartialEq, Eq)]
struct Metrics {
    concurrent_allocations: u64,
    max_concurrent_allocations: u64,
    max_allocation_size: u64,
}

impl std::fmt::Debug for Metrics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Metrics")
            .field("concurrent_allocations", &self.concurrent_allocations)
            .field(
                "max_concurrent_allocations",
                &self.max_concurrent_allocations,
            )
            .field(
                "max_allocation_size",
                &HumanizedSize(self.max_allocation_size),
            )
            .finish()
    }
}

/// An allocator decorator that records metrics for interesting interactions.
pub struct ReportingAllocator<A: ComposableAllocator> {
    allocator: A,
    label: String,
    description: String,
    metrics: Metrics,
}

impl<A: ComposableAllocator> Drop for ReportingAllocator<A> {
    fn drop(&mut self) {
        log::debug!(
            indoc::indoc! {
                "
                {}
                {}

                Report:
                {:#?}
                "
            },
            self.label,
            self.description,
            self.metrics
        )
    }
}

impl<A: ComposableAllocator> ReportingAllocator<A> {
    /// Creates a new reporting allocator.
    ///
    /// The provided label and description appear in the report when dropped.
    pub fn with_description(
        label: impl Into<String>,
        description: impl Into<String>,
        allocator: A,
    ) -> Self {
        Self {
            allocator,
            label: label.into(),
            description: description.into(),
            metrics: Metrics::default(),
        }
    }
}

impl<A: ComposableAllocator> ComposableAllocator for ReportingAllocator<A> {
    fn owns(&self, block: &Block) -> bool {
        self.allocator.owns(block)
    }

    fn allocate_memory(
        &mut self,
        requirements: AllocationRequirements,
    ) -> Result<Block> {
        let block = self.allocator.allocate_memory(requirements)?;
        self.metrics.concurrent_allocations += 1;
        self.metrics.max_concurrent_allocations = self
            .metrics
            .max_concurrent_allocations
            .max(self.metrics.concurrent_allocations);
        self.metrics.max_allocation_size = self
            .metrics
            .max_allocation_size
            .max(requirements.allocation_size);
        Ok(block)
    }

    fn free_memory(&mut self, block: &Block) {
        self.metrics.concurrent_allocations -= 1;
        self.allocator.free_memory(block);
    }
}
