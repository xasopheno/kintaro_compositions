struct Uniforms {
    width: f32;
    height: f32;
    frame: f32;
    time: f32;
};

[[group(0), binding(0)]]
var<uniform> u: Uniforms;

[[stage(vertex)]]
fn vs_main([[builtin(vertex_index)]] vertex_index: u32) -> [[builtin(position)]] vec4<f32> {
    var x = f32(i32((vertex_index << u32(1)) & u32(2)));
    var y = f32(i32(vertex_index & u32(2)));
    var uv = vec2<f32>(x, y);
    var out = 2.0 * uv - vec2<f32>(1.0, 1.0);
    return vec4<f32>(out.x, out.y, 0.0, 1.0);
}

[[stage(fragment)]]
fn fs_main(
    [[builtin(position)]] frag_coord: vec4<f32>
) -> [[location(0)]] vec4<f32> {
    var resolution = vec2<f32>(u.width, u.height);
    var uv = frag_coord.xy / resolution;
    var time = (u.time + 10.0) / 4.0;

var swirl = vec2<f32>(uv.x - 0.5, uv.y - 0.5);
swirl = swirl * length(swirl) * (3.0 + sin(time * 0.05) * 0.15);
uv = uv + swirl;

var z = sin(uv.x * 800.0 * time * 0.05) * sin(uv.y * 20.0 * time * 0.05);
var r = sin(uv.x * 40.0 * time * 0.02) * sin(uv.y * 3.0 * time * 0.02 + z * 0.31);
var g = sin(uv.x * 40.0 * time * 0.0201) * sin(uv.y * 3.0 * time * 0.0201 + z * 0.31);
var b = sin(uv.x * 40.0 * time * 0.0202) * sin(uv.y * 3.0 * time * 0.0202 + z * 0.31);

return vec4<f32>(r, g, b, 1.0);
}
