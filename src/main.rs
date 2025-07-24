use bracket_lib::prelude::*;

const BACKGROUND_COLOUR: (u8, u8, u8) = (150, 210, 255);
const GRASS_BG: (u8, u8, u8) = (50, 130, 0);
const GRASS_FG: (u8, u8, u8) = (70, 170, 0);
const PLAYER_COLOUR: (u8, u8, u8) = (140, 100, 210);

enum GameMode {
    Menu,
    Playing,
    End,
}

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;
const FRAME_DURATION: f32 = 40.0;

struct Cloud {
	x: i32,
	real_x: f32,
	y: i32,
	speed: f32,
	colour: (u8, u8, u8),
}

impl Cloud {
    fn new(x: i32, y: i32, speed: f32, colour: (u8, u8, u8)) -> Self {
        Self {
            x,
			real_x: x as f32,
            y,
			speed,
			colour
        }
    }

    fn render(&mut self, ctx: &mut BTerm) {  
		ctx.set(self.x + 3, self.y, self.colour, BACKGROUND_COLOUR, to_cp437('_'));
        ctx.set(self.x + 1, self.y + 1, self.colour, BACKGROUND_COLOUR, to_cp437('_'));
		ctx.set(self.x + 2, self.y + 1, self.colour, BACKGROUND_COLOUR, to_cp437('('));
		ctx.set(self.x + 4, self.y + 1, self.colour, BACKGROUND_COLOUR, to_cp437(')'));
		ctx.set(self.x + 5, self.y + 1, self.colour, BACKGROUND_COLOUR, to_cp437('_'));
		ctx.set(self.x, self.y + 2, self.colour, BACKGROUND_COLOUR, to_cp437('('));
		ctx.set(self.x + 1, self.y + 2, self.colour, BACKGROUND_COLOUR, to_cp437('_'));
		ctx.set(self.x + 2, self.y + 2, self.colour, BACKGROUND_COLOUR, to_cp437('_'));
		ctx.set(self.x + 3, self.y + 2, self.colour, BACKGROUND_COLOUR, to_cp437('_'));
		ctx.set(self.x + 4, self.y + 2, self.colour, BACKGROUND_COLOUR, to_cp437('_'));
		ctx.set(self.x + 5, self.y + 2, self.colour, BACKGROUND_COLOUR, to_cp437('_'));
		ctx.set(self.x + 6, self.y + 2, self.colour, BACKGROUND_COLOUR, to_cp437('_'));
		ctx.set(self.x + 7, self.y + 2, self.colour, BACKGROUND_COLOUR, to_cp437(')'));	
    }
	
	fn update(&mut self) {
		self.real_x -= self.speed;
		self.x = self.real_x as i32;
		
		if (self.x + 20) < 0 {
			self.x = SCREEN_WIDTH;
			self.real_x = self.x as f32;
		}
	}
}

struct Obstacle {
    x: i32,
    gap_y: i32,
    size: i32,
}

impl Obstacle {
    fn new(x: i32, score: i32) -> Self {
        let mut random = RandomNumberGenerator::new();

        return Obstacle {
            x,
            gap_y: random.range(10, 40),
            size: i32::max(2, 20 - score),
        };
    }

    fn render(&mut self, ctx: &mut BTerm, player_x: i32) {
        let screen_x = self.x - player_x;
        let half_size = self.size / 2;

        for y in 0..self.gap_y - half_size {
            ctx.set(screen_x, y, RED, BACKGROUND_COLOUR, to_cp437('#'));
        }

        for y in self.gap_y + half_size..(SCREEN_HEIGHT - 1) {
            ctx.set(screen_x, y, RED, BACKGROUND_COLOUR, to_cp437('#'));
        }
    }

    fn hit_obstacle(&self, player: &Player) -> bool {
        let half_size = self.size / 2;
        let does_x_match = player.x + 1 == self.x;
        let player_above_gap = player.y < self.gap_y - half_size;
        let player_below_gap = player.y > self.gap_y + half_size;
        return does_x_match && (player_above_gap || player_below_gap);
    }
}

struct Player {
    x: i32,
    y: i32,
	last_y: i32,
    velocity: f32,
}

impl Player {
    fn new(x: i32, y: i32) -> Self {
        return Player {
            x,
            y,
			last_y: y,
            velocity: 0.0,
        };
    }

    fn render(&mut self, ctx: &mut BTerm) {
		if self.last_y <= self.y {
			ctx.set(0, self.y, PLAYER_COLOUR, BACKGROUND_COLOUR, to_cp437('\\'));
			ctx.set(1, self.y, PLAYER_COLOUR, BACKGROUND_COLOUR, to_cp437('@'));
			ctx.set(2, self.y, PLAYER_COLOUR, BACKGROUND_COLOUR, to_cp437('/'));
		} else {
			ctx.set(2, self.y, PLAYER_COLOUR, BACKGROUND_COLOUR, to_cp437('\\'));
			ctx.set(1, self.y, PLAYER_COLOUR, BACKGROUND_COLOUR, to_cp437('@'));
			ctx.set(0, self.y, PLAYER_COLOUR, BACKGROUND_COLOUR, to_cp437('/'));

		}
    }

    fn gravity_and_move(&mut self) {
        if self.velocity < 2.0 {
            self.velocity += 0.2;
        }
	
		self.last_y = self.y;
        self.y += self.velocity as i32;
        self.x += 1;

        if self.y < 0 {
            self.y = 0;
        }
    }

    fn flap(&mut self) {
        self.velocity = -2.0;
    }
}

struct State {
    player: Player,
    frame_time: f32,
    obstacle: Obstacle,
	cloud1: Cloud,
	cloud2: Cloud,
	cloud3: Cloud,
	cloud4: Cloud,
    mode: GameMode,
    score: i32,
}

impl State {
    fn new() -> Self {
        return State {
            player: Player::new(5, 25),
            frame_time: 0.0,
            obstacle: Obstacle::new(SCREEN_WIDTH, 0),
			cloud1: Cloud::new(-10, 5, 0.25, (225, 225, 225)),
			cloud2: Cloud::new(-10, 8, 0.75, (245, 245, 245)),
			cloud3: Cloud::new(-10, 11, 0.50, (235, 235, 235)),
			cloud4: Cloud::new(-10, 14, 1.00, (255, 255, 255)),
            mode: GameMode::Menu,
            score: 0,
        };
    }

    fn play(&mut self, ctx: &mut BTerm) {
        ctx.cls_bg(BACKGROUND_COLOUR);
		
		for x in 0..SCREEN_WIDTH {
			ctx.set(x, SCREEN_HEIGHT - 1, GRASS_FG, GRASS_BG, to_cp437('#'));
		}
		
        self.frame_time += ctx.frame_time_ms;

        if self.frame_time > FRAME_DURATION {
            self.frame_time = 0.0;
            self.player.gravity_and_move();
			self.cloud1.update();
			self.cloud2.update();
			self.cloud3.update();
			self.cloud4.update();
        }

        if let Some(VirtualKeyCode::Space) = ctx.key {
            self.player.flap();
        }

        if let Some(VirtualKeyCode::Q) = ctx.key {
            self.mode = GameMode::End;
        }
		
		self.cloud1.render(ctx);
		self.cloud2.render(ctx);
		self.cloud3.render(ctx);
		self.cloud4.render(ctx);
        self.player.render(ctx);
        ctx.print_color(0, 0, PLAYER_COLOUR, BACKGROUND_COLOUR, "Press SPACE to flap.");
        ctx.print_color(0, 1, PLAYER_COLOUR, BACKGROUND_COLOUR, &format!("Score: {}", self.score));
        self.obstacle.render(ctx, self.player.x);

        if self.player.x > self.obstacle.x {
            self.score += 1;
            self.obstacle = Obstacle::new(self.player.x + SCREEN_WIDTH, self.score);
        }

        if self.player.y > (SCREEN_HEIGHT - 1) || self.obstacle.hit_obstacle(&self.player) {
            self.mode = GameMode::End;
        }
    }

    fn restart(&mut self) {
        self.player = Player::new(5, 25);
        self.frame_time = 0.0;
        self.obstacle = Obstacle::new(SCREEN_WIDTH, 0);
        self.mode = GameMode::Playing;
        self.score = 0;
    }

    fn main_menu(&mut self, ctx: &mut BTerm) {
        ctx.cls_bg(BACKGROUND_COLOUR);
        ctx.print_color_centered(5, PLAYER_COLOUR, BACKGROUND_COLOUR, "Welcome to Flappy Dragon");
        ctx.print_color_centered(8, PLAYER_COLOUR, BACKGROUND_COLOUR, "(P) Play Game");
        ctx.print_color_centered(9, PLAYER_COLOUR, BACKGROUND_COLOUR, "(Q) Quit Game");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }

    fn dead(&mut self, ctx: &mut BTerm) {
        ctx.cls_bg(BACKGROUND_COLOUR);
        ctx.print_color_centered(5, PLAYER_COLOUR, BACKGROUND_COLOUR, "You are dead!");
        ctx.print_color_centered(6, PLAYER_COLOUR, BACKGROUND_COLOUR, &format!("You earned {} points", self.score));
        ctx.print_color_centered(8, PLAYER_COLOUR, BACKGROUND_COLOUR, "(P) Play Game");
        ctx.print_color_centered(9, PLAYER_COLOUR, BACKGROUND_COLOUR, "(Q) Quit Game");

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
        .with_fitscreen(true)
		.with_vsync(true)
        .build()?;

    return main_loop(context, State::new());
}
