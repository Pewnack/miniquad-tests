//! Wireframe Cube Test
//!
//! Renders a rotating 3D wireframe cube using PrimitiveType::Lines.
//!
//! **How to use:**
//! Run with: `cargo run --bin wireframe`
//! You should see a green wireframe cube rotating on a black background.

use glam::{Mat4, Vec3};
use miniquad::*;
use miniquad_test_utils::create_test_window_conf;

#[repr(C)]
struct Vertex {
    pos: [f32; 3],
}

#[repr(C)]
struct Uniforms {
    mvp: Mat4,
}

struct Stage {
    ctx: Box<dyn RenderingBackend>,
    pipeline: Pipeline,
    bindings: Bindings,
    t: f32,
}

impl Stage {
    fn new() -> Stage {
        let mut ctx = window::new_rendering_backend();

        // 8 corners of a unit cube
        let vertices: [Vertex; 8] = [
            Vertex { pos: [-1.0, -1.0, -1.0] }, // 0
            Vertex { pos: [ 1.0, -1.0, -1.0] }, // 1
            Vertex { pos: [ 1.0,  1.0, -1.0] }, // 2
            Vertex { pos: [-1.0,  1.0, -1.0] }, // 3
            Vertex { pos: [-1.0, -1.0,  1.0] }, // 4
            Vertex { pos: [ 1.0, -1.0,  1.0] }, // 5
            Vertex { pos: [ 1.0,  1.0,  1.0] }, // 6
            Vertex { pos: [-1.0,  1.0,  1.0] }, // 7
        ];

        // 12 edges × 2 indices = 24 indices
        let indices: [u16; 24] = [
            0, 1,  1, 2,  2, 3,  3, 0, // back face
            4, 5,  5, 6,  6, 7,  7, 4, // front face
            0, 4,  1, 5,  2, 6,  3, 7, // connecting edges
        ];

        let vertex_buffer = ctx.new_buffer(
            BufferType::VertexBuffer,
            BufferUsage::Immutable,
            BufferSource::slice(&vertices),
        );
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

        let shader = ctx
            .new_shader(
                match ctx.info().backend {
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
            &[VertexAttribute::new("pos", VertexFormat::Float3)],
            shader,
            PipelineParams {
                primitive_type: PrimitiveType::Lines,
                ..Default::default()
            },
        );

        Stage {
            ctx,
            pipeline,
            bindings,
            t: 0.0,
        }
    }
}

impl EventHandler for Stage {
    fn update(&mut self) {
        self.t += 0.016;
    }

    fn draw(&mut self) {
        let (width, height) = window::screen_size();
        let aspect = width / height;

        let projection = Mat4::perspective_rh_gl(60.0_f32.to_radians(), aspect, 0.1, 100.0);
        let view = Mat4::from_translation(Vec3::new(0.0, 0.0, -3.0));
        let rot_y = Mat4::from_rotation_y(self.t);
        let rot_x = Mat4::from_rotation_x(self.t * 0.4);
        let mvp = projection * view * rot_y * rot_x;

        let uniforms = Uniforms { mvp };

        self.ctx.begin_default_pass(PassAction::clear_color(0.0, 0.0, 0.0, 1.0));
        self.ctx.apply_pipeline(&self.pipeline);
        self.ctx.apply_bindings(&self.bindings);
        self.ctx.apply_uniforms(UniformsSource::table(&uniforms));
        self.ctx.draw(0, 24, 1);
        self.ctx.end_render_pass();
        self.ctx.commit_frame();
    }
}

fn main() {
    let conf = create_test_window_conf();
    miniquad::start(conf, || Box::new(Stage::new()));
}

mod shader {
    use miniquad::{ShaderMeta, UniformBlockLayout, UniformDesc, UniformType};

    pub const VERTEX: &str = r#"#version 100
attribute vec3 pos;
uniform mat4 mvp;

void main() {
    gl_Position = mvp * vec4(pos, 1.0);
}
"#;

    pub const FRAGMENT: &str = r#"#version 100
void main() {
    gl_FragColor = vec4(0.0, 1.0, 0.0, 1.0);
}
"#;

    pub const METAL: &str = r#"
    #include <metal_stdlib>

    using namespace metal;

    struct Uniforms
    {
        float4x4 mvp;
    };

    struct Vertex
    {
        float3 pos [[attribute(0)]];
    };

    struct RasterizerData
    {
        float4 position [[position]];
    };

    vertex RasterizerData vertexShader(Vertex v [[stage_in]], constant Uniforms& uniforms [[buffer(0)]])
    {
        RasterizerData out;
        out.position = uniforms.mvp * float4(v.pos, 1.0);
        return out;
    }

    fragment float4 fragmentShader(RasterizerData in [[stage_in]])
    {
        return float4(0.0, 1.0, 0.0, 1.0);
    }"#;

    pub fn meta() -> ShaderMeta {
        ShaderMeta {
            images: vec![],
            uniforms: UniformBlockLayout {
                uniforms: vec![UniformDesc::new("mvp", UniformType::Mat4)],
            },
        }
    }
}
