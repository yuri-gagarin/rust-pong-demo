use ggez;
use ggez::{graphics, event, nalgebra};
use ggez::{Context, ContextBuilder, GameResult};
use graphics::Rect;

const PADDLE_HEIGHT: f32 = 100.0;
const PADDLE_WIDTH: f32 = 20.0;
const PADDLE_HEIGHT_HALF: f32 = PADDLE_HEIGHT * 0.5;
const PADDLE_WIDTH_HALF: f32 = PADDLE_WIDTH * 0.5;
struct MainState {
    player_one_pos: nalgebra::Point2<f32>,
    player_two_pos: nalgebra::Point2<f32>,
}
impl MainState {
    pub fn new(context: &mut Context) -> Self {
        let (screen_width, screen_height) = graphics::drawable_size(context);
        let (screeen_width_half, screen_height_half) = (screen_width * 0.5, screen_height * 0.5);
        return MainState {
            player_one_pos: nalgebra::Point2::new(PADDLE_WIDTH_HALF, screen_height_half),
            player_two_pos: nalgebra::Point2::new(screen_width - PADDLE_WIDTH_HALF, screen_height_half),
        };
    }
}
impl event::EventHandler for MainState {
    fn update(&mut self, context: &mut Context) -> GameResult {
        Ok(())
    }
    fn draw(&mut self, context: &mut Context) -> GameResult {
        graphics::clear(context, graphics::BLACK);

        // make a paddle //
        let first_paddle: Rect = graphics::Rect::new(-PADDLE_WIDTH_HALF, -PADDLE_HEIGHT_HALF, PADDLE_WIDTH, PADDLE_HEIGHT);
        let rect_mesh = graphics::Mesh::new_rectangle(context, graphics::DrawMode::fill(), first_paddle, graphics::WHITE).expect("Video Card Error");

        let mut draw_param = graphics::DrawParam::default();
        draw_param.dest = self.player_one_pos.into();

        graphics::draw(context, &rect_mesh, draw_param).expect("Video card error");
        graphics::present(context).expect("Video Error");
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
