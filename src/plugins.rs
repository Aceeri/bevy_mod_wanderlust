use crate::controller::*;
use crate::physics::*;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

/// The [character controller](CharacterController) plugin. Necessary to have the character controller
/// work.
pub struct WanderlustPlugin {
    pub tweaks: bool,
    pub default_systems: bool,
}

impl Default for WanderlustPlugin {
    fn default() -> Self {
        Self { tweaks: true, default_systems: true }
    }
}

impl Plugin for WanderlustPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<ControllerInput>()
            .register_type::<Option<Vec3>>()
            .register_type::<GravityForce>()
            .register_type::<Gravity>()
            .register_type::<JumpForce>()
            .register_type::<Jump>()
            .register_type::<FloatForce>()
            .register_type::<Float>()
            .register_type::<ControllerForce>()
            .register_type::<ControllerMass>()
            .register_type::<ControllerVelocity>()
            .register_type::<Parts>()
            .register_type::<Vec<Entity>>();

        if self.tweaks {
            app.add_systems(Startup, setup_physics_context);
        }

        if self.default_systems {
        app.add_systems(
            Update,
            (
                crate::get_mass_from_rapier,
                crate::get_velocity_from_rapier,
                find_ground,
                determine_groundedness,
                gravity_force,
                movement_force,
                float_force,
                upright_force,
                jump_force,
                accumulate_forces,
                crate::apply_forces,
                crate::apply_ground_forces,
            )
                .chain()
                .before(PhysicsSet::SyncBackend),
        );

        #[cfg(feature = "debug-lines")]
        app.add_systems(Update, |casts: Query<&GroundCast>, mut gizmos: Gizmos| {
            for cast in &casts {
                if let Some((entity, toi, velocity)) = cast.cast {
                    gizmos.sphere(toi.witness1, Quat::IDENTITY, 0.3, Color::LIME_GREEN);
                }
            }
        });
        }
    }
}

/// *Note: Most users will not need to use this directly. Use [`WanderlustPlugin`](crate::plugins::WanderlustPlugin) instead.
/// Alternatively, if one only wants to disable the system, use [`WanderlustPhysicsTweaks`](WanderlustPhysicsTweaks).*
///
/// This system adds some tweaks to rapier's physics settings that make the character controller behave better.
pub fn setup_physics_context(mut ctx: ResMut<RapierContext>) {
    let params = &mut ctx.integration_parameters;
    // This prevents any noticeable jitter when running facefirst into a wall.
    params.erp = 0.99;
    // This prevents (most) noticeable jitter when running facefirst into an inverted corner.
    params.max_velocity_iterations = 16;
    // TODO: Fix jitter that occurs when running facefirst into a normal corner.
}
