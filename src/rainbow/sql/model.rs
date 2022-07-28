use crate::sql::Id;

pub struct Player {
    id: Option<Id>,
    uplay: Option<Uplay>,
}

impl Player {

}

pub type Uplay = String;
