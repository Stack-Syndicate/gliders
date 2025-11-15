use wgpu::ShaderModuleDescriptor;
use pollster::FutureExt;
use bytemuck::Pod;
use crate::shader::ComputeShader;

pub struct ShaderDevice {
    pub(crate) device: wgpu::Device,
    pub(crate) queue: wgpu::Queue,
}
impl ShaderDevice {
    pub fn new() -> Self {
        let instance = wgpu::Instance::new(&Default::default());
        let adapter = instance
            .request_adapter(&Default::default())
            .block_on()
            .unwrap();
        let (device, queue) = adapter
            .request_device(&Default::default())
            .block_on()
            .unwrap();
        Self { device, queue }
    }
    pub fn create_shader(&self, path: &str) -> ComputeShader {
        let shader_source = std::fs::read_to_string(path).expect("Shader not found.");
        let shader = self.device.create_shader_module(ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(shader_source.into()),
        });
        ComputeShader {
            pipeline: None,
            buffers: Vec::new(),
            shader_module: shader,
            bind_group: None,
        }
    }
    pub fn run_shader(&self, shader: &ComputeShader, workgroups: &[u32; 3]) {
        shader.run(&self.device, &self.queue, workgroups);
    }
    pub fn add_buffer<T: Pod>(&self, shader: &mut ComputeShader, data: &[T]) {
        shader.add_buffer(&self.device, data);
    }
    pub fn build_pipeline(&self, shader: &mut ComputeShader) {
        shader.build_pipeline(&self.device);
    }
    pub fn update_buffer<T: Pod>(&self, shader: &mut ComputeShader, data: &[T], index: usize) {
        shader.update_buffer(&self.queue, data, index);
    }
    pub fn read_buffer<T: Pod>(&self, shader: &ComputeShader, index:usize) -> Vec<T> {
        shader.read_buffer(&self.device, &self.queue, index)
    }
}
