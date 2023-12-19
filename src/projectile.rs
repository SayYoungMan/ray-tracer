use crate::tuples;

pub struct Projectile {
    pub position: tuples::SpatialTuple,
    pub velocity: tuples::SpatialTuple,
}

pub struct Environment {
    pub gravity: tuples::SpatialTuple,
    pub wind: tuples::SpatialTuple,
}

fn tick(env: &Environment, proj: &mut Projectile) {
    proj.position = proj.position + proj.velocity;
    proj.velocity = proj.velocity + env.gravity + env.wind;
}

pub fn tick_until_fallen(env: &Environment, proj: &mut Projectile) {
    let mut height = proj.position.1;

    while height > 0.0 {
        tick(env, proj);
        println!(
            "Current position: {:#?}, Current velocity: {:#?}",
            proj.position, proj.velocity
        );
        height = proj.position.1;
    }
}
