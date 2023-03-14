use ggez;
use ggez::{graphics, event, nalgebra};
use ggez::{Context, ContextBuilder, GameResult};
use ggez::input::keyboard::{self, KeyCode};
use graphics::Rect;
use rand::{thread_rng, Rng};

const PADDLE_HEIGHT: f32 = 100.0;
const PADDLE_WIDTH: f32 = 20.0;
const PADDLE_HEIGHT_HALF: f32 = PADDLE_HEIGHT * 0.5;
const PADDLE_WIDTH_HALF: f32 = PADDLE_WIDTH * 0.5;
const BALL_SIZE: f32 = 30.0;
const BALL_SIZE_HALF: f32 = BALL_SIZE * 0.5;
const PLAYER_SPEED: f32 = 400.0;
const BALL_SPEED: f32 = 500.0;
// messages - errors //
const VIDEO_ERR_MSG: &str = "Video Card Error";

fn check_collision(value: &mut f32, low: f32, high: f32) {
    if *value < low {
        *value = low;
    } else if *value > high {
        *value = high;
    }
}

fn move_paddle(pos: &mut nalgebra::Point2<f32>, keycode: KeyCode, y_dir: f32, context: &mut Context) {
    let dt: f32 = ggez::timer::delta(context).as_secs_f32();
    let screen_height = graphics::drawable_size(context).1;

    if keyboard::is_key_pressed(context, keycode) {
        pos.y += y_dir * PLAYER_SPEED * dt;
    }
    check_collision(&mut pos.y, PADDLE_HEIGHT_HALF, screen_height - PADDLE_HEIGHT_HALF);

}

fn randomize_vector(vector: &mut nalgebra::Vector2<f32>, x: f32, y: f32) {
    let mut rng = thread_rng();

    vector.x = match rng.gen_bool(0.5) {
        true => x,
        false => -x,
    };
    vector.y = match rng.gen_bool(0.5) {
        true => y,
        false => -y
    };
}
struct MainState {
    player_one_pos: nalgebra::Point2<f32>,
    player_two_pos: nalgebra::Point2<f32>,
    ball_pos: nalgebra::Point2<f32>,
    ball_vector: nalgebra::Vector2<f32>,
}
impl MainState {
    pub fn new(context: &mut Context) -> Self {
        let (screen_width, screen_height) = graphics::drawable_size(context);
        let (screeen_width_half, screen_height_half) = (screen_width * 0.5, screen_height * 0.5);
        // 'ball starting velocity' //
        let mut ball_vector = nalgebra::Vector2::new(0.0, 0.0);
        randomize_vector(&mut ball_vector, BALL_SPEED, BALL_SPEED);
        return MainState {
            player_one_pos: nalgebra::Point2::new(PADDLE_WIDTH_HALF, screen_height_half),
            player_two_pos: nalgebra::Point2::new(screen_width - PADDLE_WIDTH_HALF, screen_height_half),
            ball_pos: nalgebra::Point2::new(screeen_width_half, screen_height_half),
            ball_vector: ball_vector
        };
    }
}
impl event::EventHandler for MainState {
    fn update(&mut self, context: &mut Context) -> GameResult {
        // dt to make paddles - 'ball' move smoothly //
        let dt = ggez::timer::delta(context).as_secs_f32();
        let (screen_width, screen_height) = graphics::drawable_size(context);
        // paddle movement player one //
        move_paddle(&mut self.player_one_pos, KeyCode::W, -1.0, context);
        move_paddle(&mut self.player_one_pos, KeyCode::S,  1.0, context);
        // paddle movement player two //
        move_paddle(&mut self.player_two_pos, KeyCode::Up, -1.0, context);
        move_paddle(&mut self.player_two_pos, KeyCode::Down, 1.0, context);
        Ok(())
    }
    fn draw(&mut self, context: &mut Context) -> GameResult {
        graphics::clear(context, graphics::BLACK);

        // make a paddle //
        let first_paddle: Rect = graphics::Rect::new(-PADDLE_WIDTH_HALF, -PADDLE_HEIGHT_HALF, PADDLE_WIDTH, PADDLE_HEIGHT);
        let rect_mesh = graphics::Mesh::new_rectangle(context, graphics::DrawMode::fill(), first_paddle, graphics::WHITE).expect("Video Card Error");

        let ball_rect = graphics::Rect::new(-BALL_SIZE_HALF, -BALL_SIZE_HALF, BALL_SIZE, BALL_SIZE);
        let ball_mesh = graphics::Mesh::new_rectangle(context, graphics::DrawMode::fill(), ball_rect, graphics::WHITE).expect("Video Error");

        let mut draw_param = graphics::DrawParam::default();

        draw_param.dest = self.player_one_pos.into();
        graphics::draw(context, &rect_mesh, draw_param).expect(VIDEO_ERR_MSG);

        draw_param.dest = self.player_two_pos.into();
        graphics::draw(context, &rect_mesh, draw_param).expect(VIDEO_ERR_MSG);

        draw_param.dest = self.ball_pos.into();
        graphics::draw(context, &ball_mesh, draw_param).expect(VIDEO_ERR_MSG);

        graphics::present(context).expect(VIDEO_ERR_MSG);
        Ok(())
    }
}

fn main() -> GameResult {
    let context_builder: ContextBuilder = ggez::ContextBuilder::new("Snake_0", "yuri-gagarin");  
    let (context, event_loop) = &mut context_builder.build()?;
    // 
    graphics::set_window_title(context, "SNAKE GAME");

    let mut main_state: MainState = MainState::new(context);
    event::run(context, event_loop, &mut main_state);
    Ok(())
}
