//! Purchases and their effects, readable without a keyboard or scrolling.
use super::*;
use crate::services::{Purchase, INFIRMARY_COST, SCOUT_COST, TRAINING_COST};

pub fn draw_services(g: &Game, r: Rect) -> Option<UiAction> {
    panel(r, PAPER);
    let x = r.x + 12.0;
    let w = r.w - 24.0;
    let row_h = (r.h - 16.0) / 3.0;
    let q = &g.contracts[g.selected];
    let entries = [
        ("Infirmary (optional)", "Medical leave clears twice as fast at home: 2 days per day instead of 1. Fatigue still needs rest.".to_owned(),
            INFIRMARY_COST, g.guild.services.infirmary, Purchase::Infirmary),
        ("Training yard (optional)", "Fully rested Iron recruits at the guild gain 5 XP daily, up to 60. Three quest successes are still required.".to_owned(),
            TRAINING_COST, g.guild.services.training_yard, Purchase::TrainingYard),
        ("Scout route (optional)", format!("{}: prepare the next dispatch. Choose another route in CONTRACTS. Trials forbid scouting.", q.title),
            SCOUT_COST, g.guild.services.scouted.contains(&g.selected), Purchase::Scout(g.selected)),
    ];
    for (i, (title, description, cost, owned, purchase)) in entries.into_iter().enumerate() {
        let y = r.y + 8.0 + i as f32 * row_h;
        label(title, Rect::new(x, y, w, 25.0), 24.0, INK);
        paragraph(
            &description,
            Rect::new(x, y + 27.0, w, row_h - 78.0),
            20.0,
            INK,
        );
        let text = if owned {
            "READY".to_owned()
        } else {
            format!("BUY / {cost}g")
        };
        if button(Rect::new(x, y + row_h - 48.0, w, 44.0), &text, owned) && !owned {
            return Some(UiAction::Purchase(purchase));
        }
    }
    None
}
