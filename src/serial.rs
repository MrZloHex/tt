use serialport::SerialPort;

pub struct Serial {
    port: Option<Box<dyn SerialPort>>,
    baudrate: u32,
    portname: String
}

pub fn scan() -> Option<Vec<String>> {
    if let Ok(ps) = serialport::available_ports() {
        Some(ps.iter().map(|port| port.port_name.clone()).collect::<Vec<String>>())
    } else {
        None
    }
}

impl Serial {
    pub fn new() -> Self {
        Self {
            port: None,
            baudrate: 115200,
            portname: String::new()
        }
    }

    pub fn baudrate(mut self, baudrate: u32) -> Self {
        self.baudrate = baudrate;
        self
    }

    pub fn port(mut self, port: String) -> Self {
        self.portname = port;
        self
    }


    pub fn open(&mut self) -> () {
        self.port = if let Ok(p) = serialport::new(self.portname.clone(), self.baudrate).open() {
            Some(p)
        } else {
            eprintln!("ERROR: failed to open port");
            None
        };
    }

    pub fn write(&mut self, msg: &[u8]) -> Option<usize> {
        if let Ok(s) = self.port.as_mut().unwrap().write(msg) {
            Some(s)
        } else {
            eprintln!("ERROR: failed to write");
            None
        }
    }

    pub fn read(&mut self, buf: &mut [u8]) -> Option<usize> {
        if let Ok(s) = self.port.as_mut().unwrap().read(buf) {
            Some(s)
        } else {
            eprintln!("ERROR: failed to read ");
            None
        }
    }

}
