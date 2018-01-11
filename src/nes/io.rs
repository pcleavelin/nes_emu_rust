pub enum NESIoButton {
    A = 0,
    B = 1,
    Select = 2,
    Start = 3,
    Up = 4,
    Down = 5,
    Left = 6,
    Right = 7,
}

pub struct NESIo {
    buttons: [bool;8],
    controller_read_count: u8,
    strobe: bool,
}

impl NESIo {
    pub fn new() -> NESIo {
        NESIo {
            buttons: [false;8],

            controller_read_count: 0,

            strobe: false,
        }
    }

   
    pub fn set_controller_button(&mut self, btn: NESIoButton, pressed: bool) {
        //println!("BTN: {}", btn as usize);
        self.buttons[btn as usize] = pressed;
    }

    fn get_controller_button(&mut self) -> u8 {
        if self.strobe {
            self.controller_read_count = 0;
        }


        println!("Up is: {}",self.buttons[4]);
        println!("Down is: {}",self.buttons[5]);
        println!("Getting Button {}", self.controller_read_count);
        let btn = if self.controller_read_count < 8 {
            self.buttons[self.controller_read_count as usize] as u8
        } else {
            0//self.buttons[self.controller_read_count as usize] as u8
        };

        self.controller_read_count = self.controller_read_count.wrapping_add(1);
        self.controller_read_count %= 8;

        return btn;
    }

    pub fn read(&mut self, addr: usize) -> u8 {
        match addr {
            0x4016 => {
                self.get_controller_button()
                //0
            }

            0x4017 => {
                //self.get_controller_button()
                0
            }

            _ => {
                0
            }
        }
    }

    pub fn write(&mut self, addr: usize, val: u8) {
        match addr {
            0x4016 => {
                self.strobe = val&1 > 0;
            }

            _ => { }
        }
    }
}