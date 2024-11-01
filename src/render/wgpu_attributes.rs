use std::collections::HashMap;
use crate::core::attribute::{Attribute, AttributeF32, AttributeF64, AttributeUsize};
use crate::core::resource::{ResourceId, ResourcePools};
use crate::utils::id::UId;

pub struct WGPUAttributes {
    attributes: HashMap<UId, wgpu::Buffer>,
}

impl WGPUAttributes {
    pub fn new() -> Self {
        WGPUAttributes {
            attributes: HashMap::new()
        }
    }

    pub fn borrow<T>(&self, attribute: &ResourceId<T>) -> Option<&wgpu::Buffer> {
        self.attributes.get(&attribute.id)
    }

    // TODO: 支持更新, 通过宏定义实现其他类型
    pub fn update_f32<T>(
        &mut self,
        device: &wgpu::Device,
        pools: &ResourcePools,
        attribute_rid: &ResourceId<AttributeF32>,
    ) {
        if !self.attributes.contains_key(&attribute_rid.id) {
            if let Some(attribute) = pools.borrow::<AttributeF32>().borrow(attribute_rid) {
                self.attributes.insert(attribute_rid.id, create_buffer(
                    device,
                    bytemuck::cast_slice(attribute.borrow_data()),
                    wgpu::BufferUsages::VERTEX,
                ));
            }
        }
    }
}

fn create_buffer(device: &wgpu::Device, contents: &[u8], usage: wgpu::BufferUsages) -> wgpu::Buffer {
    use wgpu::util::DeviceExt;
    device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: None,
        contents,
        usage,
    })
}
