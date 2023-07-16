use crate::{
    algebra::{Complex, Zero},
    expression::{ComplexFunction, Expr, ExprComplex, FieldOperator, Variable},
};
use wasm_bindgen::prelude::*;
use web_sys::{WebGl2RenderingContext as WebGl2, WebGlProgram, WebGlShader, WebGlUniformLocation};

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub enum DrawMode {
    DomainColouring = 1,
    ParameterStability = 2,
    Julia = 3,
}

#[wasm_bindgen]
pub struct JsComplex {
    pub re: f32,
    pub im: f32,
}

enum State {
    Loading,
    Invalid,
    Valid,
}

const SUBSAMPLE_ROOT: i32 = 2; // subsample 4 points per pixel

#[wasm_bindgen]
pub struct Plotter {
    ctx: WebGl2,

    state: State,
    function: ExprComplex,

    draw_mode: DrawMode,
    max_iter: i32,
    xscale: f32,
    center: Complex<f32>,
    parameter_c: Complex<f32>,

    u_draw_mode: Option<WebGlUniformLocation>,
    u_max_iter: Option<WebGlUniformLocation>,
    u_resolution: Option<WebGlUniformLocation>,
    u_scale: Option<WebGlUniformLocation>,
    u_center: Option<WebGlUniformLocation>,
    u_parameter_c: Option<WebGlUniformLocation>,
    u_subsample: Option<WebGlUniformLocation>,

    last_dragged: Option<(i32, i32)>,
}

#[wasm_bindgen]
impl Plotter {
    #[wasm_bindgen(constructor)]
    pub fn new(
        ctx: WebGl2,
        function: &str,
        draw_mode: DrawMode,
        max_iter: i32,
    ) -> Result<Plotter, JsValue> {
        let buffer = ctx.create_buffer().ok_or("Failed to create buffer")?;
        ctx.bind_buffer(WebGl2::ARRAY_BUFFER, Some(&buffer));

        let positions_array_buf_view =
            // just a square
            unsafe { js_sys::Float32Array::view(&[-1.0, -1.0, 1.0, -1.0, 1.0, 1.0, -1.0, 1.0]) };

        ctx.buffer_data_with_array_buffer_view(
            WebGl2::ARRAY_BUFFER,
            &positions_array_buf_view,
            WebGl2::STATIC_DRAW,
        );
        let vao = ctx
            .create_vertex_array()
            .ok_or("Could not create vertex array object")?;
        ctx.bind_vertex_array(Some(&vao));

        let mut res = Self {
            ctx,

            state: State::Invalid,
            function: ExprComplex::Constant(Complex::O),
            draw_mode,
            max_iter,
            xscale: 1.0,
            center: Complex::O,
            parameter_c: Complex::O,

            u_draw_mode: None,
            u_max_iter: None,
            u_resolution: None,
            u_scale: None,
            u_center: None,
            last_dragged: None,
            u_parameter_c: None,
            u_subsample: None,
        };
        res.set_function(function)?;
        Ok(res)
    }

    #[wasm_bindgen]
    pub fn set_function(&mut self, function: &str) -> Result<(), JsValue> {
        let new_function = function.parse::<ExprComplex>()?;
        if self.function != new_function {
            self.function = new_function;
            self.load_function()?;
        }
        Ok(())
    }

    #[wasm_bindgen]
    pub fn set_draw_mode(&mut self, draw_mode: DrawMode) {
        self.draw_mode = draw_mode;
        self.state = State::Invalid;
    }

    #[wasm_bindgen]
    pub fn set_max_iter(&mut self, max_iter: i32) {
        self.max_iter = max_iter;
        self.state = State::Invalid;
    }

    #[wasm_bindgen]
    pub fn set_resolution(&mut self, client_width: i32, client_height: i32) {
        self.ctx.viewport(0, 0, client_width, client_height);
        self.state = State::Invalid;
    }

    #[wasm_bindgen]
    pub fn set_parameter_c(&mut self, parameter_c: JsComplex) {
        self.parameter_c = Complex {
            re: parameter_c.re,
            im: parameter_c.im,
        };
        self.state = State::Invalid;
    }

    #[wasm_bindgen]
    pub fn zoom(&mut self, factor: f32) {
        self.xscale /= factor;
        self.state = State::Invalid;
    }

    #[wasm_bindgen]
    pub fn reset(&mut self) {
        self.xscale = 1.0;
        self.center = Complex::O;
        self.state = State::Invalid;
    }

    #[wasm_bindgen]
    pub fn on_pointer_down(&mut self, x: i32, y: i32) {
        self.last_dragged = Some((x, y));
        self.state = State::Invalid;
    }

    #[wasm_bindgen]
    pub fn on_pointer_move(&mut self, current_x: i32, current_y: i32) {
        if let Some((last_x, last_y)) = self.last_dragged {
            let buffer_width = self.ctx.drawing_buffer_width() as f32;
            let buffer_height = self.ctx.drawing_buffer_height() as f32;
            self.center.re += 2.0 * (current_x - last_x) as f32 / buffer_width * self.xscale;
            self.center.im -= 2.0 * (current_y - last_y) as f32 / buffer_height * self.yscale();

            self.last_dragged = Some((current_x, current_y));
            self.state = State::Invalid;
        }
    }

    #[wasm_bindgen]
    pub fn on_pointer_up(&mut self) {
        self.last_dragged = None;
        self.state = State::Invalid;
    }

    #[wasm_bindgen]
    pub fn draw(&mut self) {
        if let State::Loading | State::Valid = self.state {
            return;
        }
        // load uniforms
        self.ctx
            .uniform1i(self.u_draw_mode.as_ref(), self.draw_mode as i32);
        self.ctx.uniform1i(self.u_max_iter.as_ref(), self.max_iter);
        self.ctx.uniform2fv_with_f32_array(
            self.u_resolution.as_ref(),
            &[
                self.ctx.drawing_buffer_width() as f32,
                self.ctx.drawing_buffer_height() as f32,
            ],
        );
        self.ctx
            .uniform2fv_with_f32_array(self.u_center.as_ref(), &[self.center.re, self.center.im]);
        self.ctx
            .uniform2fv_with_f32_array(self.u_scale.as_ref(), &[self.xscale, self.yscale()]);
        self.ctx.uniform2fv_with_f32_array(
            self.u_parameter_c.as_ref(),
            &[self.parameter_c.re, self.parameter_c.im],
        );
        self.ctx
            .uniform1i(self.u_subsample.as_ref(), SUBSAMPLE_ROOT);
        self.ctx.clear_color(0.0, 0.0, 0.0, 1.0);
        self.ctx.clear(WebGl2::COLOR_BUFFER_BIT);

        self.ctx.draw_arrays(WebGl2::TRIANGLE_FAN, 0, 4);
        self.state = State::Valid;
    }

    #[wasm_bindgen]
    pub fn get_complex_at(&self, x: i32, y: i32) -> JsComplex {
        let (x, y) = (x as f32, y as f32);
        let buffer_width = self.ctx.drawing_buffer_width() as f32;
        let buffer_height = self.ctx.drawing_buffer_height() as f32;

        JsComplex {
            re: 2.0 * (x / buffer_width - 0.5) * self.xscale - self.center.re,
            im: -2.0 * (y / buffer_height - 0.5) * self.yscale() - self.center.im,
        }
    }

    fn yscale(&self) -> f32 {
        self.ctx.drawing_buffer_height() as f32 / self.ctx.drawing_buffer_width() as f32
            * self.xscale
    }

    fn load_function(&mut self) -> Result<(), JsValue> {
        self.state = State::Loading;
        // compile vertex shader
        let vert_shader = compile_shader(
            &self.ctx,
            WebGl2::VERTEX_SHADER,
            include_str!("shader/plotter.vert"),
        )?;

        // generate and compile fragment shader
        let mut fragment_src = include_str!("shader/plotter.frag").to_string();
        let (begin_mark, end_mark) = ("/*BEGIN REPLACE*/", "/*END REPLACE*/");
        if let (Some(l), Some(r)) = (fragment_src.find(begin_mark), fragment_src.find(end_mark)) {
            let mut snippet = String::new();
            build_snippet(&mut snippet, &self.function);
            fragment_src.replace_range(l..r + end_mark.len(), &snippet);
        }
        let frag_shader = compile_shader(&self.ctx, WebGl2::FRAGMENT_SHADER, &fragment_src)?;
        //log::info!("Using fragment shader\n: {fragment_src} ");

        // link and bind program
        let program = link_program(&self.ctx, &vert_shader, &frag_shader)?;
        self.ctx.use_program(Some(&program));

        // bind "position" attribute
        let pos = self.ctx.get_attrib_location(&program, "position") as u32;
        self.ctx
            .vertex_attrib_pointer_with_i32(pos, 2, WebGl2::FLOAT, false, 0, 0);
        self.ctx.enable_vertex_attrib_array(pos);

        // retrieve uniform locations
        self.u_center = self.ctx.get_uniform_location(&program, "center");
        self.u_scale = self.ctx.get_uniform_location(&program, "scale");
        self.u_max_iter = self.ctx.get_uniform_location(&program, "max_iter");
        self.u_draw_mode = self.ctx.get_uniform_location(&program, "draw_mode");
        self.u_resolution = self.ctx.get_uniform_location(&program, "resolution");
        self.u_parameter_c = self.ctx.get_uniform_location(&program, "parameter_c");
        self.u_subsample = self.ctx.get_uniform_location(&program, "n_subsample");
        // invalidate plane
        self.state = State::Invalid;
        Ok(())
    }
}

fn link_program(
    context: &WebGl2,
    vert_shader: &WebGlShader,
    frag_shader: &WebGlShader,
) -> Result<WebGlProgram, String> {
    let program = context
        .create_program()
        .ok_or_else(|| String::from("Unable to create shader object"))?;

    context.attach_shader(&program, vert_shader);
    context.attach_shader(&program, frag_shader);
    context.link_program(&program);

    if context
        .get_program_parameter(&program, WebGl2::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(program)
    } else {
        Err(context
            .get_program_info_log(&program)
            .unwrap_or_else(|| String::from("Unknown error creating program object")))
    }
}

fn compile_shader(context: &WebGl2, shader_type: u32, source: &str) -> Result<WebGlShader, String> {
    let shader = context
        .create_shader(shader_type)
        .ok_or_else(|| String::from("Unable to create shader object"))?;
    context.shader_source(&shader, source);
    context.compile_shader(&shader);

    if context
        .get_shader_parameter(&shader, WebGl2::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        Err(context
            .get_shader_info_log(&shader)
            .unwrap_or_else(|| String::from("Unknown error creating shader")))
    }
}

fn build_snippet(ret: &mut String, expr: &ExprComplex) {
    match expr {
        Expr::Variable(var) => {
            let str = match var {
                Variable::C => "c",
                Variable::Z => "z",
            };
            ret.push_str(str);
        }
        Expr::Constant(ct) => {
            let Complex { re, im } = ct;
            ret.push_str(&format!("vec2({re},{im})"));
        }
        Expr::Function(fun, e) => {
            let fun_str = match fun {
                ComplexFunction::Re => "re",
                ComplexFunction::Im => "im",
                ComplexFunction::Abs => "abs",
            };
            ret.push_str(fun_str);
            ret.push('(');
            build_snippet(ret, e);
            ret.push(')');
        }
        Expr::Operator(op, lhs, rhs) => {
            let op_str = match op {
                FieldOperator::Add => "add",
                FieldOperator::Sub => "sub",
                FieldOperator::Mul => "mul",
                FieldOperator::Div => "div",
            };
            ret.push_str(op_str);
            ret.push('(');
            build_snippet(ret, lhs);
            ret.push(',');
            build_snippet(ret, rhs);
            ret.push(')');
        }
    }
}
