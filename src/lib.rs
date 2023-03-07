use std::collections::HashSet;

enum Command {
    Reserve { number: u32, row: u32 },
}

#[derive(Debug, Eq, PartialEq, Clone)]
enum Event {
    SeatReserved { id: u32,},
}


struct CommandHandler {
    events: Vec<Event>,
}

impl CommandHandler {
    fn handle(&self, cmd: Command, publish: impl FnMut(Event)) {
        match cmd {
            Command::Reserve { .. } => {
                let state = SeatState::load(self.events.to_vec());
                
                let mut reservation = Reservation::new(state, publish);
                reservation.reserve(0);
            }
        }
    }
}

type Seat = u32;

struct SeatState {
    reserved_seats: HashSet<Seat>
}

impl SeatState {
    fn apply_events(&mut self, events: Vec<Event>) {
        for event in events {
            self.apply(event)
        }
    }
    
    fn apply(&mut self, event: Event) {
        match event {
            Event::SeatReserved { id, .. } => {
                self.reserved_seats.insert(id);
            }
        }
    }
    
    fn load(events: Vec<Event>) -> SeatState {
        let mut initial_state = SeatState {
            reserved_seats: HashSet::new(),
        };
        
        initial_state.apply_events(events);

        initial_state
    }
}


struct Reservation<F: FnMut(Event)> {
    state: SeatState,
    f: F,    
}

impl<F: FnMut(Event)> Reservation<F> {
    fn new(state: SeatState, f: F) -> Reservation<F> {
        Reservation {
            state,
            f,
        }
    }
    
    fn reserve(&mut self, id: u32) {
        let event = Event::SeatReserved {
            id,
        };

        (self.f)(event)
    }
}


#[cfg(test)]
mod tests {
    use crate::{Command, CommandHandler, Event};

    #[test]
    fn test_reserves_a_seat() {
        let events = vec![Event::SeatReserved {
            id: 0,
        }];
        TestFramework::new()
            .given(vec![])
            .when(Command::Reserve { number: 0, row: 0 })
            .then(&events);

    }

    struct TestFramework {
        history: Vec<Event>,
        published_events: Vec<Event>,
    }

    impl TestFramework {
        fn new() -> TestFramework {
            TestFramework {
                history: vec![],
                published_events: vec![],
            }
        }
        fn given(mut self, events: Vec<Event>) -> TestFramework {
            self.history = events;
            self
        }
        fn when(mut self, cmd: Command) -> TestFramework {
            let handler = CommandHandler {
                events: self.history.to_vec(),
            };

            handler.handle(cmd, |event| self.published_events.push(event));

            self
        }
        fn then(self, expected_events: &[Event]) {
            assert_eq!(expected_events, self.published_events)
        }
    }
}
