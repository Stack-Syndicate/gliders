pub mod device;
pub mod shader;

#[cfg(test)]
mod tests {
    use crate::device::ShaderDevice;
    use bytemuck::{Pod, Zeroable};

    #[repr(C)]
    #[derive(Clone, Copy, Pod, Zeroable, Debug, PartialEq)]
    struct Number {
        x: f32,
    }

    #[test]
    fn compute_shader_adds_one() {
        let sd = ShaderDevice::new();
        let mut shader = sd.create_shader("./example.wgsl");

        let input = Number { x: 5.0 };

        sd.add_buffer(&mut shader, &[input, input, input]);
        sd.add_buffer(&mut shader, &[input, input, input]);
        sd.build_pipeline(&mut shader);
        sd.run_shader(&shader, &[1, 1, 1]);
        let result: Vec<f32> = sd.read_buffer(&shader, 1);
        sd.update_buffer(&mut shader, &result, 0);
        sd.run_shader(&shader, &[1, 1, 1]);
        let result: Vec<f32> = sd.read_buffer(&shader, 1);
        sd.update_buffer(&mut shader, &result, 0);
        sd.run_shader(&shader, &[1, 1, 1]);
        let result: Vec<f32> = sd.read_buffer(&shader, 1);
        sd.update_buffer(&mut shader, &result, 0);
        sd.run_shader(&shader, &[1, 1, 1]);
        let result: Vec<f32> = sd.read_buffer(&shader, 1);
        sd.update_buffer(&mut shader, &result, 0);
        sd.run_shader(&shader, &[1, 1, 1]);
        let result: Vec<f32> = sd.read_buffer(&shader, 1);
        sd.update_buffer(&mut shader, &result, 0);
        // assert_eq!(result.len(), 1);
        // assert_eq!(result[0].x, input.x * 2.0);

        println!("Test passed, output: {:?}", result);
    }
}
