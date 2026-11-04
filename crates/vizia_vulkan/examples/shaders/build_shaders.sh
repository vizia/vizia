#!/bin/bash
# # Minimal vertex shader - fullscreen triangle with tex coords
# cat > vert.glsl << 'GLSL'
# #version 450
# vec2 positions[3] = vec2[](
#     vec2(-1.0, -1.0),
#     vec2(3.0, -1.0),
#     vec2(-1.0, 3.0)
# );
# vec2 tex_coords[3] = vec2[](
#     vec2(0.0, 1.0),
#     vec2(2.0, 1.0),
#     vec2(0.0, -1.0)
# );
# layout(location = 0) out vec2 frag_tex_coord;
# void main() {
#     gl_Position = vec4(positions[gl_VertexIndex], 0.0, 1.0);
#     frag_tex_coord = tex_coords[gl_VertexIndex];
# }
# GLSL

# # Fragment shader - sample texture
# cat > frag.glsl << 'GLSL'
# #version 450
# layout(location = 0) in vec2 frag_tex_coord;
# layout(location = 0) out vec4 out_color;
# layout(binding = 0) uniform sampler2D ui_texture;
# void main() {
#     out_color = texture(ui_texture, frag_tex_coord);
#     out_color = vec4(1.0);
# }
# GLSL

cd -- "$(dirname "${BASH_SOURCE[0]}")" && pwd

# Compile with glslc if available, otherwise use pre-built
if command -v glslc &> /dev/null; then
    glslc -fshader-stage=vert vert.glsl -o vert.spv
    glslc -fshader-stage=frag frag.glsl -o frag.spv
    echo "Compiled shaders with glslc"
else
    echo "glslc not found, will use pre-built shaders"
fi
