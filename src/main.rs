use ggez;
use ggez::nalgebra::ComplexField;
use ggez::{graphics, event, nalgebra};
use ggez::{Context, ContextBuilder, GameResult};
use ggez::input::keyboard::{self, KeyCode};
use graphics::Rect;
use rand::{thread_rng, Rng};

const PADDLE_PAD: f32 = 40.0;
const PADDLE_HEIGHT: f32 = 100.0;
const PADDLE_WIDTH: f32 = 20.0;
const PADDLE_HEIGHT_HALF: f32 = PADDLE_HEIGHT * 0.5;
const PADDLE_WIDTH_HALF: f32 = PADDLE_WIDTH * 0.5;
const BALL_SIZE: f32 = 30.0;
const BALL_SIZE_HALF: f32 = BALL_SIZE * 0.5;
const PLAYER_SPEED: f32 = 400.0;
const BALL_SPEED: f32 = 250.0;
// messages - errors //
const VIDEO_ERR_MSG: &str = "Video Card Error";

fn check_collision(value: &mut f32, low: f32, high: f32) {
    if *value < low {
        *value = low;
    } else if *value > high {
        *value = high;
    }
}

fn check_paddle_one_collision(main_state: &mut MainState) {
    if main_state.ball_pos.x - BALL_SIZE_HALF < main_state.player_one_pos.x + PADDLE_WIDTH_HALF &&
       main_state.ball_pos.x + BALL_SIZE_HALF > main_state.player_one_pos.x - PADDLE_WIDTH_HALF &&
       main_state.ball_pos.y - BALL_SIZE_HALF < main_state.player_one_pos.y + PADDLE_HEIGHT_HALF &&
       main_state.ball_pos.y + BALL_SIZE_HALF > main_state.player_one_pos.y - PADDLE_HEIGHT_HALF {
        // reverse the ball //
        main_state.ball_vector.x = main_state.ball_vector.x.abs();
    }
}

fn check_paddle_two_collision(main_state: &mut MainState) {
    if main_state.ball_pos.x - BALL_SIZE_HALF < main_state.player_two_pos.x + PADDLE_WIDTH_HALF &&
       main_state.ball_pos.x + BALL_SIZE_HALF > main_state.player_two_pos.x - PADDLE_WIDTH_HALF &&
       main_state.ball_pos.y - BALL_SIZE_HALF < main_state.player_two_pos.y + PADDLE_HEIGHT_HALF &&
       main_state.ball_pos.y + BALL_SIZE_HALF > main_state.player_two_pos.y - PADDLE_HEIGHT_HALF {
        // reverse the ball //
        main_state.ball_vector.x = -main_state.ball_vector.x.abs();
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
    player_one_score: u16,
    player_two_score: u16,
}
impl MainState {
    pub fn new(context: &mut Context) -> Self {
        let (screen_width, screen_height) = graphics::drawable_size(context);
        let (screeen_width_half, screen_height_half) = (screen_width * 0.5, screen_height * 0.5);
        // 'ball starting velocity' //
        let mut ball_vector = nalgebra::Vector2::new(10.0, 5.0);
        randomize_vector(&mut ball_vector, BALL_SPEED, BALL_SPEED);
        return MainState {
            player_one_pos: nalgebra::Point2::new(PADDLE_WIDTH_HALF + PADDLE_PAD, screen_height_half),
            player_two_pos: nalgebra::Point2::new(screen_width - PADDLE_WIDTH_HALF - PADDLE_PAD, screen_height_half),
            ball_pos: nalgebra::Point2::new(screeen_width_half, screen_height_half),
            ball_vector: ball_vector,
            player_one_score: 0,
            player_two_score: 0,
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

        // ball and ball physics //
        self.ball_pos += self.ball_vector * dt;

        if self.ball_pos.x < 0.0 {
            self.ball_pos.x = screen_width * 0.5;
            self.ball_pos.y = screen_height * 0.5;
            randomize_vector(& mut self.ball_vector, BALL_SPEED, BALL_SPEED);
            self.player_two_score += 1;
        }
        if self.ball_pos.x > screen_width {
            self.ball_pos.x = screen_width * 0.5;
            self.ball_pos.y = screen_height * 0.5;
            randomize_vector(&mut self.ball_vector, BALL_SPEED, BALL_SPEED);
            self.player_one_score += 1;
        }
        if self.ball_pos.y < BALL_SIZE_HALF {
            self.ball_pos.y = BALL_SIZE_HALF;
            self.ball_vector.y = self.ball_vector.y.abs();
        } else if self.ball_pos.y > (screen_height - BALL_SIZE_HALF) {
            self.ball_pos.y = screen_height - BALL_SIZE_HALF;
            self.ball_vector.y = -self.ball_vector.y.abs();       
        }

        check_paddle_one_collision(self);
        check_paddle_two_collision(self);

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
        // draw out the score //
        let (screen_width, screen_height) = graphics::drawable_size(context);
        let (screen_width_half, screen_height_half) = (screen_width * 0.5, screen_height * 0.5);
        let score_text = graphics::Text::new(format!("{}          {}", self.player_one_score, self.player_two_score));
        // set the inital postion //
        let mut score_position = nalgebra::Point2::new(screen_width_half, 20.0);
        // and set an offset to the X position to center the score //
        let (score_text_width, score_text_height) = score_text.dimensions(context);
        score_position -= nalgebra::Vector2::new(score_text_width as f32 * 0.5, score_text_height as f32 * 0.5);

        draw_param.dest = score_position.into();
        graphics::draw(context, &score_text, draw_param).expect(VIDEO_ERR_MSG);

        graphics::present(context).expect(VIDEO_ERR_MSG);
        Ok(())
    }
}

fn main() -> GameResult {
    let context_builder: ContextBuilder = ggez::ContextBuilder::new("Pong_1", "yuri-gagarin");  
    let (context, event_loop) = &mut context_builder.build()?;
    // 
    graphics::set_window_title(context, "PONG GAME");

    let mut main_state: MainState = MainState::new(context);
    event::run(context, event_loop, &mut main_state).expect("Game Error");
    Ok(())
}
