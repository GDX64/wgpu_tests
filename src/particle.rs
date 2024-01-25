use crate::v2::{TreeValue, V2};

#[derive(Clone, Debug)]
pub struct Particle {
    pub position: V2,
    pub velocity: V2,
}

impl Particle {
    pub fn new(position: V2, velocity: V2) -> Particle {
        Particle { position, velocity }
    }
}

impl TreeValue for Particle {
    fn position(&self) -> V2 {
        self.position.clone()
    }

    fn offset_pos(&mut self) {
        self.position.x += 0.0001;
        self.position.y += 0.0001;
    }
}

pub struct World<T> {
    pub particles: Vec<Particle>,
    dimensions: V2,
    gravity: V2,
    step: f64,
    pub tree: T,
    pub mouse_pos: Option<V2>,
    pub show_quad_tree: bool,
    pub is_pressing_mouse: bool,
}

const PRESSURE_MULTIPLIER: f64 = 10000.;
const STEP: f64 = 0.006;
const FRICTION: f64 = 0.5;
pub const PARTICLE_RADIUS: f64 = 6.;
const MOUSE_FORCE: f64 = -200.;

fn smoothing_kernel_gradient(d: f64) -> f64 {
    let v = ((PARTICLE_RADIUS - d) / PARTICLE_RADIUS).max(0.);
    v.powi(2)
}

impl<T: GeoQuery<Particle>> World<T> {
    pub fn new(dimensions: V2, gravity: V2) -> World<T> {
        World {
            particles: Vec::new(),
            tree: T::from_vec(Vec::new(), dimensions.x.max(dimensions.y)),
            dimensions,
            gravity,
            step: STEP,
            mouse_pos: None,
            show_quad_tree: false,
            is_pressing_mouse: false,
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
        self.update_tree();
    }

    pub fn calc_force(&self, particle: &Particle) -> V2 {
        let mut gradient = V2::new(0., 0.);
        let point = &particle.position;
        self.tree.query_distance(point, PARTICLE_RADIUS, |other| {
            let p_vec = other.position.sub(&particle.position);
            let p_norm = p_vec.normalized();
            let d = p_vec.len();
            let g = -smoothing_kernel_gradient(d) * PRESSURE_MULTIPLIER;
            gradient = gradient.add(&&p_norm.scalar_mul(g));
            let collision_penalty = particle.velocity.sub(&other.velocity).scalar_mul(-FRICTION);
            gradient = gradient.add(&collision_penalty);
        });
        gradient
    }

    pub fn calc_particle_acc(&self, particle: &Particle) -> V2 {
        let acc = self.calc_force(&particle);
        if let Some(ref mouse_pos) = self.mouse_pos {
            if self.is_pressing_mouse {
                let mouse_distance = mouse_pos.sub(&particle.position);
                let l = mouse_distance.len();
                if l > 100. {
                    return acc;
                }
                let mouse_acc = mouse_distance.normalized().scalar_mul(-MOUSE_FORCE);
                return acc.add(&mouse_acc);
            }
        }
        return acc;
    }

    fn update_tree(&mut self) {
        self.tree = T::from_vec(
            self.particles.clone(),
            self.dimensions.x.max(self.dimensions.y),
        );
    }

    pub fn evolve(&mut self, n: usize) {
        for _ in 0..n {
            self._evolve();
        }
    }

    fn _evolve(&mut self) {
        let dt = self.step;
        self.update_tree();
        self.particles = self
            .particles
            .iter()
            .map(|p| {
                let mut particle = p.clone();
                let acc = self.calc_particle_acc(&particle).add(&self.gravity);
                // let force = acc.add(&self.gravity);
                particle.velocity = particle.velocity.add(&acc.scalar_mul(dt));
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

pub trait GeoQuery<T> {
    fn query_distance(&self, point: &V2, radius: f64, f: impl FnMut(&T));
    fn from_vec(vec: Vec<T>, max_dim: f64) -> Self;
}
