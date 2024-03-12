use crate::*;

type RgbPins = [Output<'static, AnyPin>; 3];

pub struct Rgb {
    //Array of RGB pins 
    rgb: RgbPins,
    // Shadow variables to minimize lock contention.
    levels: [u32; 3],

    //Time interval for each tick (in microseconds)
    tick_time: u64,
}

impl Rgb {
    //Caculate tick time based on frame rate (in microseconds)
    pub fn frame_tick_time(frame_rate: u64) -> u64 {
        1_000_000 / (3 * frame_rate * LEVELS as u64)
    }

    //Constructor for RGB
    pub fn new(rgb: RgbPins, frame_rate: u64) -> Self {
        let tick_time = Self::frame_tick_time(frame_rate);
        Self {
            rgb,
            levels: [0; 3],
            tick_time,
        }
    }

    //Perform a step for a given LED
    async fn step(&mut self, led: usize) {

        //Retrieve current level
        let level = self.levels[led];

        //Turn on LED if level > 0, turn off after specified duration
        if level > 0 {
            self.rgb[led].set_high();
            let on_time = level as u64 * self.tick_time;
            Timer::after_micros(on_time).await;
            self.rgb[led].set_low();
        }

        //Caculate remaining time
        let level = LEVELS - level;
        if level > 0 {
            let off_time = level as u64 * self.tick_time;
            
            //Wait for that amount of time
            Timer::after_micros(off_time).await;
        }
    }

    pub async fn run(mut self) -> ! {
        loop {
            self.levels = get_rgb_levels().await;

            for led in 0..3 {
                self.step(led).await;
            }
        }
    }
}
