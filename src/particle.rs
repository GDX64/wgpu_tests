use rayon::prelude::*;

use crate::quad_tree;

#[derive(Clone, Debug, PartialEq)]
pub struct V2 {
    pub x: f64,
    pub y: f64,
}

impl V2 {
    pub fn new(x: f64, y: f64) -> V2 {
        V2 { x, y }
    }

    pub fn sub(&self, other: &V2) -> V2 {
        V2::new(self.x - other.x, self.y - other.y)
    }

    pub fn add(&self, other: &V2) -> V2 {
        V2::new(self.x + other.x, self.y + other.y)
    }

    pub fn len(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn scalar_mul(&self, scalar: f64) -> V2 {
        V2::new(self.x * scalar, self.y * scalar)
    }
}

#[derive(Clone, Debug)]
pub struct Particle {
    pub position: V2,
    pub velocity: V2,
}

impl Particle {
    pub fn new(position: V2, velocity: V2) -> Particle {
        Particle { position, velocity }
    }

    pub fn smoothing_gradient(&self, point: &V2) -> V2 {
        let d = self.position.sub(point).len();
        let g = smoothing_kernel_gradient(d);
        self.position.sub(point).scalar_mul(g)
    }
}

pub struct World {
    pub particles: Vec<Particle>,
    dimensions: V2,
    gravity: V2,
    step: f64,
    tree: quad_tree::QuadTree<Particle>,
    mouse_pos: Option<V2>,
}

const PARTICLE_MASS: f64 = 1.;
const PRESSURE_MULTIPLIER: f64 = 50.;
const STEP: f64 = 0.001;
const FRICTION: f64 = 0.0001;
pub const PARTICLE_RADIUS: f64 = 6.;
const MOUSE_FORCE: f64 = 2000.;

fn smoothing_kernel_gradient(d: f64) -> f64 {
    let v = (PARTICLE_RADIUS - d).max(0.);
    -v.powi(2)
}

impl World {
    pub fn new(dimensions: V2, gravity: V2) -> World {
        World {
            particles: Vec::new(),
            tree: quad_tree::QuadTree::new(V2::new(0., 0.), dimensions.x, dimensions.y),
            dimensions,
            gravity,
            step: STEP,
            mouse_pos: None,
        }
    }

    pub fn update_mouse_pos(&mut self, mouse_pos: Option<V2>) {
        self.mouse_pos = mouse_pos;
    }

    pub fn add_random_particles(&mut self, n: usize, rng: impl Fn() -> f64) {
        for _ in 0..n {
            let x = rng() * self.dimensions.x;
            let y = rng() * self.dimensions.y;
            let vx = rng() * 100. - 50.;
            let vy = rng() * 100. - 50.;
            let particle = Particle::new(V2::new(x, y), V2::new(vx, vy));
            self.particles.push(particle);
        }
        self.update_quadtree();
    }

    pub fn calc_force(&self, point: &V2) -> (V2, f64) {
        let mut gradient = V2::new(0., 0.);
        let mut neightbours = 0.;
        self.tree.query_distance(point, PARTICLE_RADIUS, |other| {
            let d = point.sub(&other.position).len();
            if d <= 0.001 {
                return;
            }
            let g = smoothing_kernel_gradient(d);
            gradient.x += g * (point.x - other.position.x);
            gradient.y += g * (point.y - other.position.y);
            neightbours += 1.;
        });
        (gradient.scalar_mul(PARTICLE_MASS), neightbours)
    }

    pub fn calc_particle_acc(&self, particle: &Particle) -> (V2, f64) {
        let pressure = PRESSURE_MULTIPLIER;
        let (gradient, n) = self.calc_force(&particle.position);
        let acc = gradient.scalar_mul(-pressure);
        if let Some(ref mouse_pos) = self.mouse_pos {
            let mouse_distance = mouse_pos.sub(&particle.position);
            let l = mouse_distance.len();
            let mouse_acc = mouse_distance.scalar_mul(-MOUSE_FORCE / l.powi(2));
            return (acc.add(&mouse_acc), n);
        }
        return (acc, n);
    }

    fn update_quadtree(&mut self) {
        self.tree = quad_tree::QuadTree::new(V2::new(0., 0.), self.dimensions.x, self.dimensions.y);
        self.particles.iter().for_each(|particle| {
            self.tree.insert(particle.clone());
        });
    }

    pub fn evolve(&mut self, n: usize) {
        for _ in 0..n {
            self._evolve();
        }
    }

    fn _evolve(&mut self) {
        let dt = self.step;
        self.update_quadtree();
        self.particles = self
            .particles
            .par_iter()
            .map(|p| {
                let mut particle = p.clone();
                let (acc, n) = self.calc_particle_acc(&particle);
                let force = acc.add(&self.gravity);
                let friction = (1. - FRICTION * n).max(0.);
                particle.velocity = particle.velocity.add(&force.scalar_mul(dt));
                particle.velocity = particle.velocity.scalar_mul(friction);
                particle.position = particle.position.add(&particle.velocity.scalar_mul(dt));

                if particle.position.x < 0. {
                    particle.position.x = 0.;
                    particle.velocity.x = -particle.velocity.x;
                }

                if particle.position.x > self.dimensions.x {
                    particle.position.x = self.dimensions.x;
                    particle.velocity.x = -particle.velocity.x;
                }

                if particle.position.y < 0. {
                    particle.position.y = 0.;
                    particle.velocity.y = -particle.velocity.y;
                }

                if particle.position.y > self.dimensions.y {
                    particle.position.y = self.dimensions.y;
                    particle.velocity.y = -particle.velocity.y;
                }
                return particle;
            })
            .collect();
    }
}
