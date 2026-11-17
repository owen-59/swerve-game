use godot::{
    classes::{CanvasLayer, Label}, prelude::*,
};

#[derive(GodotClass)]
#[class(base=CanvasLayer, init)]
pub struct Hud {
    base: Base<CanvasLayer>,

    #[init(node = "ScoreLabel")]
    score_label: OnReady<Gd<Label>>
}

#[godot_api]
pub impl Hud {
    pub fn set_score(&mut self, score: u32) {
        let text = format!("{}", score);
        self.score_label.set_text(&text);
    }
}
