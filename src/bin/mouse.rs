use miniquad::*;

const WINDOW_WIDTH: i32 = 800;
const WINDOW_HEIGHT: i32 = 600;

#[repr(C)]
struct Vertex {
    pos: [f32; 2],
    uv: [f32; 2],
}

struct Stage {
    ctx: Box<dyn RenderingBackend>,
    pipeline: Pipeline,
    bindings: Bindings,
    pixels: Vec<u8>,
    width: usize,
    height: usize,
    left_mouse_down: bool,
}

impl Stage {
    fn new() -> Stage {
        let mut ctx = window::new_rendering_backend();

        let vertices: [Vertex; 4] = [
            Vertex {
                pos: [-1.0, -1.0],
                uv: [0.0, 1.0],
            },
            Vertex {
                pos: [1.0, -1.0],
                uv: [1.0, 1.0],
            },
            Vertex {
                pos: [1.0, 1.0],
                uv: [1.0, 0.0],
            },
            Vertex {
                pos: [-1.0, 1.0],
                uv: [0.0, 0.0],
            },
        ];

        let indices: [u16; 6] = [0, 1, 2, 0, 2, 3];

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

        let width = WINDOW_WIDTH as usize;
        let height = WINDOW_HEIGHT as usize;
        let pixels = vec![0_u8; width * height * 4];

        let texture = ctx.new_texture_from_rgba8(width as u16, height as u16, &pixels);
        ctx.texture_set_filter(texture, FilterMode::Nearest, MipmapFilterMode::None);

        let bindings = Bindings {
            vertex_buffers: vec![vertex_buffer],
            index_buffer,
            images: vec![texture],
        };

        let shader = ctx
            .new_shader(
                ShaderSource::Glsl {
                    vertex: VERTEX_SHADER,
                    fragment: FRAGMENT_SHADER,
                },
                shader::meta(),
            )
            .unwrap();

        let pipeline = ctx.new_pipeline(
            &[BufferLayout::default()],
            &[
                VertexAttribute::new("pos", VertexFormat::Float2),
                VertexAttribute::new("uv", VertexFormat::Float2),
            ],
            shader,
            PipelineParams::default(),
        );

        Stage {
            ctx,
            pipeline,
            bindings,
            pixels,
            width,
            height,
            left_mouse_down: false,
        }
    }

    fn set_white_pixel(&mut self, x: f32, y: f32) {
        let px = x as i32;
        let py = y as i32;

        if px < 0 || py < 0 {
            return;
        }

        let px = px as usize;
        let py = py as usize;

        if px >= self.width || py >= self.height {
            return;
        }

        let idx = (py * self.width + px) * 4;
        self.pixels[idx] = 255;
        self.pixels[idx + 1] = 255;
        self.pixels[idx + 2] = 255;
        self.pixels[idx + 3] = 255;
    }
}

impl EventHandler for Stage {
    fn update(&mut self) {}

    fn draw(&mut self) {
        self.ctx.texture_update(self.bindings.images[0], &self.pixels);

        self.ctx
            .begin_default_pass(PassAction::clear_color(0.0, 0.0, 0.0, 1.0));
        self.ctx.apply_pipeline(&self.pipeline);
        self.ctx.apply_bindings(&self.bindings);
        self.ctx.draw(0, 6, 1);
        self.ctx.end_render_pass();
        self.ctx.commit_frame();
    }

    fn mouse_button_down_event(&mut self, button: MouseButton, x: f32, y: f32) {
        if button != MouseButton::Left {
            return;
        }

        self.left_mouse_down = true;
        self.set_white_pixel(x, y);
    }

    fn mouse_button_up_event(&mut self, button: MouseButton, _x: f32, _y: f32) {
        if button != MouseButton::Left {
            return;
        }

        self.left_mouse_down = false;
    }

    fn mouse_motion_event(&mut self, x: f32, y: f32) {
        if !self.left_mouse_down {
            return;
        }

        self.set_white_pixel(x, y);
    }
}

fn main() {
    miniquad::start(
        conf::Conf {
            window_title: String::from("Mouse Pixels"),
            window_width: WINDOW_WIDTH,
            window_height: WINDOW_HEIGHT,
            high_dpi: false,
            ..Default::default()
        },
        || Box::new(Stage::new()),
    );
}

const VERTEX_SHADER: &str = r#"#version 100
attribute vec2 pos;
attribute vec2 uv;

varying lowp vec2 texcoord;

void main() {
    gl_Position = vec4(pos, 0.0, 1.0);
    texcoord = uv;
}
"#;

const FRAGMENT_SHADER: &str = r#"#version 100
varying lowp vec2 texcoord;
uniform sampler2D tex;

void main() {
    gl_FragColor = texture2D(tex, texcoord);
}
"#;

mod shader {
    use miniquad::{ShaderMeta, UniformBlockLayout};

    pub fn meta() -> ShaderMeta {
        ShaderMeta {
            images: vec!["tex".to_string()],
            uniforms: UniformBlockLayout {
                uniforms: vec![],
            },
        }
    }
}
