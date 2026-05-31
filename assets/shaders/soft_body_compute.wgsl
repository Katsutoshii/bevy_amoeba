#import bevy_amoeba::particle::{Particle, ComputeInput, ComputeUniform, particles, frand3, seed, pcg_hash, TAU, PI, frand};

@group(0) @binding(1) var<uniform> num_particles: u32;
@group(0) @binding(2) var<uniform> dt: f32;

const half3 = vec3<f32>(0.5, 0.5, 0.5);
const white = vec4<f32>(1.0, 1.0, 1.0, 1.0);
const drag: f32 = 1.0;
const alpha_fade: f32 = 1.0;
const velocity_factor: f32 = 0.2;
const effect_seed: u32 = 12345;

const radius: f32 = 1.0;

const num_points = 3;
const points = array(
    vec2<f32>(-0.1, 0.1),
    vec2<f32>(-0.3, -0.3),
    vec2<f32>(0.2, 0.2),
);

// Compute initial position from the given index.
fn get_position(i: u32) -> vec2<f32> {
    let angle = (f32(i) / f32(num_particles)) * TAU;
    return vec2<f32>(cos(angle) * radius, sin(angle) * radius);
}

// Compute initial velocity from the givin index.
fn get_velocity(i: u32) -> vec2<f32> {
    return vec2<f32>(0.0, 0.0);
    // let angle = (f32(i) / f32(num_particles)) * TAU;
    // return (1 - 0.5 * sin(5.0 * angle)) * vec2<f32>(cos(angle) * velocity_factor, sin(angle) * velocity_factor);
}

// Compute the spring force between two point indices.
fn get_spring_force(i: u32, n: u32, k: f32) -> vec2<f32> {
    // Vector from i -> n
    let delta = particles[n].position - particles[i].position;
    let r = length(delta) * 100.0;
    return delta * r * r * k;
}

fn l2_squared(p: vec2<f32>) -> f32 {
    return p.x * p.x + p.y * p.y;
}

fn collides_with_any(position: vec2<f32>) -> bool {
    const r = 0.5;
    const r2 = r * r;
    for (var i = 0; i < num_points; i += 1) {
        let delta = position - points[i];
        let d2 = l2_squared(delta);
        if (d2 <= r2) {
          return true;
        }
    }
    return false;
}

fn get_shrink_position(position0: vec2<f32>) -> vec2<f32> {
    const step = 0.05;
    for (var d: f32 = 2.0; d > 0.2; d -= step) {
      let position = position0 * d;
      if (collides_with_any(position)) {
        return position0 * (d + step);
      }
    }
    return position0;
}

fn get_spring_forces(i: u32, k: f32) -> vec2<f32> {
    var force = vec2<f32>(0.0, 0.0);
    force += get_spring_force(i, (i + 1) % num_particles, k);
    force += get_spring_force(i, (i - 1) % num_particles, k);
    return force;
}

// Initialize the velocity of each particle.
// Positions are in a circle.
@compute @workgroup_size(#{WORKGROUP_SIZE_X})
fn init(in: ComputeInput) {
    let i = in.id.x;
    seed = pcg_hash(i ^ effect_seed);
    
    let position0 = get_position(i);
    particles[i].position = get_shrink_position(position0);
    particles[i].velocity = get_velocity(i);
    particles[i].color = white;
}

// Update the features of each particle.
@compute @workgroup_size(#{WORKGROUP_SIZE_X})
fn update(in: ComputeInput) {
    let i = in.id.x;

    let position0 = get_position(i);
    particles[i].position = get_shrink_position(position0);

    for (var s = 0; s < 3; s += 1) {   
        storageBarrier();
        particles[i].position = (
                particles[i].position
                + particles[(i + 2) % num_particles].position
                + particles[(i + 1) % num_particles].position
                + particles[(i - 1) % num_particles].position
                + particles[(i - 2) % num_particles].position
            ) / 5.0;
    }
}
