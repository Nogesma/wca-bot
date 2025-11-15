use serde::Serialize;

#[cynic::schema("wcalive")]
mod schema {}

#[derive(cynic::QueryFragment)]
pub struct RootQueryType {
    pub recent_records: Vec<Record>,
}

#[derive(cynic::QueryFragment, Serialize, Hash, Eq, PartialEq, Clone)]
pub struct Record {
    #[cynic(rename = "type")]
    pub t: String,
    pub tag: String,
    pub attempt_result: i32,
    pub result: Res,
}

#[derive(cynic::QueryFragment, Serialize, Hash, Eq, PartialEq, Clone)]
#[cynic(graphql_type = "Result")]
pub struct Res {
    pub person: Person,
    pub round: Round,
}

#[derive(cynic::QueryFragment, Serialize, Hash, Eq, PartialEq, Clone)]
pub struct Person {
    pub name: Option<String>,
    pub country: Country,
}

#[derive(cynic::QueryFragment, Serialize, Hash, Eq, PartialEq, Clone)]
pub struct Country {
    pub iso2: String,
    pub name: String,
}

#[derive(cynic::QueryFragment, Serialize, Hash, Eq, PartialEq, Clone)]
pub struct Round {
    pub id: cynic::Id,
    pub competition_event: CompetitionEvent,
}

#[derive(cynic::QueryFragment, Serialize, Hash, Eq, PartialEq, Clone)]
pub struct CompetitionEvent {
    pub event: Event,
    pub competition: Competition,
}

#[derive(cynic::QueryFragment, Serialize, Hash, Eq, PartialEq, Clone)]
pub struct Competition {
    pub id: cynic::Id,
    pub name: String,
}

#[derive(cynic::QueryFragment, Serialize, Hash, Eq, PartialEq, Clone)]
pub struct Event {
    pub id: cynic::Id,
    pub name: String,
}
