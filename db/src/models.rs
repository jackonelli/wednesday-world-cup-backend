use crate::schema::{games, groups, teams};
use serde::Serialize;
use std::convert::TryFrom;
use wwc_core::group::game::{GroupGameId, PlayedGroupGame, UnplayedGroupGame};
use wwc_core::group::GroupId;
use wwc_core::team::{FifaCode, Iso2, Rank, TeamId, TeamName};

#[derive(Debug, Serialize, Queryable, Identifiable)]
pub struct Team {
    pub id: i32,
    pub name: String,
    pub fifa_code: String,
    pub iso2: String,
    pub rank_: i32,
}

impl Into<wwc_core::Team> for Team {
    fn into(self) -> wwc_core::Team {
        let id = TeamId(u8::try_from(self.id).unwrap());
        let name = TeamName::from(self.name);
        let fifa_code = FifaCode::from(self.fifa_code);
        let iso2 = Iso2::from(self.iso2);
        let rank = Rank(u8::try_from(self.rank_).unwrap());
        wwc_core::Team {
            id,
            name,
            fifa_code,
            iso2,
            rank,
        }
    }
}

#[derive(Insertable)]
#[table_name = "teams"]
pub struct NewTeam<'a> {
    pub id: i32,
    pub name: &'a str,
    pub fifa_code: &'a str,
    pub iso2: &'a str,
    pub rank_: i32,
}

#[derive(Debug, Serialize, Queryable, Associations, Identifiable)]
#[belongs_to(parent = "Game", foreign_key = "game_id")]
pub struct Group {
    pub unik: String,
    pub id: char,
    pub game_id: i32,
}

#[derive(Insertable)]
#[table_name = "groups"]
pub struct NewGroup<'a> {
    pub unik: &'a str,
    pub id: &'a str,
    pub game_id: i32,
}

#[derive(Debug, Serialize, Queryable, Associations, Identifiable)]
#[belongs_to(parent = "Team", foreign_key = "id")]
pub struct Game {
    pub id: i32,
    pub type_: String,
    pub home_team: i32,
    pub away_team: i32,
    pub home_result: Option<i32>,
    pub away_result: Option<i32>,
    pub home_penalty: Option<i32>,
    pub away_penalty: Option<i32>,
    pub home_fair_play: Option<i32>,
    pub away_fair_play: Option<i32>,
    pub played: bool,
}

#[derive(Insertable)]
#[table_name = "games"]
pub struct NewGame<'a> {
    pub id: i32,
    pub type_: &'a str,
    pub home_team: i32,
    pub away_team: i32,
    pub home_result: Option<i32>,
    pub away_result: Option<i32>,
    pub home_penalty: Option<i32>,
    pub away_penalty: Option<i32>,
    pub home_fair_play: Option<i32>,
    pub away_fair_play: Option<i32>,
    pub played: bool,
}

impl<'a> From<&'a UnplayedGroupGame> for NewGame<'a> {
    fn from(game: &'a UnplayedGroupGame) -> Self {
        NewGame {
            id: u8::from(game.id).into(),
            type_: "group",
            home_team: u8::from(game.home).into(),
            away_team: u8::from(game.away).into(),
            home_result: None,
            away_result: None,
            home_penalty: None,
            away_penalty: None,
            home_fair_play: None,
            away_fair_play: None,
            played: false,
        }
    }
}

impl<'a> From<&'a PlayedGroupGame> for NewGame<'a> {
    fn from(game: &'a PlayedGroupGame) -> Self {
        NewGame {
            id: u8::from(game.id).into(),
            type_: "group",
            home_team: u8::from(game.home).into(),
            away_team: u8::from(game.away).into(),
            home_result: Some(u8::from(game.score.home).into()),
            away_result: Some(u8::from(game.score.away).into()),
            home_penalty: None,
            away_penalty: None,
            // TODO: FairPlay --> FairPlayScore
            //home_fair_play: Some(u8::from(game.fair_play.home).into()),
            //away_fair_play: Some(u8::from(game.fair_play.away).into()),
            home_fair_play: None,
            away_fair_play: None,
            played: false,
        }
    }
}
