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
