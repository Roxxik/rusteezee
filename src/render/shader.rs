pub mod cube {
    pub const VERTEX: &'static str = r#"
        #version 140

        in vec3 corner;

        void main() {
            gl_Position = vec4(corner, 1.0);
        }
    "#;
    pub const GEOMETRY: &'static str = r#"
        #version 450

        #extension GL_EXT_geometry_shader : enable

        layout(lines) in;
        layout(triangle_strip, max_vertices = 4) out;

        out vec2 g_texcoord;

        uniform ivec3 chunk;
        uniform mat4 vp;

        void main() {
            // Two input vertices will be the first and last vertex of the quad
            vec4 a = gl_PositionIn[0];
            vec4 d = gl_PositionIn[1];

            // Calculate the middle two vertices of the quad
            vec4 b = a;
            vec4 c = a;

            if(a.y == d.y) { // y same
                c.z = d.z;
                b.x = d.x;
            } else { // x or z same
                b.xz = d.xz;
                c.y = d.y;
            }

            // Emit the vertices of the quad
            g_texcoord = vec2(0.0, 0.0); gl_Position = vp * (a + ivec4(chunk, 0) * 16); EmitVertex();
            g_texcoord = vec2(1.0, 0.0); gl_Position = vp * (b + ivec4(chunk, 0) * 16); EmitVertex();
            g_texcoord = vec2(0.0, 1.0); gl_Position = vp * (c + ivec4(chunk, 0) * 16); EmitVertex();
            g_texcoord = vec2(1.0, 1.0); gl_Position = vp * (d + ivec4(chunk, 0) * 16); EmitVertex();
            EndPrimitive();
        }
    "#;
    pub const FRAGMENT: &'static str = r#"
        #version 140

        in vec2 g_texcoord;
        out vec4 color;

        uniform sampler2D tex;

        void main() {
            color = texture(tex, g_texcoord);
        }
    "#;
}

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

    uniform ivec3 chunk;
    uniform mat4 vp;

    void main() {
        v_id = cube_pos;
        gl_Position = vp * vec4(pos + cube_pos + (chunk * 16), 1.0);
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
