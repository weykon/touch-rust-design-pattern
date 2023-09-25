use super::Device;

#[derive(Clone)]
pub struct Radio {
    on: bool,
    volumn: u8,
    channel: u16,
}

impl Default for Radio {
    fn default() -> Self {
        Self {
            on: false,
            volumn: 30,
            channel: 1,
        }
    }
}

impl Device for Radio {
    fn is_enabled(&self) -> bool {
        self.on
    }

    fn enable(&mut self) {
        self.on = true;
    }

    fn disable(&mut self) {
        self.on = false;
    }

    fn volume(&self) -> u8 {
        self.volumn
    }

    fn set_volume(&mut self, percent: u8) {
        self.volumn = std::cmp::min(percent, 100);
    }

    fn channel(&self) -> u16 {
        self.channel
    }

    fn set_channel(&mut self, channel: u16) {
        self.channel = channel;
    }

    fn print_status(&self) {
        println!("------------------------------------");
        println!("Radio:");
        println!("  enabled: {}", self.on);
        println!("  volumn: {}", self.volumn);
        println!("  channel: {}", self.channel);
        println!("------------------------------------\n");
    }
}
