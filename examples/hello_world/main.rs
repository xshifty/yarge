use yarge::{
    workspace::{WorkspaceBuilder},
    stage::{StageBuilder},
    sprite::{SpriteBuilder},
};

fn main() {
    let mut game = WorkspaceBuilder::build("Hello world", [800, 600]);

    game.add_stage(StageBuilder::build("default", &|game, events|{
        let fighter = SpriteBuilder::build("resources/fighter.png", [0, 0], [256, 488], [60, 100]);
        game.add_sprite(fighter);
    }));

    game.bootstrap();
}
