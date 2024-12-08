use crate::primitive::Status;

pub trait AppRunner {
    fn run(&self) -> Status;
}

pub struct MacOS {}

impl AppRunner for MacOS {
    fn run(&self) -> Status {
        println!("Macos is running app");
        Status::Running
    }
}

pub struct IPhone {}

impl AppRunner for IPhone {
    fn run(&self) -> Status {
        println!("IPhone is running app");
        Status::Schedued
    }
}

pub fn run_apps(envs: Vec<Box<dyn AppRunner>>) {
    for env in envs {
        env.run();
    }
}
