//! Starfield Test
//!
//! Renders a simple animated starfield using PrimitiveType::Points.
//! Stars are placed in 3D space and projected onto the screen each frame,
//! giving the illusion of flying through space.
//!
//! **What it tests:**
//! - PrimitiveType::Points rendering
//! - Dynamic vertex buffer updates (BufferUsage::Stream)
//! - Point size variation via gl_PointSize / [[point_size]]
//!
//! **How to use:**
//! Run with: `cargo run --bin starfield`
//! You should see white stars streaming toward you on a dark background.

use miniquad::*;
use miniquad_test_utils::create_test_window_conf;

const NUM_STARS: usize = 400;
const FAR_Z: f32 = 5.0;
const NEAR_Z: f32 = 0.1;
const SPEED: f32 = 0.025;

#[repr(C)]
struct Vertex {
    pos: [f32; 2],
    brightness: f32,
}

struct Stage {
    ctx: Box<dyn RenderingBackend>,
    pipeline: Pipeline,
    bindings: Bindings,
    /// Each star is stored as [x, y, z] in world space.
    stars: Vec<[f32; 3]>,
}

/// Pseudo-random spread using a simple LCG so we have no external RNG dependency.
fn lcg(seed: u64) -> f32 {
    let x = seed.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
    // Map to [-1, 1]: take the upper 32 bits, cast to u32 (unsigned) before f32
    ((x >> 32) as u32 as f32 / u32::MAX as f32) * 2.0 - 1.0
}

impl Stage {
    fn new() -> Stage {
        let mut ctx = window::new_rendering_backend();

        // Spread stars randomly in x/y and evenly across the depth range.
        let stars: Vec<[f32; 3]> = (0..NUM_STARS)
            .map(|i| {
                let seed = i as u64;
                let x = lcg(seed ^ 0xdeadbeef) * 2.0;
                let y = lcg(seed ^ 0xcafebabe) * 2.0;
                let z = NEAR_Z + (i as f32 / NUM_STARS as f32) * (FAR_Z - NEAR_Z);
                [x, y, z]
            })
            .collect();

        // Initial vertex data (will be overwritten every frame).
        let vertices: Vec<Vertex> = (0..NUM_STARS)
            .map(|_| Vertex { pos: [0.0, 0.0], brightness: 1.0 })
            .collect();

        let vertex_buffer = ctx.new_buffer(
            BufferType::VertexBuffer,
            BufferUsage::Stream,
            BufferSource::slice(&vertices),
        );

        // One index per point — sequential.
        let indices: Vec<u16> = (0..NUM_STARS as u16).collect();
        let index_buffer = ctx.new_buffer(
            BufferType::IndexBuffer,
            BufferUsage::Immutable,
            BufferSource::slice(&indices),
        );

        let bindings = Bindings {
            vertex_buffers: vec![vertex_buffer],
            index_buffer,
            images: vec![],
        };

        let backend = ctx.info().backend;

        // On desktop OpenGL, gl_PointSize in the vertex shader is only respected
        // when GL_PROGRAM_POINT_SIZE is enabled. Metal's [[point_size]] always works.
        if backend == Backend::OpenGl {
            unsafe { miniquad::gl::glEnable(miniquad::gl::GL_PROGRAM_POINT_SIZE) };
        }

        let shader = ctx
            .new_shader(
                match backend {
                    Backend::OpenGl => ShaderSource::Glsl {
                        vertex: shader::VERTEX,
                        fragment: shader::FRAGMENT,
                    },
                    Backend::Metal => ShaderSource::Msl {
                        program: shader::METAL,
                    },
                },
                shader::meta(),
            )
            .unwrap();

        let pipeline = ctx.new_pipeline(
            &[BufferLayout::default()],
            &[
                VertexAttribute::new("pos", VertexFormat::Float2),
                VertexAttribute::new("brightness", VertexFormat::Float1),
            ],
            shader,
            PipelineParams {
                primitive_type: PrimitiveType::Points,
                ..Default::default()
            },
        );

        Stage { ctx, pipeline, bindings, stars }
    }
}

impl EventHandler for Stage {
    fn update(&mut self) {
        for (i, star) in self.stars.iter_mut().enumerate() {
            star[2] -= SPEED;
            if star[2] < NEAR_Z {
                // Recycle star at the far end with a new x/y position.
                let seed = i as u64 ^ (star[2].to_bits() as u64);
                star[0] = lcg(seed ^ 0xdeadbeef) * 2.0;
                star[1] = lcg(seed ^ 0xcafebabe) * 2.0;
                star[2] += FAR_Z - NEAR_Z;
            }
        }
    }

    fn draw(&mut self) {
        // Project each star to 2D NDC coordinates.
        let vertices: Vec<Vertex> = self.stars.iter().map(|s| {
            let x = s[0] / s[2];
            let y = s[1] / s[2];
            // Stars closer to the viewer are brighter.
            let t = (s[2] - NEAR_Z) / (FAR_Z - NEAR_Z);
            let brightness = 1.0 - t;
            Vertex { pos: [x, y], brightness }
        }).collect();

        self.ctx.buffer_update(
            self.bindings.vertex_buffers[0],
            BufferSource::slice(&vertices),
        );

        self.ctx.begin_default_pass(PassAction::clear_color(0.0, 0.0, 0.05, 1.0));
        self.ctx.apply_pipeline(&self.pipeline);
        self.ctx.apply_bindings(&self.bindings);
        self.ctx.draw(0, NUM_STARS as i32, 1);
        self.ctx.end_render_pass();
        self.ctx.commit_frame();
    }
}

fn main() {
    let conf = create_test_window_conf();
    miniquad::start(conf, || Box::new(Stage::new()));
}

mod shader {
    use miniquad::{ShaderMeta, UniformBlockLayout};

    pub const VERTEX: &str = r#"#version 100
attribute vec2 pos;
attribute float brightness;

varying float v_brightness;

void main() {
    gl_Position = vec4(pos, 0.0, 1.0);
    gl_PointSize = 1.5 + brightness * 3.5;
    v_brightness = brightness;
}
"#;

    pub const FRAGMENT: &str = r#"#version 100
precision mediump float;

varying float v_brightness;

void main() {
    gl_FragColor = vec4(v_brightness, v_brightness, v_brightness, 1.0);
}
"#;

    pub const METAL: &str = r#"
    #include <metal_stdlib>

    using namespace metal;

    struct Vertex
    {
        float2 pos [[attribute(0)]];
        float  brightness [[attribute(1)]];
    };

    struct RasterizerData
    {
        float4 position [[position]];
        float  point_size [[point_size]];
        float  brightness [[user(locn0)]];
    };

    vertex RasterizerData vertexShader(Vertex v [[stage_in]])
    {
        RasterizerData out;
        out.position   = float4(v.pos, 0.0, 1.0);
        out.point_size = 1.5 + v.brightness * 3.5;
        out.brightness = v.brightness;
        return out;
    }

    fragment float4 fragmentShader(RasterizerData in [[stage_in]])
    {
        return float4(in.brightness, in.brightness, in.brightness, 1.0);
    }"#;

    pub fn meta() -> ShaderMeta {
        ShaderMeta {
            images: vec![],
            uniforms: UniformBlockLayout { uniforms: vec![] },
        }
    }
}
