use macroquad::prelude::*;
use crate::rand::rand;

const PLAYER_SIZE: Vec2 = Vec2::from_array([30f32, 100f32]);
const PLAYER_SPEED: f32 = 400f32;
const BALL_SIZE: Vec2 = Vec2::from_array([25f32, 25f32]);
const BALL_SPEED: f32 = 300f32;

enum GameState {
    Menu,
    Ingame,
    End,
}

struct Player {
    rect: Rect,
    is_player_one: bool,
    score: u32,
}

impl Player {
    pub fn new(isplayerone: bool) -> Self {
        Self {
            rect: Rect::new(
                if isplayerone {50f32} else {screen_width() - 50f32},
                screen_height() * 0.5f32 - PLAYER_SIZE.y * 0.5f32,
                PLAYER_SIZE.x,
                PLAYER_SIZE.y,
            ),
            is_player_one: isplayerone,
            score: 0,
        }
    }
    pub fn draw(&self) {
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, WHITE);
    }
    pub fn update(&mut self, dt: f32) {
        let y_move = match (
            is_key_down(if self.is_player_one {KeyCode::W} else {KeyCode::Up}),
            is_key_down(if self.is_player_one {KeyCode::S} else {KeyCode::Down})
        ) {
            (true, false) => -1f32,
            (false, true) => 1f32,
            _ => 0f32,
        };
        self.rect.y += y_move * dt * PLAYER_SPEED;

        if self.rect.y < 0f32 {
            self.rect.y = 0f32;
        }
        if self.rect.y > screen_height() - self.rect.h {
            self.rect.y = screen_height() - self.rect.h;
        }
    }
}

struct Ball {
    rect: Rect,
    vel: Vec2,
}

impl Ball {
    pub fn new(player_one_scored: bool) -> Self {
        let coin = rand() % 2u32 == 0;
        Self {
            rect: Rect::new(screen_width() * 0.5f32, screen_height() * 0.5f32,BALL_SIZE.x, BALL_SIZE.y),
            vel: Vec2 {x: if player_one_scored {1f32} else {-1f32}, y: if coin {1f32} else {-1f32}}.normalize(),
        }
    }
    pub fn draw(&self) {
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, WHITE);
    }
    pub fn update(&mut self, dt: f32, player1: &mut Player, player2: &mut Player) {
        let coin = rand() % 2u32 == 0;
        let dev = rand::gen_range(0.5f32, 2f32);

        self.rect.x += self.vel.x * dt * BALL_SPEED;
        self.rect.y += self.vel.y * dt * BALL_SPEED;
        match (self.rect.x > screen_width() - self.rect.w, self.rect.x < 0f32) {
            (true, _) => {
                self.vel = Vec2 {x: 1f32, y: if coin {dev} else {-dev}}.normalize();
                self.rect = Rect::new(screen_width() * 0.5f32, screen_height() * 0.5f32,BALL_SIZE.x, BALL_SIZE.y);
                player1.score += 1;
            },
            (_, true) => {
                self.vel = Vec2 {x: -1f32, y: if coin {dev} else {-dev}}.normalize();
                self.rect = Rect::new(screen_width() * 0.5f32, screen_height() * 0.5f32,BALL_SIZE.x, BALL_SIZE.y);
                player2.score += 1;
            },
            (_, _) => {},
        };
        match (self.rect.y > screen_height() - self.rect.h, self.rect.y < 0f32) {
            (true, _) => {self.vel.y = -1f32;},
            (_, true) => {self.vel.y = 1f32;},
            (_, _) => {},
        };
    }
}

fn resolve_collision(a: &mut Rect, vel: &mut Vec2, b: &Rect) {
    let intersection = match a.intersect(*b) {
        Some(intersection) => intersection,
        None => return,
    };
    let a_center = a.center();
    let b_center = b.center();
    let to = b_center - a_center;
    let to_signum = to.signum();
    match intersection.w > intersection.h {
        true => {
            a.y -= to_signum.y * intersection.h;
            match to_signum.y > 0f32 {
                true => vel.y = -vel.y.abs(),
                false => vel.y = vel.y.abs(),
            }
        }
        false => {
            a.x -= to_signum.x * intersection.w;
            match to_signum.x < 0f32 {
                true => vel.x = vel.x.abs(),
                false => vel.x = -vel.x.abs(),
            }
        }
    }
}

#[macroquad::main("pong")]
async fn main() {
    let mut game_state = GameState::Menu;
    let font = load_ttf_font("./font_folder/PressStart2P-Regular.ttf").await.unwrap();
    let mut player1 = Player::new(true);
    let mut player2 = Player::new(false);
    let mut ball: Ball = Ball::new(true);

    loop {
        clear_background(BLACK);
        match game_state {
            GameState::Menu => {
                let menu_text = format!("Press SPACE to play!");
                let menu_text_dim = measure_text(&menu_text, Some(font), 40, 1.0f32);
                draw_text_ex(
                    &menu_text, 
                    screen_width() * 0.5f32 - menu_text_dim.width * 0.5f32,
                    screen_height() * 0.5f32 - menu_text_dim.height * 0.5f32,
                    TextParams { font: font, font_size: 40, font_scale: 1.0f32, color: WHITE, ..Default::default() }
                );
                if is_key_pressed(KeyCode::Space) {
                    game_state = GameState::Ingame;
                }
            },
            GameState::Ingame => {
                let score_text = format!("{}    {}", player1.score, player2.score);
                let score_text_dim = measure_text(&score_text, Some(font), 40, 1.0);

                draw_text_ex(
                   &score_text,
                    screen_width() * 0.5f32 - score_text_dim.width * 0.5f32,
                    50f32, 
                   TextParams { 
                        font: font, 
                        font_size: 40, 
                        font_scale: 1.0, 
                        color: WHITE,
                        ..Default::default()
                    }
                );

                draw_line(screen_width() * 0.5f32, 0f32, screen_width() * 0.5f32, screen_height(), 5f32, WHITE);


                player1.update(get_frame_time());
                player1.draw();
                player2.update(get_frame_time());
                player2.draw();
                ball.update(get_frame_time(), &mut player1, &mut player2);
                ball.draw();
                resolve_collision(&mut ball.rect, &mut ball.vel, &mut player1.rect);
                resolve_collision(&mut ball.rect, &mut ball.vel, &mut player2.rect);

                if player1.score == 10 || player2.score == 10 {
                    game_state = GameState::End;
                };
            },
            GameState::End => {
                let end_text = if player1.score > player2.score {format!("Player 1 wins!")} else {format!("Player 2 wins!")};
                let end_text_dim = measure_text(&end_text, Some(font), 40, 1.0f32);
                draw_text_ex(
                    &end_text, 
                    screen_width() * 0.5f32 - end_text_dim.width * 0.5f32,
                    screen_height() * 0.5f32 - end_text_dim.height * 0.5f32,
                    TextParams { font: font, font_size: 40, font_scale: 1.0f32, color: WHITE, ..Default::default() }
                );
                if is_key_pressed(KeyCode::Space) {
                    game_state = GameState::Menu;
                }
            }
        };
        next_frame().await
    }
}