#![warn(clippy::all, clippy::pedantic)]
use bracket_lib::prelude::*;
enum GameMode {
    Menu,
    Playing,
    End,
}
const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;
const FRAME_DURATION: f32 = 75.0;
struct Player {
    x: i32,
    y: i32,
    x_velocity: f32,
    y_velocity: f32,
}
impl Player {
    fn new() -> Self {
        Player {
            x: 5,
            y: 25,
            x_velocity: 1.0,
            y_velocity: 0.0,
        }
    }
    fn render(&mut self, ctx: &mut BTerm) {
        ctx.set(0, self.y, YELLOW, BLACK, to_cp437('&'));
    }
    fn gravity(&mut self) {
        if self.y_velocity < 2.0 {
            self.y_velocity += 0.2;
        }
    }
    #[allow(clippy::cast_possible_truncation)]
    fn motion(&mut self) {
        self.x += self.x_velocity as i32;
        self.y += self.y_velocity as i32;
        if self.x < 0 {
            self.x = 0;
        }
        if self.y < 0 {
            self.y = 0;
        }
    }
    fn flap(&mut self) {
        self.y_velocity += -2.0;
    }
}
struct Obstacle {
    x: i32,
    gap_y: i32,
    gap_size: i32,
}
impl Obstacle {
    fn new(x: i32, score: i32) -> Self {
        let mut random = RandomNumberGenerator::new();
        Obstacle {
            x,
            gap_y: random.range(10, 40),
            gap_size: i32::max(2, 20 - score),
        }
    }
    fn render(&self, ctx: &mut BTerm, player_x: i32) {
        let screen_x = self.x - player_x;
        let half_size = self.gap_size / 2;

        // Draw top of obstacle
        for y in 0..self.gap_y - half_size {
            ctx.set(screen_x, y, RED, BLACK, to_cp437('|'));
        }
        // Draw bottom of obstacle
        for y in self.gap_y + half_size..SCREEN_HEIGHT {
            ctx.set(screen_x, y, RED, BLACK, to_cp437('|'));
        }
    }
    fn lethal_interaction(&self, player: &Player) -> bool {
        if player.x != self.x {
            return false;
        };
        let half_gap_size = self.gap_size / 2;
        let player_above_gap = player.y < self.gap_y - half_gap_size;
        let player_below_gap = player.y > self.gap_y + half_gap_size;
        player_above_gap || player_below_gap
    }
}
struct State {
    player: Player,
    score: i32,
    obstacle: Obstacle,
    frame_time: f32,
    mode: GameMode,
}
impl State {
    fn new() -> Self {
        State {
            player: Player::new(),
            score: 0,
            obstacle: Obstacle::new(SCREEN_WIDTH, 0),
            frame_time: 0.0,
            mode: GameMode::Menu,
        }
    }
    fn main_menu(&mut self, ctx: &mut BTerm) {
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
    fn dead(&mut self, ctx: &mut BTerm) {
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
    fn play(&mut self, ctx: &mut BTerm) {
        ctx.cls_bg(NAVY);
        self.frame_time += ctx.frame_time_ms;
        if self.frame_time > FRAME_DURATION {
            self.frame_time = 0.0;

            self.player.gravity();
            self.player.motion();
        }
        if let Some(VirtualKeyCode::Space) = ctx.key {
            self.player.flap();
        }
        self.player.render(ctx);
        self.obstacle.render(ctx, self.player.x);
        if self.player.x > self.obstacle.x {
            self.score += 1;
            self.obstacle = Obstacle::new(self.player.x + SCREEN_WIDTH, self.score);
        }
        ctx.print(1, 0, "Press SPACE to Flap");

        if self.player.y > SCREEN_HEIGHT || self.obstacle.lethal_interaction(&self.player) {
            self.mode = GameMode::End;
        }
    }
    fn restart(&mut self) {
        self.player = Player::new();
        self.frame_time = 0.0;
        self.obstacle = Obstacle::new(self.player.x + SCREEN_WIDTH, self.score);
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

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Flappy Dragon")
        .build()?;
    main_loop(context, State::new())
}
