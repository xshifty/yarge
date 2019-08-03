use yarge::{
    workspace::{WorkspaceBuilder},
    stage::{StageBuilder},
    sprite::{SpriteBuilder},
};

fn main() {
    let mut game = WorkspaceBuilder::build("Hello world", [800, 600]);
    let x = 50;

    game.add_sprite("fighter1", SpriteBuilder::build("resources/fighter.png", [5, 2], [0, 0], [180, 250]))
        .add_sprite("fighter2", SpriteBuilder::build("resources/fighter.png", [5, 2], [0, 0], [180, 250]).set_current_frame(1))
        .add_sprite("fighter3", SpriteBuilder::build("resources/fighter.png", [5, 2], [0, 0], [180, 250]).set_current_frame(2))
        .add_stage(StageBuilder::build("default", &|game, events|{
            for event in events {
                match event {
                    sdl2::event::Event::KeyDown{keycode: Some(sdl2::keyboard::Keycode::Right), ..} => {
                    },
                    _ => {}
                }
            }
            game.draw_sprite("fighter1", [50, 50], [90, 125]);
            game.draw_sprite("fighter2", [250, 50], [90, 125]);
            game.draw_sprite("fighter3", [450, 50], [90, 125]);
        }))
        .bootstrap();
}
