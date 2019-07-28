use yarge_core;

use yarge_core::*;

fn main() {
    let mut game = WorkspaceBuilder::build("Hello world", 800, 600);

    game.add_stage(Stage::new("default", &|game|{
        println!("Loading game resources");
        game.switch_stage("menu");
    }));

    game.add_stage(Stage::new("menu", &|game| {
        println!("Alternative Stage: {:?}", game);
    }));

    game.bootstrap();
}
