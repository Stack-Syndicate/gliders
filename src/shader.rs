use wgpu::util::DeviceExt;
use bytemuck::Pod;

pub struct ComputeShader {
    pub(crate) pipeline: Option<wgpu::ComputePipeline>,
    pub(crate) buffers: Vec<wgpu::Buffer>,
    pub(crate) shader_module: wgpu::ShaderModule,
    pub(crate) bind_group: Option<wgpu::BindGroup>,
}
impl ComputeShader {
    pub fn run(&self, device: &wgpu::Device, queue: &wgpu::Queue, workgroups: &[u32; 3]) {
        let mut encoder =
            device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        {
            let mut cpass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: None,
                ..Default::default()
            });
            cpass.set_pipeline(&self.pipeline.clone().unwrap());
            cpass.set_bind_group(0, &self.bind_group, &[]);
            cpass.dispatch_workgroups(workgroups[0], workgroups[1], workgroups[2]);
        }
        queue.submit(Some(encoder.finish()));
    }
    pub fn add_buffer<T: Pod>(&mut self, device: &wgpu::Device, data: &[T]) {
        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Compute Buffer"),
            contents: bytemuck::cast_slice(data),
            usage: wgpu::BufferUsages::STORAGE
                | wgpu::BufferUsages::COPY_DST
                | wgpu::BufferUsages::COPY_SRC,
        });

        self.buffers.push(buffer.clone());
    }
    pub fn read_buffer<T: Pod>(&self, device: &wgpu::Device, queue: &wgpu::Queue, index: usize) -> Vec<T> {
        let src_buffer = &self.buffers[index];
        let buffer_size = src_buffer.size();

        let staging_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Staging Buffer"),
            size: buffer_size,
            usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Read Encoder"),
        });
        encoder.copy_buffer_to_buffer(src_buffer, 0, &staging_buffer, 0, buffer_size);
        queue.submit(Some(encoder.finish()));

        let mapped = std::sync::Arc::new(std::sync::Mutex::new(None));
        let mapped_clone = mapped.clone();
        staging_buffer
            .slice(..)
            .map_async(wgpu::MapMode::Read, move |r| {
                *mapped_clone.lock().unwrap() = Some(r);
            });

        device
            .poll(wgpu::wgt::PollType::Wait {
                submission_index: None,
                timeout: None,
            })
            .unwrap();

        // Check mapping result
        let res = mapped.lock().unwrap().take().unwrap();
        res.unwrap();

        // Read data
        let data = staging_buffer.slice(..).get_mapped_range();
        let result: Vec<T> = bytemuck::cast_slice(&data).to_vec();
        drop(data);
        staging_buffer.unmap();

        result
    }
    pub fn update_buffer<T: Pod>(&mut self, queue: &wgpu::Queue, data: &[T], index: usize) {
        queue.write_buffer(&self.buffers[index], 0, bytemuck::cast_slice(data));
    }
    pub fn build_pipeline(&mut self, device: &wgpu::Device) {
        let entries: Vec<wgpu::BindGroupLayoutEntry> = self
            .buffers
            .iter()
            .enumerate()
            .map(|(i, _)| wgpu::BindGroupLayoutEntry {
                binding: i as u32,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: false },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            })
            .collect();
        let bind_group_entries: Vec<wgpu::BindGroupEntry> = self
            .buffers
            .iter()
            .enumerate()
            .map(|(i, buf)| wgpu::BindGroupEntry {
                binding: i as u32,
                resource: buf.as_entire_binding(),
            })
            .collect();

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &entries,
        });
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("ComputeShader BindGroup"),
            layout: &bind_group_layout,
            entries: &bind_group_entries,
        });
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        self.pipeline = Some(
            device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                label: None,
                layout: Some(&pipeline_layout),
                module: &self.shader_module,
                entry_point: Some("main"),
                compilation_options: Default::default(),
                cache: Default::default(),
            }),
        );

        self.bind_group = Some(bind_group);
    }
}