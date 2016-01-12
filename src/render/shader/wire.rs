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
