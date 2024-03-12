use crate::*;

struct UiState {
    levels: [u32; 3],
    frame_rate: u64,
}

impl UiState {
    //Displays the current state of the UI
    fn show(&self) {
        let names = ["red", "green", "blue"];
        rprintln!();
        for (name, level) in names.iter().zip(self.levels.iter()) {
            rprintln!("{}: {}", name, level);
        }
        rprintln!("frame rate: {}", self.frame_rate);
    }
}

impl Default for UiState {
    fn default() -> Self {
        Self {
            levels: [LEVELS - 1, LEVELS - 1, LEVELS - 1],
            frame_rate: 100,
        }
    }
}

pub struct Ui {
    knob: Knob,
    _button_a: Button,
    _button_b: Button,
    state: UiState,
}

impl Ui {
    //Constructor 
    pub fn new(knob: Knob, _button_a: Button, _button_b: Button) -> Self {
        Self {
            knob,
            _button_a,
            _button_b,
            state: UiState::default(),
        }
    }

    //Async method to run the UI
    pub async fn run(&mut self) -> ! {
        //Measure knob level for Blue channel (levels = ["red", "green", "blue"])
        self.state.levels[2] = self.knob.measure().await;

        //Measure knob level for Green channel (levels = ["red", "green", "blue"])
        self.state.levels[1] = self.knob.measure().await;

        //Measure knob level for Red channel (levels = ["red", "green", "blue"])
        self.state.levels[0] = self.knob.measure().await;

        //Set the rgb levels based on ui state
        set_rgb_levels(|rgb| {
            *rgb = self.state.levels;
        })
        .await;

        self.state.show();

        loop {
            //Measure knob level
            let level = self.knob.measure().await;

            if self._button_a.is_high() && self._button_b.is_high(){
                if self.state.frame_rate != (level as u64 * 10)+10{
                    self.state.frame_rate = (level as u64 * 10)+10;
                    self.state.show();
                }
            }
                
            //If blue channel level has changed
            if self._button_a.is_low() && level != self.state.levels[2] {

                //Update level
                self.state.levels[2] = level;

                self.state.show();

                //Update rgb levels in rgb struct
                set_rgb_levels(|rgb| {
                    *rgb = self.state.levels;
                })
                .await;
            }

            if self._button_b.is_low() && level != self.state.levels[1] {

                //Update level
                self.state.levels[1] = level;

                self.state.show();

                //Update rgb levels in rgb struct
                set_rgb_levels(|rgb| {
                    *rgb = self.state.levels;
                })
                .await;
            }

            if (self._button_a.is_low() && self._button_b.is_low()) && level != self.state.levels[0] {

                //Update level
                self.state.levels[0] = level;

                self.state.show();

                //Update rgb levels in rgb struct
                set_rgb_levels(|rgb| {
                    *rgb = self.state.levels;
                })
                .await;
            }

            Timer::after_millis(50).await;
        }
    }
}
