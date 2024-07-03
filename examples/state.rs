enum Signals {
    Opened,
    Closed,
    Taster,
}

trait State {
    fn new() -> Self;
    fn entry();
    #[allow(unused)]
    fn exit();
    fn trans(self, signal: Signals) -> States;
}

#[derive(Clone)]
struct DoorOpen;

#[derive(Clone)]
struct DoorClosed;

#[derive(Clone)]
struct DoorOpening;

#[derive(Clone)]
struct DoorClosing;

#[derive(Clone)]
struct InterruptedOpening;

#[derive(Clone)]
struct InterruptedClosing;

#[derive(Clone)]
enum States {
    DoorClosed(DoorClosed),
    DoorOpening(DoorOpening),
    DoorOpen(DoorOpen),
    DoorClosing(DoorClosing),
    InterruptedOpening(InterruptedOpening),
    InterruptedClosing(InterruptedClosing),
}

struct Fsm {
    state: States,
}

impl State for DoorOpen {
    fn new() -> Self {
        DoorOpen::entry();
        DoorOpen {}
    }

    fn entry() {
        println!("Door is open");
    }

    fn exit() {
        todo!()
    }

    fn trans(self, signal: Signals) -> States {
        match signal {
            Signals::Taster => States::DoorClosing(DoorClosing::new()),
            _ => States::DoorOpen(self),
        }
    }
}

impl State for DoorClosing {
    fn new() -> Self {
        DoorClosing::entry();
        DoorClosing {}
    }

    fn entry() {
        println!("The door is closing");
    }

    fn exit() {
        println!("The door stopped closing");
    }

    fn trans(self, signal: Signals) -> States {
        match signal {
            Signals::Taster => States::InterruptedClosing(InterruptedClosing::new()),
            Signals::Closed => States::DoorClosed(DoorClosed::new()),
            _ => States::DoorClosing(self),
        }
    }
}

impl State for DoorClosed {
    fn new() -> Self {
        DoorClosed::entry();
        DoorClosed {}
    }

    fn entry() {
        println!("Door is Closed");
    }

    fn exit() {
        todo!();
    }

    fn trans(self, signal: Signals) -> States {
        match signal {
            Signals::Taster => States::DoorOpening(DoorOpening::new()),
            _ => States::DoorClosed(self),
        }
    }
}

impl State for DoorOpening {
    fn new() -> Self {
        DoorOpening::entry();
        DoorOpening {}
    }

    fn entry() {
        println!("The door is opening");
    }

    fn exit() {
        println!("The door stopped opening");
    }

    fn trans(self, signal: Signals) -> States {
        match signal {
            Signals::Taster => States::InterruptedOpening(InterruptedOpening::new()),
            Signals::Opened => States::DoorOpen(DoorOpen::new()),
            _ => States::DoorOpening(self),
        }
    }
}

impl State for InterruptedClosing {
    fn new() -> Self {
        InterruptedClosing::entry();
        InterruptedClosing {}
    }

    fn entry() {
        println!("The door is interrupted closing");
    }

    fn exit() {
        todo!()
    }

    fn trans(self, signal: Signals) -> States {
        match signal {
            Signals::Taster => States::DoorOpening(DoorOpening::new()),
            _ => States::InterruptedClosing(self),
        }
    }
}

impl State for InterruptedOpening {
    fn new() -> Self {
        InterruptedOpening::entry();
        InterruptedOpening {}
    }

    fn entry() {
        println!("The door is interrupted opening");
    }

    fn exit() {
        todo!()
    }

    fn trans(self, signal: Signals) -> States {
        match signal {
            Signals::Taster => States::DoorClosing(DoorClosing::new()),
            _ => States::InterruptedOpening(self),
        }
    }
}

impl State for States {
    fn new() -> Self {
        todo!()
    }

    fn entry() {}

    #[warn(dead_code)]
    fn exit() {}

    fn trans(self, signal: Signals) -> States {
        match self {
            States::DoorOpening(state) => state.trans(signal),
            States::DoorClosing(state) => state.trans(signal),
            States::InterruptedOpening(state) => state.trans(signal),
            States::InterruptedClosing(state) => state.trans(signal),
            States::DoorClosed(state) => state.trans(signal),
            States::DoorOpen(state) => state.trans(signal),
        }
    }
}

impl Fsm {
    fn new() -> Self {
        Fsm {
            state: States::InterruptedOpening(InterruptedOpening::new()),
        }
    }

    fn trans(&mut self, signal: Signals) {
        self.state = self.state.clone().trans(signal);
    }
}

fn main() {
    let mut fsm = Fsm::new();
    fsm.trans(Signals::Taster);
    fsm.trans(Signals::Taster);
    fsm.trans(Signals::Taster);
    fsm.trans(Signals::Closed);
    fsm.trans(Signals::Opened);
    fsm.trans(Signals::Closed);
    fsm.trans(Signals::Opened);
    fsm.trans(Signals::Taster);
    fsm.trans(Signals::Opened);
}
