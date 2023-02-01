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
    var time = u.time + 100.0;

    var r = sin(uv.x * 200.0 * u.time * 0.02) * sin(uv.y * 200.0 * u.time * 0.02);
    var g = sin(uv.x * 300.0 * u.time * 0.03) * sin(uv.y * 300.0 * u.time * 0.03);
    var b = sin(uv.x * 400.0 * u.time * 0.04) * sin(uv.y * 400.0 * u.time * 0.04);

    return vec4<f32>(r, g, b, 1.0);
}
