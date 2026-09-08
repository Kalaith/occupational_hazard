use super::*;

pub fn career(g: &Game, r: Rect) -> Option<UiAction> {
    dossier::draw_dossier(g, r)
}

pub fn facility(g: &Game, r: Rect, room: Room) -> Option<UiAction> {
    let recovery = room == Room::Recovery;
    let purchased = if recovery { g.guild.services.infirmary } else { g.guild.services.training_yard };
    let cost = if recovery { crate::services::INFIRMARY_COST } else { crate::services::TRAINING_COST };
    text(if recovery { "RECOVERY ROOM" } else { "TRAINING YARD" }, Rect::new(r.x, r.y, r.w, 40.), 26., INK);
    let body = if recovery {
        "Basic rest is always available. People at home recover fatigue and medical leave when you advance a day.\n\nThe infirmary adds medical equipment and doubles medical-leave recovery. Beds are scenery, with no capacity limit."
    } else {
        "The unequipped courtyard grants no training.\n\nEquip the yard for fully rested Iron recruits to gain 5 XP per day at home, up to 60 XP. Training never awards successful-contract credit."
    };
    text(body, Rect::new(r.x, r.y + 56., r.w, r.h - 200.), 21., INK);
    let people = g.guild.roster.iter().enumerate().filter(|(id, _)| activity(&g.guild, *id).room() == Some(room)).map(|(_, a)| a.name.as_str()).collect::<Vec<_>>().join(", ");
    text(&format!("Here now: {}", if people.is_empty() { "No staff" } else { &people }), Rect::new(r.x, r.bottom() - 134., r.w, 65.), 18., MUTED);
    if primary(Rect::new(r.x, r.bottom() - 50., r.w, 48.), &if purchased { "FACILITY OPEN".into() } else { format!("{} · {}g", if g.guild.gold >= cost { "PURCHASE" } else { "NEED" }, cost) }, !purchased && g.guild.gold >= cost) { return Some(UiAction::Purchase(if recovery { crate::services::Purchase::Infirmary } else { crate::services::Purchase::TrainingYard })); }
    None
}
