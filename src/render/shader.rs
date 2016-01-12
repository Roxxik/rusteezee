pub mod cube {
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
}

pub mod wire {
    pub const VERTEX: &'static str = r#"
        #version 150

        in vec3 corner;

        void main() {
            gl_Position = vec4(corner, 1.0);
        }
    "#;
    pub const GEOMETRY: &'static str = r#"
        #version 150

        layout(lines) in;
        layout(line_strip, max_vertices = 16) out;

        uniform mat4 vp;
        uniform ivec3 chunk;
        uniform uvec3 pos;

        void main() {

            vec4 a = gl_in[0].gl_Position;
            vec4 h = gl_in[1].gl_Position;

            vec4 b = a;
            vec4 c = a;
            vec4 d = a;
            vec4 e = a;
            vec4 f = a;
            vec4 g = a;

            // coordinates
            // ^  z
            // |

            // -> x

            b.x  = h.x;
            c.z  = h.z;
            d.xz = h.xz;
            e.y  = h.y;
            f.xy = h.xy;
            g.yz = h.yz;

            ivec4 offset = ivec4(ivec3(pos) + chunk * 16, 0);

            a = vp * (a + offset);
            b = vp * (b + offset);
            c = vp * (c + offset);
            d = vp * (d + offset);
            e = vp * (e + offset);
            f = vp * (f + offset);
            g = vp * (g + offset);
            h = vp * (h + offset);

            //top layer:
            // ab
            // cd

            //bottom layer:
            // ef
            // gh

            gl_Position = a; EmitVertex();
            gl_Position = b; EmitVertex();
            gl_Position = d; EmitVertex();
            gl_Position = c; EmitVertex();
            gl_Position = a; EmitVertex();
            gl_Position = e; EmitVertex();
            gl_Position = f; EmitVertex();
            gl_Position = h; EmitVertex();
            gl_Position = g; EmitVertex();
            gl_Position = e; EmitVertex();
            EndPrimitive();

            gl_Position = b; EmitVertex();
            gl_Position = f; EmitVertex();
            EndPrimitive();

            gl_Position = c; EmitVertex();
            gl_Position = g; EmitVertex();
            EndPrimitive();

            gl_Position = d; EmitVertex();
            gl_Position = h; EmitVertex();
            EndPrimitive();
        }
    "#;
    pub const FRAGMENT: &'static str = r#"
        #version 150

        out vec4 f_color;

        uniform vec4 color;

        void main() {
            f_color = color;
        }
    "#;
}

pub mod picking {
    pub const VERTEX: &'static str = r#"
        #version 150

        in uint face;
        in uvec3 pos;
        in vec3 corner;

        flat out uint v_face;
        flat out uvec3 v_pos;

        void main() {
            v_face = face;
            v_pos = pos;
            gl_Position = vec4(corner + pos, 1.0);
        }
    "#;
    pub const GEOMETRY: &'static str = r#"
        #version 150

        layout(lines) in;
        layout(triangle_strip, max_vertices = 4) out;

        flat in uint v_face[2];
        flat in uvec3 v_pos[2];

        flat out uint g_id;

        uniform ivec3 chunk;
        uniform mat4 vp;

        void main() {
            if (
                   chunk.x >= -1 && chunk.x <= 1
                && chunk.y >= -1 && chunk.y <= 1
                && chunk.z >= -1 && chunk.z <= 1
            ){
                //calculate face id
                // first bit means empty -> 1 bit
                // 6 faces               -> 3 bits
                // 0..15 for pos         -> 4 bits * 3
                // -1..1(0..2) for chunk -> 2 bits * 3
                // total: 22 bits
                uint id =
                      uint(chunk.x) + 1u << 20u
                    | uint(chunk.y) + 1u << 18u
                    | uint(chunk.z) + 1u << 16u
                    | uint(v_pos[0].x)   << 12u
                    | uint(v_pos[0].y)   << 8u
                    | uint(v_pos[0].z)   << 4u
                    | uint(v_face[0])    << 1u
                    | 1u;

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
                g_id = id; gl_Position = vp * (a + ivec4(chunk * 16, 0)); EmitVertex();
                g_id = id; gl_Position = vp * (b + ivec4(chunk * 16, 0)); EmitVertex();
                g_id = id; gl_Position = vp * (c + ivec4(chunk * 16, 0)); EmitVertex();
                g_id = id; gl_Position = vp * (d + ivec4(chunk * 16, 0)); EmitVertex();
                EndPrimitive();
            }
        }
    "#;
    pub const FRAGMENT: &'static str = r#"
        #version 150

        flat in uint g_id;
        out uint f_id;

        void main() {
            f_id = g_id;
        }
    "#;
    pub const FRAGMENT_ALT: &'static str = r#"
            #version 150

            flat in uint g_id;
            out vec4 color;

            void main() {
                color = vec4(g_id, g_id, g_id, 1.0);
            }
    "#;
}
