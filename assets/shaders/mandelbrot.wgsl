struct Uniforms {
    time: f32,
    zoom: f32,
};

@group(2) @binding(0)
var<uniform> uniforms: Uniforms;

@fragment
fn fragment(@location(0) frag_uv: vec2<f32>) -> @location(0) vec4<f32> {
    let x0 = frag_uv.x * 0.003;
    let y0 = frag_uv.y * 0.003;

    var z = vec2<f32>(0.0, 0.0);
    var iter = 0u;
    let max_iter = 100u;

    // Mandelbrot iteration
    loop {
        if (iter >= max_iter || dot(z, z) > 4.0) {
            break;
        }
        let xtemp = z.x*z.x - z.y*z.y + x0;
        z.y = 2*z.x*z.y+y0;
        z.x = xtemp;
        iter += 1u;
    }
    // Map iteration count to a color (simple grayscale example)
    if (iter == max_iter){
        return vec4<f32>(0.,0.,0., 1.0);
    }else{
        var color = palette_v2(iter, max_iter, uniforms.time);
        return vec4<f32>(color, 1.0);
    }
}

fn palette(i: u32, max_iter: u32) -> vec3<f32> {
    let t = f32(i) / f32(max_iter);
    return vec3<f32>(t, t, t); // from black to white
}

fn palette_v2(i: u32, max_iter: u32, time: f32) -> vec3<f32> {
    // Normalisation de l'itération sur l'intervalle [0, 1]
    let t = f32(i) / f32(max_iter);

    // Constante pour un cycle complet (2π)
    let pi2 = 6.2831;

    // Multiplicateur pour le décalage temporel (ajustez pour accélérer ou ralentir l'animation)
    let speed = 1.;  // Par exemple, 0.5; augmentez cette valeur pour voir un changement plus rapide

    // Calcul d'un décalage combiné qui mélange l'itération et le temps
    let combined = t + time * speed;

    // Génération des composantes R, G, B à l'aide de fonctions cosinus avec des décalages de phase
    let red   = 0.5 + 0.5 * cos(pi2 * combined);
    let green = 0.5 + 0.5 * cos(pi2 * (combined + 0.3333)); // Décalage d'environ 120° (1/3 de cycle)
    let blue  = 0.5 + 0.5 * cos(pi2 * (combined + 0.6666)); // Décalage d'environ 240° (2/3 de cycle)

    return vec3<f32>(red, green, blue);
}


