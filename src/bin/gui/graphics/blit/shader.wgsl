[[builtin(vertex_index)]] var<in>  vert_idx : u32;
[[builtin(position)]]     var<out> pos      : vec4<f32>;

[[stage(vertex)]]
fn vs_main() {
  if (vert_idx == 0) { pos = vec4<f32>(-1.0, -1.0, 0.0, 1.0); }
  if (vert_idx == 1) { pos = vec4<f32>( 1.0, -1.0, 0.0, 1.0); }
  if (vert_idx == 2) { pos = vec4<f32>(-1.0,  1.0, 0.0, 1.0); }
  if (vert_idx == 3) { pos = vec4<f32>( 1.0,  1.0, 0.0, 1.0); }
}

[[builtin(frag_coord)]] var<in> coord : vec4<f32>;

[[location(0)]] var<out> out_color : vec4<f32>;

type Pixel = vec4<f32>;
type PixelArr = [[stride(16)]] array<Pixel>;
[[block]] struct Blit { pixels : PixelArr; };
[[group(0), binding(0)]] var<storage> blit : [[access(read)]] Blit;

[[block]] struct Frame { width : u32; };
[[group(0), binding(1)]] var<uniform> frame : Frame;

fn srgb(channel: f32) -> f32 {
  if (channel <= 0.0031308) { return 12.92 * channel; }
  else { return 1.055 * pow(channel, 0.416667) - 0.055; }
}

[[stage(fragment)]]
fn fs_main() {
  var pixel : Pixel = blit.pixels[u32(coord.y) * frame.width + u32(coord.x)];
  out_color = vec4<f32>(srgb(pixel.x / pixel.w), srgb(pixel.y / pixel.w),
                        srgb(pixel.z / pixel.w), 1.0);
}
