pub struct V2 {
    pub x: f64,
    pub y: f64,
}

impl V2 {
    pub fn new(x: f64, y: f64) -> V2 {
        V2 { x, y }
    }
}

pub struct Particle {
    pub position: V2,
    pub velocity: V2,
}

impl Particle {
    pub fn new(position: V2, velocity: V2) -> Particle {
        Particle { position, velocity }
    }
}

pub struct World {
    pub particles: Vec<Particle>,
    dimensions: V2,
    gravity: V2,
    step: f64,
}

impl World {
    pub fn new(dimensions: V2, gravity: V2) -> World {
        World {
            particles: Vec::new(),
            dimensions,
            gravity,
            step: 0.05,
        }
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
    }

    pub fn add_particle(&mut self, particle: Particle) {
        self.particles.push(particle);
    }

    pub fn evolve(&mut self) {
        let dt = self.step;
        for particle in &mut self.particles {
            particle.position.x += particle.velocity.x * dt;
            particle.position.y += particle.velocity.y * dt;
            particle.velocity.x += self.gravity.x * dt;
            particle.velocity.y += self.gravity.y * dt;

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
        }
    }
}
