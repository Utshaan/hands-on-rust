use bracket_lib::prelude::*;

struct State {
    message: &'static str,
    message_len: usize,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print(40 - self.message_len / 2, 25, self.message);
    }
}

fn main() -> BError {
    let message = "Hello, Bracket Terminal!";
    let message_len = message.len();
    let context = BTermBuilder::simple80x50()
        .with_title("Flappy dragon")
        .build()?;

    main_loop(
        context,
        State {
            message,
            message_len,
        },
    )
}
