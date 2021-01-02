use std::sync::Arc;
use crate::world::entity::{Entity, EntityIterator, EntityContainer};
use std::any::{TypeId, type_name};
use crate::world::World;
use crate::world::entity::component::{Component, ComponentManager, ComponentWriteAccess, ComponentReadAccess};
use std::fmt::{Formatter, Display, Debug};
use std::error::Error;
use core::fmt;
use std::thread;
use std::marker::PhantomData;
use std::thread::sleep;
use failure::_core::time::Duration;
use std::time::Instant;
use crate::world::entity::builder::EntityBuilder;
use crate::world::entity::component::transform::Transform;
use crate::world::system::translate::TranslateSystem;

pub mod translate;
pub mod gravity;
pub mod input;

type SysEnvComponent<'a, C> = ComponentReadAccess<'a, C>;
type SysEnvComponentMut<'a, C> = ComponentWriteAccess<'a, C>;

pub trait System<'a>: Send + Sync
{
    type Environment;

    fn new() -> Self;

    fn on_fetch<T: EntityContainer>(&mut self, source: &T) -> Result<(), SystemRuntimeError>;

    fn on_freeze(&'a self) -> Result<Self::Environment, SystemRuntimeError>;

    //fn on_recalculate(&'a mut self);

    fn on_run(&self, environment: Self::Environment, delta: Duration);

    fn start<'b, T: EntityContainer>(&'b mut self, source: T) where 'b: 'a {
        let system_name = type_name::<Self>();

        println!("{}: Initializing system...", system_name);

        // Initialize:
        while let Err(SystemRuntimeError(e)) = self.on_fetch(&source){
            let timeout_s = 5;
            print!("{}: System runtime error ({})", system_name, e);
            println!("; Retrying in {} secs", timeout_s);
            sleep(Duration::new(timeout_s, 0));
        }

        println!("{}: Now online!", system_name);

        let mut prev_time = Instant::now();

        loop {
            let now = Instant::now();
            let delta = now - prev_time;
            prev_time = now;

            if let Ok(res) = self.on_freeze() {
                self.on_run(res, delta);
            }

            std::thread::sleep_ms(10);
        }
    }
}

pub struct SystemRuntimeError(&'static str);
impl Error for SystemRuntimeError {}
impl Display for SystemRuntimeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "System runtime error")
    }
}
impl Debug for SystemRuntimeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "")
    }
}

pub fn start_system_in_parallel<T1, T2>(environment: T2)
    where T1: for<'a> System<'a> + 'static,
          T2: 'static + EntityContainer {

    let env_copy = environment.clone();
    thread::spawn(  move || { T1::new().start(env_copy); });
}
