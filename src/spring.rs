use bevy::{math::*, prelude::*};

/// Spring parameters for a dampened harmonic oscillator.
///
/// Some good readings on this:
/// - https://www.ryanjuckett.com/damped-springs/
/// - https://gafferongames.com/post/spring_physics/
#[derive(Debug, Clone, Copy, Reflect)]
pub struct Spring {
    /// How strong the spring will push it into position.
    pub frequency: f32,
    /// Damping ratio for the spring, this prevents endless oscillation if greater than 0.
    /// <1 is under-dampened so it will overshoot the target
    /// 1 is critically dampened so it will slow just enough to reach the target without overshooting
    /// >1 is over-dampened so it will reach the target slowly.
    pub damp_ratio: f32,
}

impl Default for Spring {
    fn default() -> Self {
        Self {
            frequency: 1.0,
            damp_ratio: 0.25,
        }
    }
}

impl Spring {
    pub fn stiffness(&self, mass: f32) -> f32 {
        mass * self.frequency * self.frequency
    }

    /// The damping coefficient that will just reach the target without overshooting.
    pub fn critical_damping_point(&self, mass: f32) -> f32 {
        2.0 * (mass * self.stiffness(mass)).sqrt()
    }

    /// Get the correct damping coefficient for our damping ratio.
    /// See [`Spring`]'s damping for more information on the ratio.
    pub fn damp_coefficient(&self, mass: f32) -> f32 {
        self.damp_ratio * self.critical_damping_point(mass)
    }

    pub fn soft_constraint(
        &self,
        mass: f32,
        displacement: f32,
        relative_velocity: f32,
        delta_time: f32,
    ) -> f32 {
        let ks = self.stiffness(mass);
        let kc = self.damp_coefficient(mass);

        // 1 / c + hk
        let gamma = 1.0 / (kc + delta_time * ks);
        // hk / c + hk
        let beta = (delta_time * ks) / (kc + delta_time * ks);

        let bias = beta / delta_time * displacement;

        let lambda = -(relative_velocity + bias) / (1.0 / mass + gamma);
        lambda
    }
}

#[cfg(test)]
mod tests {
    use super::Spring;
    use bevy::prelude::*;
    use plotly::common::*;
    use plotly::*;

    pub fn damped_harmonic_oscillator(
        frequency: f32,
        damp_ratio: f32,
        timestep: f32,
    ) -> Box<dyn Trace> {
        let mut time = 0.0;

        let mass = 1.0;
        let mut position = 1.0;
        let mut velocity = 0.0;
        let mut lambda = 0.0;

        let mut plot_x = Vec::new();
        let mut plot_y = Vec::new();

        let integrate =
            |position: &mut f32, velocity: &mut f32, lambda: &mut f32, time: &mut f32| {
                let spring = Spring {
                    frequency: frequency,
                    damp_ratio: damp_ratio,
                };
                let damping = spring.damp_coefficient(mass);
                *lambda = -spring.stiffness(mass) * *position - damping * *velocity;

                *velocity += *lambda * timestep / mass;
                *position += *velocity * timestep;

                *time += timestep;
            };

        plot_x.push(time);
        plot_y.push(position);

        let per_second = 1.0 / timestep;

        for _ in 0..(per_second * 0.25) as usize {
            integrate(&mut position, &mut velocity, &mut lambda, &mut time);
            plot_x.push(time);
            plot_y.push(position);
            println!("x, v, a: {:.4?} {:.4?} {:.4?}", position, velocity, lambda);
        }

        println!("------");
        // simulate 1/4 second
        for _ in 0..(per_second * 0.25) as usize {
            integrate(&mut position, &mut velocity, &mut lambda, &mut time);
            plot_x.push(time);
            plot_y.push(position);
        }
        println!("x, v, a: {:.4?} {:.4?} {:.4?}", position, velocity, lambda);
        // simulate 1/4 second
        for _ in 0..(per_second * 0.25) as usize {
            integrate(&mut position, &mut velocity, &mut lambda, &mut time);
            plot_x.push(time);
            plot_y.push(position);
        }
        println!("x, v, a: {:.4?} {:.4?} {:.4?}", position, velocity, lambda);
        // simulate 10 seconds
        for _ in 0..(per_second * 10.0) as usize {
            integrate(&mut position, &mut velocity, &mut lambda, &mut time);
            plot_x.push(time);
            plot_y.push(position);
        }
        println!("x, v, a: {:.4?} {:.4?} {:.4?}", position, velocity, lambda);

        Scatter::new(plot_x, plot_y)
            .web_gl_mode(true)
            .mode(Mode::Lines)
    }

    pub fn soft_constraint(frequency: f32, damp_ratio: f32, timestep: f32) -> Box<dyn Trace> {
        let mut time = 0.0;

        let mass = 1.0;
        let mut position = 1.0;
        let mut velocity = 0.0;
        let mut lambda = 0.0;

        let mut plot_x = Vec::new();
        let mut plot_y = Vec::new();

        let integrate =
            |position: &mut f32, velocity: &mut f32, lambda: &mut f32, time: &mut f32| {
                let spring = Spring {
                    frequency: frequency,
                    damp_ratio: damp_ratio,
                };
                let force = spring.soft_constraint(1.0, *position, *velocity, timestep);
                *lambda = force;

                *velocity += *lambda * timestep / mass;
                *position += *velocity * timestep;

                *time += timestep;
            };

        let per_second = 1.0 / timestep;

        for _ in 0..(per_second * 0.25) as usize {
            integrate(&mut position, &mut velocity, &mut lambda, &mut time);
            plot_x.push(time);
            plot_y.push(position);
            println!("x, v, a: {:.4?} {:.4?} {:.4?}", position, velocity, lambda);
        }

        println!("------");
        // simulate 1/4 second
        for _ in 0..(per_second * 0.25) as usize {
            integrate(&mut position, &mut velocity, &mut lambda, &mut time);
            plot_x.push(time);
            plot_y.push(position);
        }
        println!("x, v, a: {:.4?} {:.4?} {:.4?}", position, velocity, lambda);
        // simulate 1/4 second
        for _ in 0..(per_second * 0.25) as usize {
            integrate(&mut position, &mut velocity, &mut lambda, &mut time);
            plot_x.push(time);
            plot_y.push(position);
        }
        println!("x, v, a: {:.4?} {:.4?} {:.4?}", position, velocity, lambda);
        // simulate 10 seconds
        for _ in 0..(per_second * 10.0) as usize {
            integrate(&mut position, &mut velocity, &mut lambda, &mut time);
            plot_x.push(time);
            plot_y.push(position);
        }
        println!("x, v, a: {:.4?} {:.4?} {:.4?}", position, velocity, lambda);

        use plotly::common::*;
        use plotly::*;
        //let trace = Scatter::new(plot_x, plot_y)
        //    .mode(plotly::common::Mode::Lines);
        //println!("plot_x: {:?}", plot_x.len());
        let trace = Scatter::new(plot_x, plot_y)
            .web_gl_mode(true)
            .mode(Mode::Lines);
        trace
    }

    #[test]
    pub fn plot() {
        let mut plot = Plot::new();
        let timestep = 1.0 / 60.0;
        for i in 0..10 {
            //plot.add_trace(damped_harmonic_oscillator(i as f32, 1.0));
        }
        plot.add_trace(damped_harmonic_oscillator(1.0, 2.0, timestep));
        plot.add_trace(soft_constraint(1.0, 2.0, timestep));
        //plot.add_trace(soft_constraint(1.0, 0.0, timestep / 2.0));
        //plot.add_trace(soft_constraint(1.0, 0.0, timestep / 4.0));
        plot.show();
    }
}
