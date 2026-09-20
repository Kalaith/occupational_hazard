//! Presentation derived from the guild. No rewards or availability live here.
use crate::{data::TextCatalog, simulation::Guild};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Room {
    Common,
    Assignments,
    Recovery,
    Records,
    Gate,
    Training,
}

impl Room {
    pub const ALL: [Self; 6] = [
        Self::Common,
        Self::Assignments,
        Self::Recovery,
        Self::Records,
        Self::Gate,
        Self::Training,
    ];
    pub fn name(self, text: &TextCatalog) -> &str {
        match self {
            Self::Common => text.get("room.common"),
            Self::Assignments => text.get("room.assignments"),
            Self::Recovery => text.get("room.recovery"),
            Self::Records => text.get("room.records"),
            Self::Gate => text.get("room.departure"),
            Self::Training => text.get("room.training"),
        }
    }
    pub fn center(self) -> (f32, f32) {
        match self {
            Self::Common => (330., 650.),
            Self::Assignments => (775., 650.),
            Self::Recovery => (330., 280.),
            Self::Records => (775., 280.),
            Self::Gate => (1130., 650.),
            Self::Training => (1400., 650.),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Activity {
    Away,
    Recovering,
    Resting,
    Training,
    Ready,
}

pub fn activity(g: &Guild, id: usize) -> Activity {
    let a = &g.roster[id];
    if g.busy(id) {
        Activity::Away
    } else if a.injury > 0 {
        Activity::Recovering
    } else if a.fatigue > 0 {
        Activity::Resting
    } else if g.services.training_yard && !a.bronze && a.xp < g.config.progression.training_xp_cap {
        Activity::Training
    } else {
        Activity::Ready
    }
}

impl Activity {
    pub fn label(self, text: &TextCatalog) -> &str {
        match self {
            Self::Away => text.get("activity.away"),
            Self::Recovering => text.get("activity.medical"),
            Self::Resting => text.get("activity.resting"),
            Self::Training => text.get("activity.training"),
            Self::Ready => text.get("activity.ready"),
        }
    }
    pub fn room(self) -> Option<Room> {
        match self {
            Self::Away => None,
            Self::Recovering | Self::Resting => Some(Room::Recovery),
            Self::Training => Some(Room::Training),
            Self::Ready => Some(Room::Common),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sheet {
    None,
    Commissions,
    Rooms,
    Jobs,
    Career,
    Returns,
    Facility(Room),
}

pub struct Headquarters {
    pub sheet: Sheet,
    pub focus: Option<Room>,
    pub reduced_motion: bool,
    pub large_text: bool,
    pub readiness_details: bool,
    pub page: usize,
    pub journey: Option<usize>,
    pub staff_open: bool,
    pub journey_page: usize,
    pub return_section: usize,
    pub transition: Option<Transition>,
    pub pointer_start: Option<macroquad::prelude::Vec2>,
    pub dragged: bool,
    pub(crate) day_input_cooldown: f32,
}

pub struct Transition {
    pub people: Vec<usize>,
    pub arriving: bool,
    pub elapsed: f32,
}

impl Default for Headquarters {
    fn default() -> Self {
        Self {
            sheet: Sheet::None,
            focus: None,
            reduced_motion: false,
            large_text: false,
            readiness_details: false,
            page: 0,
            journey: None,
            staff_open: false,
            journey_page: 0,
            return_section: 0,
            transition: None,
            pointer_start: None,
            dragged: false,
            day_input_cooldown: 0.,
        }
    }
}

impl Headquarters {
    pub fn tick(&mut self, dt: f32) {
        self.day_input_cooldown = (self.day_input_cooldown - dt).max(0.);
        if let Some(t) = &mut self.transition {
            t.elapsed += dt;
            if t.elapsed >= 1.2 || self.reduced_motion {
                self.transition = None;
            }
        }
    }
    /// Coalesce rapid duplicate taps while the date and return surface change.
    pub fn begin_day_change(&mut self) -> bool {
        if self.day_input_cooldown > 0. {
            return false;
        }
        self.day_input_cooldown = 0.25;
        true
    }
    pub fn open(&mut self, sheet: Sheet) {
        self.sheet = sheet;
        self.page = 0;
        self.journey = None;
    }
}
