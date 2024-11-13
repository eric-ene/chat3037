use std::sync::Mutex;
use tauri::State;
use crate::appstate::context::Context;

#[tauri::command]
pub fn try_connect(state: State<'_, Mutex<Context>>, seq: String) -> Result<String, String> {
  let mut ctx = state.lock().expect("couldn't lock state mutex!");
  
  return match ctx.session.try_prase(&seq) {
    Ok(_) => {
      ctx.session.connect()?;
      ctx.session.get_other()
    },
    Err(e) => Err(e)
  };
}