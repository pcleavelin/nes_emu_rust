use libusb::*;
use std::result::Result;
use std::time::Duration;

pub struct ControllerScanner {
    context: Context,
}

pub struct Adapter<'a> {
    device: Device<'a>,
}

pub struct Listener<'a> {
    handle: DeviceHandle<'a>,
    buffer: [u8; 37],
    has_kernel_driver: bool,
    interface: u8,
    endpoint_in: u8,
}

pub struct Controller {
    pub a: bool,
    pub b: bool,
    pub select: bool,
    pub start: bool,
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
}

impl ControllerScanner {
    pub fn new() -> ControllerScanner {
        ControllerScanner {
            context: Context::new().unwrap(),
        }
    }

    pub fn find_adapter<'a>(&'a mut self, vid: u16, pid: u16) -> Result<Option<Adapter<'a>>,Error> {
        for mut device in try!(self.context.devices()).iter() {
            let device_desc = match device.device_descriptor() {
                Ok(d) => d,
                Err(_) => continue
            };

            if device_desc.vendor_id() == vid && device_desc.product_id() == pid {
                println!("Found adapter");
                return Ok(Some(Adapter {
                    device: device
                }))
            }
        }

        println!("Couldn't find adapter");
        return Ok(None);
    }
}

impl<'a> Adapter<'a> {
    pub fn listen(&mut self) -> Result<Listener<'a>, Error> {
        let mut handle = try!(self.device.open());

        let config = try!(self.device.config_descriptor(0));

        let mut interface_descriptor: Option<_> = None;
        let mut endpoint_in = None;
        let mut endpoint_out = None;

        for interface in config.interfaces() {
            interface_descriptor = None;
            endpoint_in = None;
            endpoint_out = None;
            for desc in interface.descriptors() {
                for endpoint in desc.endpoint_descriptors() {
                    match endpoint.direction() {
                        Direction::In => endpoint_in = Some(endpoint.address()),
                        Direction::Out => endpoint_out = Some(endpoint.address()),
                    }
                }
                interface_descriptor = Some(desc);
            }
        }

        if interface_descriptor.is_none() || endpoint_in.is_none() || endpoint_out.is_none() {
            println!("Descriptor not supported");
            return Err(Error::NotSupported);
        }

        let interface_descriptor = interface_descriptor.unwrap();
        let interface_number = interface_descriptor.interface_number();

        let has_kernel_driver = match handle.kernel_driver_active(interface_number) {
            Ok(true) => {
                try!(handle.detach_kernel_driver(interface_number));
                true
            },
            _ => false,
        };

        try!(handle.set_active_configuration(config.number()));
        try!(handle.claim_interface(interface_number));
        let setting = interface_descriptor.setting_number();
        try!(handle.set_alternate_setting(interface_number, setting));

        // Tell the adapter to start sending packets.

        println!("Got listener");
        Ok(Listener {
            handle: handle,
            buffer: [0; 37],
            has_kernel_driver: has_kernel_driver,
            interface: interface_number,
            endpoint_in: endpoint_in.unwrap(),
        })
    }
}

impl<'a> Listener<'a> {
    pub fn read(&mut self) -> Result<Option<Controller>, Error> {
        let timeout = Duration::from_secs(1);
        match self.handle.read_interrupt(self.endpoint_in, &mut self.buffer, timeout) {
            Ok(_) => Ok(Controller::parse(&self.buffer)),
            //Ok(_) => Err(Error::Io),
            Err(err) => Err(err),
        }
    }
}

impl Controller {
    fn parse(data: &[u8]) -> Option<Controller> {
        //println!("Controller Data: {:?}", data);
        Some(Controller{
            a: (data[0] >> 1)&1 > 0,
            b: data[0]&1 > 0,

            select: data[1]&1 > 0,
            start: (data[1] >> 1)&1 > 0,

            up: data[2] == 0 || data[2] == 1 || data[2] == 7,
            down: data[2] == 3 || data[2] == 4 || data[2] == 5,
            left: data[2] == 5 || data[2] == 6 || data[2] == 7,
            right: data[2] == 1 || data[2] == 2 || data[2] == 3,
        })
    }
}