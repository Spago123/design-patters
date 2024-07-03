use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::{self, Write};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

#[derive(Debug, Clone)]
struct Aircraft {
    id: String,
    fuel_left: u32,
    distance_from_airport: u32,
    malfunction: bool,
    requested_landing: bool,
}

impl Aircraft {
    fn new(id: String, fuel_left: u32, distance_from_airport: u32) -> Self {
        Aircraft {
            id,
            fuel_left,
            distance_from_airport,
            malfunction: false,
            requested_landing: false,
        }
    }

    fn set_malfunction(&mut self, malfunction: bool) {
        self.malfunction = malfunction;
    }
}

enum ATCCommand {
    RegisterAircraft(Aircraft),
    RequestLanding(String),
    NotifyMalfunction(String),
    LandingCleared,
}

trait Mediator {
    fn register_aircraft(&mut self, aircraft: Aircraft);
    fn request_landing(&mut self, id: &str);
    fn notify_malfunction(&mut self, id: &str);
    fn landing_cleared(&mut self) -> Option<String>;
}

struct AirTrafficControl {
    aircrafts: HashMap<String, Aircraft>,
}

impl AirTrafficControl {
    fn new() -> Self {
        AirTrafficControl {
            aircrafts: HashMap::new(),
        }
    }

    fn decide_landing_priority(&self) -> Option<&Aircraft> {
        self.aircrafts
            .values()
            .filter(|a| a.requested_landing)
            .filter(|a| a.malfunction)
            .min_by_key(|a| (a.fuel_left, a.distance_from_airport))
            .or_else(|| {
                self.aircrafts
                    .values()
                    .filter(|a| a.requested_landing)
                    .filter(|a| !a.malfunction)
                    .min_by_key(|a| a.distance_from_airport)
            })
    }

    fn log_message(&self, message: &str) {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open("log.txt")
            .unwrap();
        writeln!(file, "{}", message).unwrap();
    }
}

impl Mediator for AirTrafficControl {
    fn register_aircraft(&mut self, aircraft: Aircraft) {
        self.aircrafts.insert(aircraft.id.clone(), aircraft.clone());
        self.log_message(&format!(
            "Aircraft {} registered with fuel left: {}, distance from airport: {}",
            aircraft.id, aircraft.fuel_left, aircraft.distance_from_airport
        ));
    }

    fn request_landing(&mut self, id: &str) {
        let mut requested_landing = false;
        if let Some(aircraft) = self.aircrafts.get_mut(id) {
            aircraft.requested_landing = true;
            requested_landing = true;
        }
        if requested_landing {
            if let Some(priority_aircraft) = self.decide_landing_priority() {
                if priority_aircraft.id == id {
                    self.log_message(&format!("Aircraft {} is cleared to land.", id));
                } else {
                    self.log_message(&format!(
                        "Aircraft {} is not cleared to land yet. Priority is given to Aircraft {}.",
                        id, priority_aircraft.id
                    ));
                }
            }
        }
    }

    fn notify_malfunction(&mut self, id: &str) {
        if let Some(aircraft) = self.aircrafts.get_mut(id) {
            aircraft.set_malfunction(true);
            self.log_message(&format!(
                "Aircraft {} has malfunctioned. It will be given priority to land.",
                id
            ));
        }
    }

    fn landing_cleared(&mut self) -> Option<String> {
        if let Some(priority_aircraft) = self.decide_landing_priority() {
            let id = priority_aircraft.id.clone();
            self.aircrafts.remove(&id);
            self.log_message(&format!(
                "Aircraft {} has landed and is removed from the queue.",
                id
            ));
            if let Some(next_aircraft) = self.decide_landing_priority() {
                self.log_message(&format!(
                    "Aircraft {} is cleared to land next.",
                    next_aircraft.id
                ));
            } else {
                self.log_message("No more aircrafts waiting to land.");
            }
            Some(id)
        } else {
            self.log_message("No aircrafts waiting to land.");
            None
        }
    }
}

fn main() {
    let atc = Arc::new(Mutex::new(AirTrafficControl::new()));
    let (tx, rx) = mpsc::channel();

    let atc1 = Arc::clone(&atc);
    let handle1 = thread::spawn(move || {
        while let Ok(command) = rx.recv() {
            let mut atc = atc1.lock().unwrap();
            match command {
                ATCCommand::RegisterAircraft(aircraft) => {
                    atc.register_aircraft(aircraft);
                }
                ATCCommand::RequestLanding(id) => {
                    atc.request_landing(&id);
                }
                ATCCommand::NotifyMalfunction(id) => {
                    atc.notify_malfunction(&id);
                }
                ATCCommand::LandingCleared => {
                    atc.landing_cleared();
                }
            }
        }
    });

    let tx1 = tx.clone();
    let handle2 = thread::spawn(move || loop {
        let mut input = String::new();
        print!("Enter command: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();

        let parts: Vec<&str> = input.split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }

        match parts[0] {
            "register" => {
                if parts.len() == 4 {
                    let id = parts[1].to_string();
                    let fuel_left: u32 = parts[2].parse().unwrap_or(0);
                    let distance_from_airport: u32 = parts[3].parse().unwrap_or(0);
                    tx1.send(ATCCommand::RegisterAircraft(Aircraft::new(
                        id,
                        fuel_left,
                        distance_from_airport,
                    )))
                    .unwrap();
                } else {
                    println!("Usage: register <id> <fuel_left> <distance_from_airport>");
                }
            }
            "request" => {
                if parts.len() == 2 {
                    let id = parts[1].to_string();
                    tx1.send(ATCCommand::RequestLanding(id)).unwrap();
                } else {
                    println!("Usage: request <id>");
                }
            }
            "malfunction" => {
                if parts.len() == 2 {
                    let id = parts[1].to_string();
                    tx1.send(ATCCommand::NotifyMalfunction(id)).unwrap();
                } else {
                    println!("Usage: malfunction <id>");
                }
            }
            "clear" => {
                tx1.send(ATCCommand::LandingCleared).unwrap();
            }
            _ => {
                println!("Unknown command");
            }
        }
    });

    handle1.join().unwrap();
    handle2.join().unwrap();
}
