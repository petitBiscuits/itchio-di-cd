use std::f32::consts::PI;
use bevy::math::Vec2;

pub fn constrain_distance(point:&Vec2, anchor:&Vec2, distance:f32) -> Vec2 {
    ((point - anchor).normalize_or_zero() * distance) + anchor
}

pub fn constrain_angle(prev: Vec2, curr: Vec2, next: Vec2, max_angle: f32) -> Vec2 {
    let vec1 = (curr - prev).normalize();
    let vec2 = (next - curr).normalize();
    let dot = vec1.dot(vec2).clamp(-1.0, 1.0);
    let current_angle = dot.acos();

    if current_angle > max_angle {
        // Determine the rotation needed. One approach:
        let angle_diff = current_angle - max_angle;
        // Rotate vec1 by +angle_diff (or -angle_diff, depending on your desired behavior)
        let new_direction = rotate(vec1, angle_diff);
        // Recalculate 'next' position based on the new direction while keeping the distance constant
        let distance = (next - curr).length();
        return curr + new_direction * distance;
    }
    next
}

pub fn contraint_angle_v2(angle: f32, anchor: f32, consraint: f32) -> f32 {
    // println!("{} : {}", relative_angle_diff(angle, anchor), consraint);
    if (relative_angle_diff(angle, anchor) <= consraint) {
        return simplify_angle(angle);
    }

    if (relative_angle_diff(angle, anchor) > consraint) {
        return simplify_angle(anchor + consraint);
    }

    simplify_angle(anchor + consraint)
}

fn relative_angle_diff(mut angle: f32, mut anchor: f32) -> f32 {
    angle = simplify_angle(angle + PI - anchor);
    anchor = PI;

    anchor - angle
}

fn simplify_angle(mut angle:f32) -> f32 {
    while (angle >= PI*2.) {
        angle -= PI*2.;
    }

    while (angle < 0.) {
        angle += PI*2.;
    }

    angle
}


fn rotate(vector: Vec2, angle: f32) -> Vec2 {
    let (sin, cos) = angle.sin_cos();
    Vec2 {
        x: vector.x * cos - vector.y * sin,
        y: vector.x * sin + vector.y * cos,
    }
}