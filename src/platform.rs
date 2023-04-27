use zbus::{dbus_interface, SignalContext, fdo};

pub struct Greeter {
    pub count: u64
}

#[dbus_interface(name = "org.zbus.MyGreeter1")]
impl Greeter {
    // Can be `async` as well.
    fn say_hello(&mut self, name: &str) -> String {
        self.count += 1;
        format!("Hello {}! I have been called {} times.", name, self.count)
    }

    // Rude!
    async fn go_away(
        &self,
        #[zbus(signal_context)]
        ctxt: SignalContext<'_>,
    ) -> fdo::Result<()> {
        //Self::greeted_everyone(&ctxt).await?;
        //self.done.notify(1);

        Ok(())
    }

    fn switch_fan_mode(&self, value: i32) -> i32 {
        2 * 3 * value
    }
}