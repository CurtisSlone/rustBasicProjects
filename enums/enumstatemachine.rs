enum ConnectionState {
    Closed,
    Listening,
    Connected,
    Error,
}

impl ConnectionState {
    fn next(self, action: &str) -> ConnectionState {
        match (self, action) {
            (ConnectionState::Closed, "listen") => ConnectionState::Listening,
            (ConnectionState::Listening, "connect") => ConnectionState::Connected,
            (ConnectionState::Connected, "close") => ConnectionState::Closed,
            (_, "error") => ConnectionState::Error,
            (state, _) => state, // Invalid action
        }
    }
}

fn main() {
    let state = ConnectionState::Closed;
    let state = state.next("listen");
    let state = state.next("connect");
    let state = state.next("error");

    match state {
        ConnectionState::Error => println!("Connection error occurred"),
        _ => println!("Connection state changed"),
    }
}
