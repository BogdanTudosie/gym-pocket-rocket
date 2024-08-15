#[macro_use] extern crate rocket;
use serde_json;
use std::sync::{Arc, Mutex};
use rocket::{State, serde::json::Json};

mod model;
use model::gym::{Gym, load_gyms_from_file, save_gyms_to_file};

// sample index route, to be removed
#[get("/")]
fn index() -> &'static str {
    "Hello World"
}

#[get("/gyms")]
fn gyms(gyms: &State<Arc<Mutex<Vec<Gym>>>>) -> String {
    // Lock the mutex to get access to the inner Vec<Gym>
    let gyms_ref = gyms.lock().expect("Failed to lock mutex");
    // Serialize the gyms to JSON
    serde_json::to_string(&*gyms_ref).expect("Failed to serialize gyms")
}

#[post("/add_gym", format = "json", data = "<new_gym>")]
fn add_gym(new_gym: Json<Gym>, gyms: &State<Arc<Mutex<Vec<Gym>>>>) -> &'static str {
    // Lock the mutex to get access to the inner Vec<Gym>
    let mut gyms_ref = gyms.lock().expect("Failed to lock mutex");
    // Add the new gym to the list
    gyms_ref.push(new_gym.into_inner());
    // Save the updated list to the file
    save_gyms_to_file("gyms.json", &*gyms_ref);
    "Gym added and file updated"
}

#[launch]
fn rocket() -> _ {
    // Load gyms from a JSON file once at startup
    let gyms = load_gyms_from_file("gyms.json");
    let gyms = Arc::new(Mutex::new(gyms));

    rocket::build()
        .manage(gyms)
        .mount("/", routes![index, gyms, add_gym])
}
