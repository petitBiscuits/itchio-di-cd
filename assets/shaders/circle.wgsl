@group(0) @binding(0) var<uniform> color: vec4<f32>;

@fragment
fn fragment(@builtin(position) frag_pos: vec4<f32>) -> @location(0) vec4<f32> {
    let uv = frag_pos.xy / vec2<f32>(640.0, 480.0); // Normalize to screen size
    let center = vec2<f32>(0.0, 0.0);
    let dist = distance(uv, center);

    if dist < 1 {
        return color; // Inside circle
    }
    return vec4<f32>(0.0, 0.0, 0.0, 1.0); // Outside circle
}