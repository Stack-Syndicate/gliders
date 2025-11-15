@group(0) @binding(0)
var<storage, read_write> data: array<f32>;
@group(0) @binding(1)
var<storage, read_write> output: array<f32>;

@compute @workgroup_size(3)
fn main(@builtin(global_invocation_id) id: vec3<u32>) {
    output[id.x] = data[id.x] * 2;
}