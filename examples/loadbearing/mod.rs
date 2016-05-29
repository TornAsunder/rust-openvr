extern crate glium;

use glium::backend::Facade;

pub fn foo<S: Facade>(display: &S) 
{
    let vertex_shader_src = r#"
            #version 140

            in vec3 position;

            uniform mat4 matrix;

            void main() {
                gl_Position = matrix * vec4(position, 1.0);
            }
        "#;

    let fragment_shader_src = r#"
            #version 140

            out vec4 color;

            void main() {
                color = vec4(0.9,1,0.9,1);
            }
        "#;

    let program = glium::Program::from_source(display, vertex_shader_src, fragment_shader_src, None).unwrap();
}
