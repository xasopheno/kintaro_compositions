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

    var r = 0.0;
    var g = 0.0;
    var b = 0.0;
    if (
      true
      // uv.y * 1000.0 % 25.0 < 0.5
      // || uv.x * 1000.0 % 25.0 < 0.5
    ) {
        r = 0.1 / uv.y * 1.0/uv.x;
        b = 3.0 / uv.x * 20.0/uv.y - 50.0;
        g = (3.0 / uv.x * 20.0/uv.y - 40.0) * 0.01;
    } else {

    };
    if (uv.y * 100.0 % 10.0 < 1.0) {
    } else {
    };

    return vec4<f32>(
        r,
        g,
        b,
        // sin(((uv.y) / (uv.x)) * (time)) * 0.3,  
        // sin(((uv.y) / (uv.x)) * (time)) * 0.3,
        1.0
    );
//    return vec4<f32>(sin(u.time) * 0.01, 0.01, sin(u.time) * 0.03, 1.0);
   // var color = vec4<f32>(sin(uv.y * 100.0 * u.time * 0.04) * (sin(uv.x * 200.0 * sin(u.time) * 0.01)) * (sin(uv.x *
   // 500.0) + uv.y) * uv.y,
   // sin(uv.x * 300.0 * u.time * 0.02) * uv.y - uv.x, 
  // 0.0,
  // sin(u.time * 0.01) * uv.x - uv.y, 1.0) * 0.01;
  // return color;
   // * vec4<f32>(atan(u.time * 50.0 * (1.0 - uv.x - uv.y / uv.x)) * 0.1, atan(u.time * 100.0 * (uv.y - uv.y / uv.x))
   // * 0.01, atan(u.time * 300.0 * (1.0 - uv.x - uv.y / uv.x)) * 0.03, 1.0)
   // * vec4<f32>(20.0, 0.1, 0.1, 1.0); 
 //  if (uv.y * 1000.0 % 25.0 < 1.0) {
 //      return vec4<f32>(sin(0.3 * u.time), tan(u.time * 0.3), tan(0.2), 1.0);
 //  } else {
 //      return vec4<f32>(sin(abs(u.time * 0.5 - uv.x) * 0.01), 0.0, sin(abs(uv.x - 0.5) * u.time) * 0.02, 0.0); 
 //    // return vec4<f32>(0.002, 0.002, 0.003, 1.0);
 // }
}


