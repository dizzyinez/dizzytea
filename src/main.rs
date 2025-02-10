use flecs_ecs::prelude::*;

fn main() {
    let ecs = World::new();
    ecs.app()
        .enable_rest(0)
        .run()
        ;
}
