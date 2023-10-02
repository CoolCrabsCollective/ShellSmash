// This shader computes the chromatic aberration effect

#import bevy_pbr::utils

// Since post processing is a fullscreen effect, we use the fullscreen vertex shader provided by bevy.
// This will import a vertex shader that renders a single fullscreen triangle.
//
// A fullscreen triangle is a single triangle that covers the entire screen.
// The box in the top left in that diagram is the screen. The 4 x are the corner of the screen
//
// Y axis
//  1 |  x-----x......
//  0 |  |  s  |  . ´
// -1 |  x_____x´
// -2 |  :  .´
// -3 |  :´
//    +---------------  X axis
//      -1  0  1  2  3
//
// As you can see, the triangle ends up bigger than the screen.
//
// You don't need to worry about this too much since bevy will compute the correct UVs for you.
#import bevy_core_pipeline::fullscreen_vertex_shader FullscreenVertexOutput

@group(0) @binding(0) var screen_texture: texture_2d<f32>;
@group(0) @binding(1) var texture_sampler: sampler;
struct PostProcessSettings {
    time: f32,
    enable_effect: f32,
    padding: vec2<f32>,
}
@group(0) @binding(2) var<uniform> settings: PostProcessSettings;

@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {

    if settings.enable_effect == 0.0 {
      return vec4<f32>(
          textureSample(screen_texture, texture_sampler, in.uv).rgb,
          1.0
      );
    }
    // Chromatic aberration strength
    // let offset_strength = settings.intensity;

    let value_1 = 12.5;
    let value_2 = 0.01;

    // https://www.shadertoy.com/view/4tG3WR
    let x = in.uv.x * value_1 + settings.time;
    let y = in.uv.y * value_1 + settings.time;
    let water_uv = vec2(
      in.uv.x + cos(x-y) * value_2 * sin(y),
      in.uv.y + cos(x+y) * value_2 * cos(y)
    );

    let distance_from_center = clamp(length(2.0 * in.uv - 1.0) - 0.2, 0.0, 1.0);

    let uv = mix(in.uv, water_uv, distance_from_center);
    
    let screen_texture_color = textureSample(screen_texture, texture_sampler, uv);

    
    // float X = uv.x*25.+iTime;
    //     float Y = uv.y*25.+iTime;
    //     uv.y += cos(X+Y)*0.01*cos(Y);
    //     uv.x += sin(X-Y)*0.01*sin(Y);

    // Sample each color channel with an arbitrary shift
    return vec4<f32>(
        mix(screen_texture_color.rgb, vec3(0.0, 0.8, 1.0), 0.1),
        1.0
    );
}