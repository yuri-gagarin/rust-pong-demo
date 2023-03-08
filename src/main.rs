use ggez;
use ggez::{Context, ContextBuilder, GameResult, graphics, event};

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
