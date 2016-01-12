    pub const VERTEX: &'static str = r#"
        #version 150

        in uint face;
        in uvec3 pos;
        in vec3 corner;

        void main() {
            gl_Position = vec4(corner + pos, 1.0);
        }
    "#;
    pub const GEOMETRY: &'static str = r#"
        #version 150

        layout(lines) in;
        layout(triangle_strip, max_vertices = 4) out;

        out vec2 g_texcoord;

        uniform ivec3 chunk;
        uniform mat4 vp;

        void main() {
            // Two input vertices will be the first and last vertex of the quad
            vec4 a = gl_in[0].gl_Position;
            vec4 d = gl_in[1].gl_Position;

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
        #version 150

        in vec2 g_texcoord;
        out vec4 color;

        uniform sampler2D tex;

        void main() {
            color = texture(tex, g_texcoord);
        }
    "#;
