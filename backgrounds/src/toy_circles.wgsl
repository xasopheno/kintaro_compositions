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

fn smoothstep(edge0: f32, edge1: f32, x: f32) -> f32 {
  var t = clamp((x - edge0) / (edge1 - edge0), 0.0, 1.0);
  return t * t * (3.0 - 2.0 * t);
}

[[stage(fragment)]]
fn fs_main(
    [[builtin(position)]] frag_coord: vec4<f32>
) -> [[location(0)]] vec4<f32> {
    var resolution = vec2<f32>(u.width, u.height);
    var uv = frag_coord.xy / resolution * 2.0 - vec2<f32>(1.0, 1.0);
    uv.y = uv.y * resolution.y / resolution.x;
    var t = u.time * 0.5;

    var sphere_center = vec3<f32>(0.0, 0.0, 0.2);
    var sphere_radius = 0.5;
    var sphere_pos = vec3<f32>(uv, -0.2);
    var sphere_dir = normalize(sphere_pos - sphere_center);
    var sphere_dist = length(sphere_pos - sphere_center) - sphere_radius;

    var flag_dir = vec2<f32>(0.0, 1.0);
    var flag_pos = vec2<f32>(sphere_dir.x * 0.3, sphere_dir.y);
    var flag_dist = length(flag_pos) - 0.3;

    var stripe_width = 0.05;
    var stripe_count = 8.0;
    var stripe_offset = t * 3.0;
    var stripe_dist = sin(sphere_dist * t * 10.0);
    var stripe_id = floor((flag_pos.y + stripe_offset) * stripe_count / stripe_width);

    var flag_alpha = smoothstep(-0.1, 0.1, flag_dist);
    var stripe_alpha = 1.0 - smoothstep(0.0, 0.01, abs(stripe_dist));

    var stripe_color = vec3<f32>(1.0);

    var color = vec3<f32>(0.0);
    color = mix(color, stripe_color, stripe_alpha * flag_alpha);
   
   return vec4<f32>(color, 1.0);
 }
