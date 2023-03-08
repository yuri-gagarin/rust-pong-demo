use ggez;
use ggez::{graphics, event};
use ggez::{Context, ContextBuilder, GameResult};
use graphics::Rect;

struct MainState {

}
impl MainState {
    pub fn new() -> Self {
        return MainState {};
    }
}
impl event::EventHandler for MainState {
    fn update(&mut self, context: &mut Context) -> GameResult {
        Ok(())
    }
    fn draw(&mut self, context: &mut Context) -> GameResult {
        graphics::clear(context, graphics::BLACK);

        // make a paddle //
        let first_paddle: Rect = graphics::Rect::new(10.0, 10.0, 300.0, 150.0);
        let rect_mesh = graphics::Mesh::new_rectangle(context, graphics::DrawMode::fill(), first_paddle, graphics::WHITE).expect("Video Card Error");

        graphics::draw(context, &rect_mesh, graphics::DrawParam::default()).expect("Video card error");
        graphics::present(context).expect("Video Error");
        Ok(())
    }
}

fn main() -> GameResult {
    let context_builder: ContextBuilder = ggez::ContextBuilder::new("Snake_0", "yuri-gagarin");  
    let (context, event_loop) = &mut context_builder.build()?;
    // 
    graphics::set_window_title(context, "SNAKE GAME");

    let mut main_state: MainState = MainState::new();
    event::run(context, event_loop, &mut main_state);
    Ok(())
}
