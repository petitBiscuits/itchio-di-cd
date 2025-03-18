struct Uniforms {
    time: f32,
    zoom: f32,
    offsetx: f32,
    offsety: f32,
};

@group(2) @binding(0)
var<uniform> uniforms: Uniforms;

@fragment
fn fragment(@location(0) frag_uv: vec4<f32>) -> @location(0) vec4<f32> {
    // On place l’origine au centre de l’écran (0.5, 0.5).
    let dx = (frag_uv.x  - 0.5);
    let dy = (frag_uv.y  - 0.5);

    // On applique la "taille" initiale (0.003) puis le zoom.
    let scaled_x = dx * 0.003 * uniforms.zoom;
    let scaled_y = dy * 0.003 * uniforms.zoom;

    // On ajoute l'offset (qui représente la coordonnée fractale que l'on veut "voir au centre").
    let x0 = uniforms.offsetx + scaled_x;
    let y0 = uniforms.offsety + scaled_y;

    var z = vec2<f32>(0.0, 0.0);
    var iter = 0u;
    let max_iter = 1000u;

    let exponent = 2.0 + uniforms.time / 2;

    // Mandelbrot iteration
    loop {
        if (iter >= max_iter || dot(z, z) > 4.0) {
            break;
        }
//        let xtemp = pow(z.x, 2.) - pow(z.y, 2.) + x0 ;
//        z.y = 2*z.x*z.y + y0;
//        z.x = xtemp;
//        iter += 1u;

        // Convertit z en polaire
        let r     = sqrt(dot(z, z));          // norme
        let theta = atan2(z.y, z.x);          // argument

        let rn = pow(r, exponent);
        let nx = rn * cos(exponent * theta);
        let ny = rn * sin(exponent * theta);

        z = vec2<f32>(nx + x0, ny + y0);

        iter += 1u;
    }

    // Map iteration count to a color (simple grayscale example)
    if (iter == max_iter) {
        return vec4<f32>(0.0, 0.0, 0.0, 1.0);
    } else {
        var color = palette_v2(iter, max_iter, uniforms.time);
        return vec4<f32>(color, 1.0);
    }
}

// ✅ Palette functions remain unchanged
fn palette(i: u32, max_iter: u32) -> vec3<f32> {
    let t = f32(i) / f32(max_iter);
    return vec3<f32>(t, t, t);
}

fn palette_v2(i: u32, max_iter: u32, time: f32) -> vec3<f32> {
    let t = f32(i) / f32(max_iter);
    let pi2 = 6.2831;
    let speed = 1.0;
    let combined = t + time * speed;

    let red   = 0.5 + 0.5 * cos(pi2 * combined);
    let green = 0.5 + 0.5 * cos(pi2 * (combined + 0.3333));
    let blue  = 0.5 + 0.5 * cos(pi2 * (combined + 0.6666));

    return vec3<f32>(red, green, blue);
}
