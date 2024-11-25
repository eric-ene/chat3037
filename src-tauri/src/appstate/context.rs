use crate::appstate::session::Session;

pub struct Context {
  pub id: Option<String>,
  pub name: Option<String>,
  pub session: Session,
}