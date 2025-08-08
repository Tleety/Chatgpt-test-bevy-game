pub mod components;
pub mod systems;
pub mod resources;
pub mod plugins;

// Re-export commonly used items
pub use components::*;
pub use systems::*;
pub use plugins::*;

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::prelude::*;

    #[test]
    fn test_velocity_component_default() {
        let velocity = Velocity::default();
        assert_eq!(velocity.x, 0.0);
        assert_eq!(velocity.y, 0.0);
    }

    #[test]
    fn test_components_exist() {
        // Test that we can create player and velocity components
        let _player = Player;
        let velocity = Velocity { x: 10.0, y: 20.0 };
        assert_eq!(velocity.x, 10.0);
        assert_eq!(velocity.y, 20.0);
    }
    
    #[test]
    fn test_basic_transform_manipulation() {
        // Test basic transform math that our movement system would do
        let mut transform = Transform::from_translation(Vec3::new(0.0, 0.0, 0.0));
        let velocity = Velocity { x: 100.0, y: 50.0 };
        let delta_time = 0.1;
        
        // Simulate what our movement system does
        transform.translation.x += velocity.x * delta_time;
        transform.translation.y += velocity.y * delta_time;
        
        assert_eq!(transform.translation.x, 10.0);
        assert_eq!(transform.translation.y, 5.0);
    }
}