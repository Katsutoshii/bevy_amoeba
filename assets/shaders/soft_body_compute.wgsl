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

// Compute initial position from the given index.
fn get_position(i: u32) -> vec2<f32> {
    if (i == 0) {
        return vec2<f32>(0.0, 0.0);
    } else {
        let angle = (f32(i-1) / f32(num_particles)) * TAU;
        return vec2<f32>(cos(angle) * radius, sin(angle) * radius);
    }
}

// Compute initial velocity from the givin index.
fn get_velocity(i: u32) -> vec2<f32> {
    if (i == 0) {
        return vec2<f32>(0.0, 0.0);
    } else {
        let angle = (f32(i-1) / f32(num_particles)) * TAU;
        return (1 - 0.5 * sin(5.0 * angle)) * vec2<f32>(cos(angle) * velocity_factor, sin(angle) * velocity_factor);
    }
}

// Initialize the velocity of each particle.
// Positions are in a circle.
@compute @workgroup_size(#{WORKGROUP_SIZE_X})
fn init(in: ComputeInput) {
    let i = in.id.x;
    seed = pcg_hash(i ^ effect_seed);
    particles[i].position = get_position(i);
    particles[i].velocity = get_velocity(i);
    particles[i].color = white;
}

// Update the features of each particle.
@compute @workgroup_size(#{WORKGROUP_SIZE_X})
fn update(in: ComputeInput) {
    let i = in.id.x;

    particles[i].position += particles[i].velocity * dt;
    particles[i].velocity -= drag * particles[i].velocity * dt;
}
