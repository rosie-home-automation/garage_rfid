extern crate garage_rfid;
use garage_rfid::great_manager::GreatManager;

fn main() {
  println!("START");
  match GreatManager::new() {
    Ok(mut great_manager) => great_manager.start(),
    Err(err) => println!("Error {:?}", err)
  }
  println!("DONE");
}
