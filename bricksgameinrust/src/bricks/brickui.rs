use sdl2::Audiosubsystem;
use sdl2::Sdl;
use sdl2::render::Renderer;
use sdl2_mixer::Music;
use sdl2_ttf::Sdl2TtfContext;
use std::rc::Rc;

/// Interface to get user input, play music, draw pixels
pub struct BrickUi {
    pub sdl_ctx: Sdl,
    pub renderer: Renderer<'static>,
    pub ttf_ctx: Sdl2TtfContext,
    pub sdl_audio: AudioSubsystem,
    pub paddle_hit_sound: Rc<Music>,
    pub brick_knock_sound: Rc<Music>
}

impl BrickUi {
    pub fn new(sdl_ctx: Sdl,
               renderer: Renderer<'static>,
               ttf_ctx: Sdl2TtfContext,
               sdl_audio: AudioSystem,
               paddle_hit_sound: Music,
               brick_knock_sound: Music) -> BrickUi {

        return BrickUi {
            sdl_ctx,
            renderer,
            ttf_ctx,
            sdl_audio,
            paddle_hit_sound: Rc::new(paddle_hit_sound),
            brick_knock_sound: Rc::new(brick_knock_sound)
        };
    }

    //poll for user event
    pub fn poll_event(&self) -> Option<Event> {
        return self.sdl_ctx.event_pump().unwrap().poll_event();
    }
}

/// Traits for types for which drawing can be done on the screen
pub trait Drawable {
    fn draw(&self, bricksui: &mut BrickUi);
}
