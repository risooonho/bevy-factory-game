mod assets;
mod claw;
mod common;
mod conveyor;
mod furnace;
pub mod iso_pos;
mod item;
pub mod prelude;
mod ui;
mod util;

use bevy::prelude::*;
use prelude::*;

fn test_scene(commands: &mut Commands, common_assets: Res<CommonAssets>) {
    spawn::spawner(commands, &common_assets, IsoPos::new(-5, -3), 8);
    spawn::spawner(commands, &common_assets, IsoPos::new(-5, -4), 8);
    spawn::destroyer(commands, &common_assets, IsoPos::new(-5, -6));
}

fn main() {
    App::build()
        .add_resource(WindowDescriptor {
            width: 600.0,
            height: 500.0,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(common::Plug)
        .add_plugin(assets::Plug)
        .add_plugin(ui::Plug)
        .add_plugin(util::Plug)
        .add_plugin(furnace::Plug)
        .add_plugin(claw::Plug)
        .add_plugin(conveyor::Plug)
        .add_plugin(item::Plug)
        .add_startup_system(test_scene.system())
        .run();
}
