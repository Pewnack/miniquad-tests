//! Viewport Test
//!
//! Renders a shaded triangle inside a restricted viewport centered in the window.
//! A light grey rectangle is drawn around the viewport border to make its bounds visible.
//!
//! **What it tests:**
//! - `apply_viewport` to restrict rendering to a sub-region of the window
//! - Per-vertex colour interpolation (shaded triangle)
//! - Multi-draw within a single render pass (border rect → viewport background → triangle)
//!
//! **How to use:**
//! Run with: `cargo run --bin viewport`
//! You should see:
//! - A dark grey background filling the window
//! - A light grey rectangle marking the viewport bounds
//! - A dark blue interior inside the viewport
//! - A colour-shaded triangle (red top, green bottom-left, blue bottom-right)

use miniquad::*;
use miniquad_test_utils::create_test_window_conf;

// Window dimensions (must match create_test_window_conf).
const WIN_W: f32 = 800.0;
const WIN_H: f32 = 600.0;

// Viewport: centred, half the window in each dimension.
const VP_X: i32 = 200;
const VP_Y: i32 = 150; // OpenGL y-from-bottom = WIN_H - VP_Y - VP_H = 600-150-300 = 150
const VP_W: i32 = 400;
const VP_H: i32 = 300;

// Border thickness around the viewport (pixels).
const BORDER: f32 = 8.0;

#[repr(C)]
struct Vertex {
    pos: [f32; 2],
    color: [f32; 4],
}

/// Convert window pixel coordinate (y = 0 at top) to full-window NDC.
fn px_to_ndc(px: f32, py: f32) -> [f32; 2] {
    let nx = px / WIN_W * 2.0 - 1.0;
    let ny = 1.0 - py / WIN_H * 2.0;
    [nx, ny]
}

struct Stage {
    ctx: Box<dyn RenderingBackend>,
    pipeline: Pipeline,
    /// Single vertex buffer holding all geometry (border rect + bg quad + triangle).
    vertex_buffer: BufferId,
    /// Single index buffer.
    index_buffer: BufferId,
}

impl Stage {
    fn new() -> Stage {
        let mut ctx = window::new_rendering_backend();

        // ── Grey border rectangle ─────────────────────────────────────────────
        // Drawn in full-window NDC so it appears at the exact pixel position.
        // Viewport window-pixel bounds: (VP_X, VP_Y) with size VP_W × VP_H.
        // (window y = 0 at top; NDC y = +1 at top)
        let bx0 = VP_X as f32 - BORDER; // left pixel
        let bx1 = VP_X as f32 + VP_W as f32 + BORDER; // right pixel
        let by0 = VP_Y as f32 - BORDER; // top pixel
        let by1 = VP_Y as f32 + VP_H as f32 + BORDER; // bottom pixel

        let grey = [0.82_f32, 0.82, 0.82, 1.0];
        let tl_b = px_to_ndc(bx0, by0);
        let tr_b = px_to_ndc(bx1, by0);
        let br_b = px_to_ndc(bx1, by1);
        let bl_b = px_to_ndc(bx0, by1);

        // ── Dark viewport background quad ─────────────────────────────────────
        // Drawn after apply_viewport so NDC (-1,−1)–(1,1) maps to the viewport.
        let dark = [0.05_f32, 0.05, 0.10, 1.0];

        // ── Shaded triangle (viewport-local NDC) ──────────────────────────────
        let red   = [1.0_f32, 0.2, 0.2, 1.0];
        let green = [0.2_f32, 1.0, 0.3, 1.0];
        let blue  = [0.2_f32, 0.4, 1.0, 1.0];

        #[rustfmt::skip]
        let vertices: &[Vertex] = &[
            // 0–3  grey border rect (full-window NDC)
            Vertex { pos: tl_b,            color: grey  },
            Vertex { pos: tr_b,            color: grey  },
            Vertex { pos: br_b,            color: grey  },
            Vertex { pos: bl_b,            color: grey  },
            // 4–7  dark viewport background (viewport-local NDC)
            Vertex { pos: [-1.0,  1.0],    color: dark  },
            Vertex { pos: [ 1.0,  1.0],    color: dark  },
            Vertex { pos: [ 1.0, -1.0],    color: dark  },
            Vertex { pos: [-1.0, -1.0],    color: dark  },
            // 8–10 shaded triangle (viewport-local NDC)
            Vertex { pos: [ 0.0,  0.7],    color: red   },
            Vertex { pos: [-0.65, -0.55],  color: green },
            Vertex { pos: [ 0.65, -0.55],  color: blue  },
        ];

        #[rustfmt::skip]
        let indices: &[u16] = &[
            // border rect (two triangles)
            0, 1, 2,  0, 2, 3,
            // viewport background (two triangles)
            4, 5, 6,  4, 6, 7,
            // shaded triangle
            8, 9, 10,
        ];

        let vertex_buffer = ctx.new_buffer(
            BufferType::VertexBuffer,
            BufferUsage::Immutable,
            BufferSource::slice(vertices),
        );
        let index_buffer = ctx.new_buffer(
            BufferType::IndexBuffer,
            BufferUsage::Immutable,
            BufferSource::slice(indices),
        );

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
            &[
                VertexAttribute::new("pos",   VertexFormat::Float2),
                VertexAttribute::new("color", VertexFormat::Float4),
            ],
            shader,
            PipelineParams::default(),
        );

        Stage { ctx, pipeline, vertex_buffer, index_buffer }
    }
}

impl EventHandler for Stage {
    fn update(&mut self) {}

    fn draw(&mut self) {
        let bindings = Bindings {
            vertex_buffers: vec![self.vertex_buffer],
            index_buffer: self.index_buffer,
            images: vec![],
        };

        self.ctx.begin_default_pass(PassAction::clear_color(0.13, 0.13, 0.13, 1.0));
        self.ctx.apply_pipeline(&self.pipeline);
        self.ctx.apply_bindings(&bindings);

        // 1. Draw the light grey border rectangle (full-window viewport, default NDC).
        //    Indices 0–5 (two triangles forming the rect).
        self.ctx.draw(0, 6, 1);

        // 2. Restrict rendering to the viewport region.
        //    OpenGL y is measured from the bottom: bottom-left y = WIN_H - VP_Y - VP_H.
        let gl_vp_y = WIN_H as i32 - VP_Y - VP_H;
        self.ctx.apply_viewport(VP_X, gl_vp_y, VP_W, VP_H);

        // 3. Fill the viewport with a dark background (indices 6–11).
        self.ctx.draw(6, 6, 1);

        // 4. Draw the shaded triangle on top (indices 12–14).
        self.ctx.draw(12, 3, 1);

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
attribute vec4 color;

varying vec4 v_color;

void main() {
    gl_Position = vec4(pos, 0.0, 1.0);
    v_color = color;
}
"#;

    pub const FRAGMENT: &str = r#"#version 100
precision mediump float;

varying vec4 v_color;

void main() {
    gl_FragColor = v_color;
}
"#;

    pub const METAL: &str = r#"
    #include <metal_stdlib>

    using namespace metal;

    struct Vertex
    {
        float2 pos   [[attribute(0)]];
        float4 color [[attribute(1)]];
    };

    struct RasterizerData
    {
        float4 position [[position]];
        float4 color    [[user(locn0)]];
    };

    vertex RasterizerData vertexShader(Vertex v [[stage_in]])
    {
        RasterizerData out;
        out.position = float4(v.pos, 0.0, 1.0);
        out.color    = v.color;
        return out;
    }

    fragment float4 fragmentShader(RasterizerData in [[stage_in]])
    {
        return in.color;
    }"#;

    pub fn meta() -> ShaderMeta {
        ShaderMeta {
            images: vec![],
            uniforms: UniformBlockLayout { uniforms: vec![] },
        }
    }
}
