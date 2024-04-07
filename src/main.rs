use bracket_lib::prelude::*;

fn main() -> BError {
    enum GameMode {
        Menu,
        Playing,
        End,
    }
    struct State {
        mode: GameMode,
    }
    impl State {
        fn new() ->  Self {
            State{
            mode: GameMode::Menu,
            }
        }
        fn main_menu (&mut self, ctx: &mut BTerm){
            ctx.cls();
            ctx.print(1, 1, "Flappy Dragon v0.0.1");
            ctx.print_centered(5, "Welcome to Flappy Dragon!");
            ctx.print_centered(8, "(P) Play Game");
            ctx.print_centered(9, "(Q) Quit Game");
            if let Some(key) = ctx.key {
                match key {
                    VirtualKeyCode::P => self.restart(),
                    VirtualKeyCode::Q => ctx.quitting = true,
                    _ => {}
                }
            }
        }
        fn dead (&mut self, ctx: &mut BTerm){
            ctx.cls();
            ctx.print(1, 1, "Flappy Dragon v0.0.1");
            ctx.print_centered(5, "You are DEAD!");
            ctx.print_centered(8, "(P) Play Again");
            ctx.print_centered(9, "(Q) Quit Game");
            if let Some(key) = ctx.key {
                match key {
                    VirtualKeyCode::P => self.restart(),
                    VirtualKeyCode::Q => ctx.quitting = true,
                    _ => {}
                }
            }
        }
        fn play (&mut self, ctx: &mut BTerm){
            //TODO: fill in
            self.mode = GameMode::End;
        }
        fn restart(&mut self) {
            self.mode = GameMode::Playing;
        }
    }
    impl GameState for State {
        fn tick(&mut self, ctx: &mut BTerm) {
            match self.mode {
                GameMode::Menu => self.main_menu(ctx),
                GameMode::Playing => self.play(ctx),
                GameMode::End => self.dead(ctx),
            }
        }
    }
    let context = BTermBuilder::simple80x50()
        .with_title("Flappy Dragon")
        .build()?;
    main_loop(context, State::new())
}
