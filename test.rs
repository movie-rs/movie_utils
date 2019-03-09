impl Actor {
    fn start(self) -> Handle<StdThread> {}
    fn start_with(self, runner: T) -> Handle<O>
    where
        T: Fn() -> O,
    {
        // ...
    }
}

actor! {
    StreamParsingActor
        in:
            ChangeSource(String),
            Tick,
            Stop,
        out:
            Ok,
            Err(String),
        config:
            device: u16,
        context:
            lines_parsed: u64 = 0,
        on_init: ()
        on_message:
            ChangeSource(name) => {
                self.device = lookup_device(name);
                Ok
            },
            Tick => {
                println!("{}", parse(get_data_from_device(self.device)));
                Ok
            },
            Stop => Ok,
}

actor! {
    SimplestActor
        in:
            Ping,
        out:
            Pong,
        on_message:
            Ping => Pong,
}

actor! {
    SimpleActor
        in:
            Ping,
        out:
            Pong,
        data:
            counter: u64 = 0,
        on_message:
            Ping => {
                counter += 1;
                Pong
            },
}

// movie actors are expensive
