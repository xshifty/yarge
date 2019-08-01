use yarge::{
    workspace::{WorkspaceBuilder},
    stage::{StageBuilder},
    sprite::{SpriteBuilder},
};

fn main() {
    let mut game = WorkspaceBuilder::build("Hello world", [800, 600]);

    game.add_stage(StageBuilder::build("default", &|game, events|{
        let fighter = SpriteBuilder::build("resources/fighter.png", [5, 2], [0, 0], [180, 220]);

        game.add_sprite(fighter);
    }));

    game.bootstrap();
}
