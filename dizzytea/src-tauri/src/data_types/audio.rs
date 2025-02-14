use flecs_ecs::prelude::*;

#[derive(Debug, Component)]
struct Music {
    title: String,
    artist: String,
    album: String,
    album_artist: String,
    genre: String,
    track_number: u32,
    release_year: u32,
    duration: u32,
}
