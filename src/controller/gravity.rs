use crate::controller::*;

/// How strong is the gravity for this controller.
#[derive(Component, Reflect)]
#[reflect(Component, Default)]
pub struct Gravity {
    /// Acceleration in the `up_vector` direction due to gravity.
    ///
    /// The default is `-9.817`, but for most games it is recommended to
    /// use a higher acceleration. The reasoning being that normal/reality-based
    /// gravity tends to feel floaty.
    pub acceleration: f32,
    /// Direction we should float up from.
    ///
    /// The default is `Vec3::Y`.
    pub up_vector: Vec3,
    /// Direction we face.
    ///
    /// The default is `Vec3::NEG_Z`.
    pub forward_vector: Vec3,
}

impl Default for Gravity {
    fn default() -> Self {
        Gravity {
            acceleration: -9.817,
            up_vector: Vec3::Y,
            //up_vector: (Vec3::new(1.0, 0.0, 0.0) + Vec3::new(0.0, 0.0, 1.0)).normalize(),
            forward_vector: Vec3::NEG_Z,
        }
    }
}

impl Gravity {
    pub fn project(&self, other: Vec3) -> Vec3 {
        let up = self.up_vector.normalize();
        if up.length_squared() > 0.0 {
            let (x, z) = up.any_orthonormal_pair();
            other.project_onto(x) + other.project_onto(z)
        } else {
            other
        }
    }

    pub fn rotation(&self) -> Quat {
        Transform::default().looking_to(self.forward_vector, self.up_vector).rotation
        //Quat::from_rotation_arc(self.up_vector, Vec3::Y)
    }
}

/// Calculated gravity force.
#[derive(Component, Default, Reflect)]
#[reflect(Component, Default)]
pub struct GravityForce {
    /// Linear gravitational force.
    pub linear: Vec3,
}

/// Calculate gravity force.
pub fn gravity_force(mut query: Query<(&GlobalTransform, &mut GravityForce, &Gravity, &ControllerMass)>, mut gizmos: Gizmos) {
    for (global, mut force, gravity, mass) in &mut query {
        force.linear = gravity.up_vector * mass.mass * gravity.acceleration;
        gizmos.ray(global.translation(), force.linear, Color::YELLOW);
    }
}
