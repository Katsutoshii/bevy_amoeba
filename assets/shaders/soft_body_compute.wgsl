#import bevy_amoeba::vertices::SoftBodyVertex2d;


struct ComputeInput {
    @builtin(global_invocation_id) id: vec3<u32>,
};

@group(0) @binding(0) var<storage, read_write> vertices: array<SoftBodyVertex2d>;
@group(0) @binding(1) var<storage, read> nodes: array<vec2<f32>>;
@group(0) @binding(2) var<uniform> num_vertices: u32;

const TAU: f32 = 6.28318531;
const PI: f32 =  3.14159274;

const RADIUS: f32 = 1.0;

// Compute initial position from the given index.
fn get_position(i: u32, n: u32) -> vec2<f32> {
    let angle = (f32(i) / f32(n)) * TAU;
    return vec2<f32>(cos(angle) * RADIUS, sin(angle) * RADIUS);
}

// Compute L2 distance squared.
fn l2_squared(p: vec2<f32>) -> f32 {
    return p.x * p.x + p.y * p.y;
}

// Get the closest circle-line intersection point to p1.
// Quadratic coefficients: A t^2 + B t + C = 0
fn get_closest_circle_line_intersection(
    c: vec2<f32>,
    r: f32,
    p1: vec2<f32>,
    p2: vec2<f32>,
) -> vec2<f32> {
    let d = p2 - p1;
    let A = dot(d, d);
    let f = p1 - c;
    let B = 2.0 * dot(f, d);
    let C = dot(f, f) - (r * r);

    let discriminant = (B * B) - (4.0 * A * C);
    if (discriminant < 0.0) {
        return p2;
    }

    let epsilon = 0.00001; 
    let t1 = (-B - sqrt(discriminant)) / (2.0 * A);
    let t2 = (-B + sqrt(discriminant)) / (2.0 * A);

    if (abs(discriminant) < epsilon) {
        return p1 + t1 * d;
    }

    let q1 = p1 + t1 * d;
    let q2 = p1 + t2 * d;
    if (l2_squared(q1 - p1) < l2_squared(q2 - p1)) {
        return q1;
    } else {
        return q2;
    }
}

// Find the closest intersection from point p to the origin and the nodes.
fn get_closest_intersection(p: vec2<f32>) -> vec2<f32> {
    const o = vec2<f32>(0.0, 0.0);
    var min_d2 = 1000000.0;
    var min_q = p;
    for (var i: u32 = 0; i < arrayLength(&nodes); i += 1) {
        let c = nodes[i].xy;
        let q = get_closest_circle_line_intersection(c, 0.5, p, o);
        let d2 = l2_squared(p - q);
        if (d2 < min_d2) {
            min_d2 = d2;
            min_q = q;
        }
    }
    return min_q;
}

// Mean position of the 5 approximate neighbors of particle i.
fn mean_position5(i: u32, n: u32) -> vec2<f32> {
    return (
        vertices[i].position
        + vertices[(i + 2) % n].position
        + vertices[(i + 1) % n].position
        + vertices[(i - 1) % n].position
        + vertices[(i - 2) % n].position
    ) / 5.0;
}

// Initialize the velocity of each particle.
@compute @workgroup_size(#{WORKGROUP_SIZE_X})
fn init(in: ComputeInput) {
    let i = in.id.x;
    
    let position0 = get_position(i, num_vertices);
    vertices[i].position = get_closest_intersection(position0);

    for (var s = 0; s < 3; s += 1) {   
        storageBarrier();
        vertices[i].position = mean_position5(i, num_vertices);
    }
}

// Update positions of each particle.
@compute @workgroup_size(#{WORKGROUP_SIZE_X})
fn update(in: ComputeInput) {
    let i = in.id.x;

    let position0 = get_position(i, num_vertices);
    vertices[i].position = get_closest_intersection(position0);

    for (var s = 0; s < 3; s += 1) {   
        storageBarrier();
        vertices[i].position = mean_position5(i, num_vertices);
    }
}
