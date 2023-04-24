use crate::{
    algebra::{Complex, Zero},
    expression::{Expr, ExprComplex, FieldFunction, FieldOperator},
    log,
};
use wasm_bindgen::prelude::*;
use web_sys::{WebGl2RenderingContext as WebGl2, WebGlProgram, WebGlShader, WebGlUniformLocation};


// crazy one: z*z*z-1*i-0.21
// good one: (z*z+1)/(z*z-1)+z
pub enum State {
    Loading,
    Invalid,
    Valid,
}

#[wasm_bindgen]
pub struct ComplexPlane {
    ctx: WebGl2,

    state: State,

    center: Complex<f32>,
    niter: i32,
    xscale: f32,
    function: ExprComplex,

    uniform_center: Option<WebGlUniformLocation>,
    uniform_niter: Option<WebGlUniformLocation>,
    uniform_xscale: Option<WebGlUniformLocation>,
    uniform_yscale: Option<WebGlUniformLocation>,

    last_dragged: Option<(i32, i32)>,
}

#[wasm_bindgen]
impl ComplexPlane {
    #[wasm_bindgen(constructor)]
    pub fn new(ctx: WebGl2) -> Result<ComplexPlane, JsValue> {
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

            function: ExprComplex::default(),

            xscale: 1.0,
            niter: 1,
            center: Complex::O,

            uniform_center: None,
            uniform_niter: None,
            uniform_xscale: None,
            uniform_yscale: None,

            last_dragged: None,
        };

        res.load_function()?;

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
    pub fn set_niter(&mut self, niter: i32) -> Result<(), JsValue> {
        self.niter = niter;
        log::info!("Received niter {}", niter);
        self.state = State::Invalid;
        Ok(())
    }

    #[wasm_bindgen]
    pub fn set_resolution(&mut self, width_px: i32, height_px: i32) {
        self.ctx.viewport(0, 0, width_px, height_px);
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
    pub fn on_pointer_up(&mut self, _x: i32, _y: i32) {
        self.last_dragged = None;
        self.state = State::Invalid;
    }

    fn get_complex_at(&self, x: i32, y: i32) -> Complex<f32> {
        let (x, y) = (x as f32, y as f32);
        let buffer_width = self.ctx.drawing_buffer_width() as f32;
        let buffer_height = self.ctx.drawing_buffer_height() as f32;

        Complex {
            re: 2.0 * (x / buffer_width - 0.5) * self.xscale - self.center.re,
            im: -2.0 * (y / buffer_height - 0.5) * self.yscale() - self.center.im,
        }
    }



    #[wasm_bindgen]
    pub fn display_image_at(&self, x: i32, y: i32) -> String {
        let z = self.get_complex_at(x, y);
        self.function.eval(&z).to_string()
    }

    #[wasm_bindgen]
    pub fn display_value_at(&self, x: i32, y: i32) -> String {
        self.get_complex_at(x, y).to_string()
    }


    #[wasm_bindgen]
    pub fn draw(&mut self) {
        if let State::Loading | State::Valid = self.state {
            return;
        }
        self.ctx.clear_color(0.0, 0.0, 0.0, 1.0);
        self.ctx.clear(WebGl2::COLOR_BUFFER_BIT);
        self.ctx.uniform2fv_with_f32_array(
            self.uniform_center.as_ref(),
            &[self.center.re, self.center.im],
        );
        self.ctx.uniform1i(self.uniform_niter.as_ref(), self.niter);
        self.ctx
            .uniform1f(self.uniform_xscale.as_ref(), self.xscale);
        self.ctx
            .uniform1f(self.uniform_yscale.as_ref(), self.yscale());
        self.ctx.draw_arrays(WebGl2::TRIANGLE_FAN, 0, 4);
        self.state = State::Valid;
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
            include_str!("shader/plane.vert"),
        )?;

        // generate and compile fragment shader
        let mut fragment_src = include_str!("shader/plane.frag").to_string();
        let (begin_mark, end_mark) = ("/*BEGIN REPLACE*/", "/*END REPLACE*/");
        if let (Some(l), Some(r)) = (fragment_src.find(begin_mark), fragment_src.find(end_mark)) {
            let mut snippet = String::new();
            build_snippet(&mut snippet, &self.function);
            fragment_src.replace_range(l..r + end_mark.len(), &snippet);
        }
        let frag_shader = compile_shader(&self.ctx, WebGl2::FRAGMENT_SHADER, &fragment_src)?;
        log::info!("Using fragment shader\n: {fragment_src} ");

        // link and bind program
        let program = link_program(&self.ctx, &vert_shader, &frag_shader)?;
        self.ctx.use_program(Some(&program));

        // bind "position" attribute
        let pos = self.ctx.get_attrib_location(&program, "position") as u32;
        self.ctx
            .vertex_attrib_pointer_with_i32(pos, 2, WebGl2::FLOAT, false, 0, 0);
        self.ctx.enable_vertex_attrib_array(pos);

        // retrieve uniform locations
        self.uniform_center = self.ctx.get_uniform_location(&program, "center");
        self.uniform_xscale = self.ctx.get_uniform_location(&program, "xscale");
        self.uniform_yscale = self.ctx.get_uniform_location(&program, "yscale");
        self.uniform_niter = self.ctx.get_uniform_location(&program, "niter");

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
        Expr::Variable => ret.push_str("z"),
        Expr::Constant(ct) => {
            let Complex { re, im } = ct;
            ret.push_str(&format!("vec2({re},{im})"));
        }
        Expr::Function(fun, e) => {
            let fun_str = match fun {
                FieldFunction::Neg => "-",
                FieldFunction::Inv => "1.0/",
            };
            ret.push_str(fun_str);
            build_snippet(ret, e);
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
