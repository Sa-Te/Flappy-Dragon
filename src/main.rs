use bracket_lib::prelude::*;
use rand::Rng; // Import the Rng trait

enum GameMode {
    Menu,
    Playing,
    End,
}

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;
const FRAME_DURATION: f32 = 100.0;

struct Player {
    x: i32,
    y: i32,
    velocity: f32,
}

impl Player {
    fn new(x: i32, y: i32) -> Self {
        Player {
            x,
            y,
            velocity: 0.0,
        }
    }

    fn render(&mut self, ctx: &mut BTerm) {
        ctx.set(0, self.y, YELLOW, BLACK, to_cp437('@'));
    }

    fn gravity_and_move(&mut self) {
        if self.velocity < 2.0 {
            self.velocity += 0.8;
        }

        self.y += self.velocity as i32;
        self.x += 1;

        if self.y < 0 {
            self.y = 0;
        }
    }

    fn flap(&mut self) {
        self.velocity = -2.7;
    }
}

struct Obstacle {
    x: i32,
    gap_y: i32,
    size: i32,
    x_velocity: f32,
}

impl Obstacle {
    fn new(x: i32, score: i32) -> Self {
        let mut rng = rand::thread_rng();
        let gap_y = rng.gen_range(10..40);
        let size = i32::max(2, 20 - score / 5); // Reduce size more slowly
        let x_velocity = -rng.gen_range(1.0..3.0) - (score as f32 * 0.1); //increase velocity over time.

        Obstacle {
            x,
            gap_y,
            size,
            x_velocity,
        }
    }

    fn render(&mut self, ctx: &mut BTerm, player_x: i32) {
        let screen_x = self.x - player_x;
        let half_size = self.size / 2;

        if screen_x < -1 {
            // Only render if on screen.
            return;
        }

        // Draw the top of the obstacle
        for y in 0..self.gap_y - half_size {
            ctx.set(screen_x, y, RED, BLACK, to_cp437('|'));
        }

        // Draw the bottom half of the obstacle
        for y in self.gap_y + half_size..SCREEN_HEIGHT {
            ctx.set(screen_x, y, RED, BLACK, to_cp437('|'));
        }
    }

    fn hit_obstacle(&self, player: &Player) -> bool {
        let half_size = self.size / 2;
        let does_x_match = (player.x - self.x).abs() < 2; // More forgiving hitbox

        let player_above_gap = player.y < self.gap_y - half_size;
        let player_below_gap = player.y > self.gap_y + half_size;

        does_x_match && (player_above_gap || player_below_gap)
    }

    fn update(&mut self) {
        self.x += self.x_velocity as i32;
    }
}

struct State {
    player: Player,
    frame_time: f32,
    obstacles: Vec<Obstacle>, // A list of obstacles
    score: i32,
    mode: GameMode,
    obstacle_timer: f32,    // Timer for obstacle generation
    obstacle_interval: f32, // Time between obstacle generations
}

impl State {
    fn new() -> Self {
        State {
            player: Player::new(5, 25),
            frame_time: 0.0,
            obstacles: Vec::new(),
            score: 0,
            mode: GameMode::Menu,
            obstacle_interval: 200.0, // Initial interval
            obstacle_timer: 0.0,
        }
    }

    fn play(&mut self, ctx: &mut BTerm) {
        ctx.cls_bg(NAVY);
        self.frame_time += ctx.frame_time_ms;
        self.obstacle_timer += ctx.frame_time_ms;

        if self.frame_time > FRAME_DURATION {
            self.frame_time = 0.0;
            self.player.gravity_and_move();
        }

        if let Some(VirtualKeyCode::Space) = ctx.key {
            self.player.flap();
        }
        self.player.render(ctx);
        ctx.print(0, 0, "Press SPACE to flap.");
        ctx.print(0, 1, &format!("Score: {}", self.score));

        // Obstacle generation
        if self.obstacle_timer > self.obstacle_interval {
            let last_obstacle_x = self.obstacles.last().map_or(self.player.x, |o| o.x);
            if last_obstacle_x < self.player.x + SCREEN_WIDTH / 2 {
                self.obstacles
                    .push(Obstacle::new(self.player.x + SCREEN_WIDTH, self.score));
            }
            self.obstacles
                .push(Obstacle::new(last_obstacle_x + SCREEN_WIDTH, self.score));
            self.obstacle_timer = 0.0;
            self.obstacle_interval = f32::max(150.0, self.obstacle_interval - 1.0);
        }

        // Render and update obstacles
        let player_x = self.player.x;
        self.obstacles
            .retain(|obstacle| obstacle.x > player_x - SCREEN_WIDTH);
        for obstacle in &mut self.obstacles {
            obstacle.render(ctx, player_x);
            obstacle.update();

            if obstacle.hit_obstacle(&self.player) || self.player.y > SCREEN_HEIGHT {
                self.mode = GameMode::End;
            }
        }

        // Score update
        if let Some(first_obstacle) = self.obstacles.first() {
            if self.player.x > first_obstacle.x + first_obstacle.size / 2 {
                self.score += 1;
                self.obstacles.remove(0);
            }
        }
    }

    fn restart(&mut self) {
        self.player = Player::new(5, 25);
        self.frame_time = 0.0;
        self.obstacles.clear();
        self.mode = GameMode::Playing;
        self.score = 0;
        self.obstacle_interval = 200.0;
        self.obstacle_timer = 200.0;
    }

    fn main_menu(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "Welcome to Flappy Dragon");
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
        ctx.print_centered(5, "You are Dead");
        ctx.print_centered(6, &format!("You earned {} points", self.score));
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
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.mode {
            GameMode::Menu => self.main_menu(ctx),
            GameMode::End => self.dead(ctx),
            GameMode::Playing => self.play(ctx),
        }
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Flappy Dragon")
        .build()?;

    main_loop(context, State::new())
}
