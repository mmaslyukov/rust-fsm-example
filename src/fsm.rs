
use core::fmt;

#[derive(Debug)]
pub enum Event {
    Stop,
    Start,
    Name(String),
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

#[derive(Debug)]
enum State {
    New(Data),
    Idle(Data),
    Reg(Data),
    Surv(Data),
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

#[derive(Debug)]
struct Data {
    name: String,
    active: bool
}
impl fmt::Display for Data {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl Data {
    pub fn new() -> Self {
        Self {
            name: String::default(),
            active: false
        }
    }

    pub fn wrap<S>(mut self, state: S) -> State
    where
        S: 'static + Fn(Self) -> State,
    {
        let new_state = state(self);
        println!("</TR> transit to {}", new_state.to_string());
        new_state
    }

    pub fn start(&mut self, _e: Event) {
        self.active = true;
    }
    pub fn stop(&mut self, _e: Event) {
        self.active = false;
    }
    pub fn name(&mut self, e: Event) {
        println!("Name {:?}", e);
        if let Event::Name(name) = e {
            self.name = name;
        }
    }
}

impl State {
    pub fn consume(self, e: Event) -> Transition {
        match (self, e) {
            (State::New(data), e @ Event::Start) => {
                Transition::make_valid(State::Reg, data, e, Data::start)
            }
            (State::Reg(data), e @ Event::Name(_)) => {
                Transition::make_valid(State::Reg, data, e, Data::name)
            }
            (State::Reg(data), e @ Event::Stop) => {
                Transition::make_valid(State::Idle, data, e, Data::stop)
            }
            (s, e) => Transition::make_shallow(s, e),
        }
    }
    pub fn data(&self) -> &Data {
        match self {
            State::New(data) => &data,
            State::Idle(data) => &data,
            State::Reg(data) => &data,
            State::Surv(data) => &data,
        }
    }
}

type Wrapper = fn(Data) -> State;
type Handler = fn(&mut Data, Event);

struct Transition {
    wrap: Option<Wrapper>,
    data: Option<Data>,
    next: Option<State>,
    event: Event,
    func: Option<Handler>,
}

impl Transition {
    pub fn make_valid(wrap: Wrapper, data: Data, event: Event, func: Handler) -> Self {
        Self {
            wrap: Some(wrap),
            data: Some(data),
            next: None,
            event,
            func: Some(func),
        }
    }

    pub fn make_shallow(next: State, event: Event) -> Self {
        Self {
            wrap: None,
            data: None,
            next: Some(next),
            event,
            func: None,
        }
    }

    pub fn transit(mut self) -> State {
        println!("<TR> event: {}", self.event);
        if self.data.is_some() && self.wrap.is_some() {
            let wrap = self.wrap.unwrap();
            let mut data = self.data.unwrap();
            if self.func.is_some() {
                (self.func.unwrap())(&mut data, self.event);
            }
            let state = data.wrap(wrap);
            state
        } else {
            self.next.unwrap()
        }
    }
}


// To see output run: 
//  cargo test -- --nocapture test_fsm
#[cfg(test)]
mod tests {
    use super::{Data, Event, State};

    #[test]
    pub fn test_fsm() {
        let mut state = State::New(Data::new());
        state = state.consume(Event::Start).transit();
        state = state.consume(Event::Name("Johny".into())).transit();
        state = state.consume(Event::Stop).transit();

        println!("data: {}", state.data())
    }
}