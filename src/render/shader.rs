pub const CUBE_VERTEX: &'static str = r#"
    #version 140

    in vec3 pos;
    in vec2 tex_pos;
    in ivec3 cube_pos;

    out vec2 v_tex_pos;

    uniform mat4 vp;

    void main() {
        v_tex_pos = tex_pos;
        gl_Position = vp * vec4(pos + cube_pos, 1.0);
    }
"#;
pub const CUBE_FRAGMENT: &'static str = r#"
    #version 140

    in vec2 v_tex_pos;
    out vec4 color;

    uniform sampler2D tex;

    void main() {
        color = texture(tex, v_tex_pos);
    }
"#;

pub const WIRE_VERTEX: &'static str = r#"
    #version 140

    in vec3 pos;
    in vec3 color;
    in ivec3 cube_pos;

    out vec3 v_color;

    uniform mat4 vp;

    void main() {
        v_color = color;
        gl_Position = vp * vec4(pos + cube_pos, 1.0);
    }
"#;
pub const WIRE_FRAGMENT: &'static str = r#"
    #version 140

    in vec3 v_color;
    out vec4 color;

    void main() {
        color = vec4(v_color, 1.0);
    }
"#;

pub const PICK_VERTEX: &'static str = r#"
    #version 140

    in vec3 pos;
    in vec3 color;
    in ivec3 cube_pos;

    flat out ivec3 v_id;

    uniform mat4 vp;

    void main() {
        v_id = cube_pos;
        gl_Position = vp * vec4(pos + cube_pos, 1.0);
    }
"#;
pub const PICK_FRAGMENT: &'static str = r#"
    #version 140

    flat in ivec3 v_id;
    out ivec3 f_id;

    void main() {
        f_id = v_id;
    }
"#;
