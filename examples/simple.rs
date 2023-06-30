use truba::{Context, Message, MpscChannel};

struct Value(u32);

impl Message for Value {
    type Channel = MpscChannel<Self>;
}

struct MyActor {
    value: u32,
}

impl MyActor {
    fn run(ctx: Context, value: u32) {
        let mut value_in = ctx.receiver::<Value>();
        let mut actor = MyActor { value };

        truba::spawn_min_event_loop!(ctx, {
            Some(msg) = value_in.recv() => {
                actor.handle_value(msg);
            },
        });
    }

    fn handle_value(&mut self, Value(value): Value) {
        self.value = value;
        println!("receive value {value}");
    }
}

#[tokio::main]
async fn main() {
    let ctx = Context::new();
    MyActor::run(ctx.clone(), 42);

    let sender = ctx.sender::<Value>();
    sender.send(Value(11)).await.ok();
    sender.send(Value(22)).await.ok();
}
