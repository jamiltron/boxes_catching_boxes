extern crate sdl2_ttf;

use sdl2::rect::Rect;
use sdl2::render::Renderer;
use sdl2::render::Texture;
use sdl2::render::TextureQuery;
use sdl2::pixels::Color;
use sdl2_ttf::Font;

const MIN_SCORE: i32 = 0;
const MAX_SCORE: i32 = 999999;

pub struct ScoreKeeper {
    pub score: i32,
    old_score: i32,
    font: Font,
    font_texture: Texture,
    font_target: Rect,
    text_color: Color,
    game_width: u32,
}

impl ScoreKeeper {
    fn create_font_texture(font: &Font,
                           text_color: Color,
                           text: String,
                           renderer: &Renderer)
                           -> Texture {
        let font_surface = font.render(text.as_str())
            .solid(text_color)
            .unwrap();

        renderer.create_texture_from_surface(&font_surface).unwrap()
    }

    fn create_font_target(game_width: u32, font_texture: &Texture) -> Rect {
        let TextureQuery { width, height, .. } = font_texture.query();
        Rect::new((game_width - (width + 12)) as i32,
                  (height - 12) as i32,
                  width,
                  height)
    }

    pub fn new(font: Font, text_color: Color, renderer: &Renderer, game_width: u32) -> Self {
        let text = format!("Score: {:06}", 0);
        let font_texture = ScoreKeeper::create_font_texture(&font, text_color, text, renderer);
        let font_target = ScoreKeeper::create_font_target(game_width, &font_texture);


        ScoreKeeper {
            score: 0,
            old_score: 0,
            font: font,
            text_color: text_color,
            font_target: font_target,
            font_texture: font_texture,
            game_width: game_width,
        }
    }

    pub fn update(&mut self, points: i32) -> () {
        self.score += points;

        if self.score > MAX_SCORE {
            self.score = MAX_SCORE;
        }

        if self.score < MIN_SCORE {
            self.score = MIN_SCORE;
        }
    }

    pub fn render(&mut self, renderer: &mut Renderer) -> () {
        if self.old_score != self.score {
            self.old_score = self.score;

            let font_surface = self.font
                .render(format!("Score: {:06}", self.score).as_str())
                .solid(self.text_color)
                .unwrap();

            self.font_texture = renderer.create_texture_from_surface(&font_surface).unwrap();
            let TextureQuery { width, height, .. } = self.font_texture.query();
            self.font_target = Rect::new((self.game_width - (width + 12)) as i32,
                                         (height - 12) as i32,
                                         width,
                                         height);
        }

        renderer.set_draw_color(self.text_color);
        renderer.copy(&mut self.font_texture, None, Some(self.font_target));
    }
}
