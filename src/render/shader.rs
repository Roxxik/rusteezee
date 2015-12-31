pub const cube_shader_vertex_src: &'static str = r#"
    #version 140

    in vec3 pos;
    in vec2 tex_pos;
    in ivec3 cube_pos;

    out vec2 v_tex_pos;

    uniform mat4 perspective;
    uniform mat4 view;
    uniform mat4 model;

    void main() {
        v_tex_pos = tex_pos;
        gl_Position = perspective * view * model * vec4(pos + cube_pos, 1.0);
    }
"#;
pub const cube_shader_fragment_src: &'static str = r#"
    #version 140

    in vec2 v_tex_pos;
    out vec4 color;

    uniform sampler2D tex;

    void main() {
        color = texture(tex, v_tex_pos);
    }
"#;

pub const wire_shader_vertex_src: &'static str = r#"
    #version 140

    in vec3 pos;
    in vec3 color;
    in ivec3 cube_pos;

    out vec3 v_color;

    uniform mat4 perspective;
    uniform mat4 view;
    uniform mat4 model;

    void main() {
        v_color = color;
        gl_Position = perspective * view * model * vec4(pos + cube_pos, 1.0);
    }
"#;
pub const wire_shader_fragment_src: &'static str = r#"
    #version 140

    in vec3 v_color;
    out vec4 color;

    void main() {
        color = vec4(v_color, 1.0);
    }
"#;
