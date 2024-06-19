use std::sync::Arc;
use tokio::sync::{Mutex, oneshot};
use std::collections::HashMap;
use std::ops::Deref;
use actix_web::App;
use actix_web::dev::{ServiceFactory, ServiceRequest, ServiceResponse};
use serde_json::Value;
use anyhow::{Context, Result};
use crate::com_handle::{ComputerHandle, ComputerID, RawCCRequestO};
use crate::dylib::{get_programs_from_file};
use crate::net::Data;
use crate::prog::{ConstructableProgram, Program, ProgramID};

// typedefs
pub type CallbackID = String;


pub struct State {
    pub(crate) computer_registry: Mutex<HashMap<ComputerID, ComputerHandle>>,
    pub(crate) callback_registry: Mutex<HashMap<CallbackID, oneshot::Sender<RawCCRequestO>>>,
    pub(crate) programs_registry: Mutex<HashMap<ProgramID, Arc<dyn Program>>>,
}

impl State {
    pub fn create_data() -> (Data, Arc<State>) {
        let s = State {
            computer_registry: Mutex::new(HashMap::new()),
            callback_registry: Mutex::new(HashMap::new()),
            programs_registry: Mutex::new(HashMap::new()),
        };
        let d = Data::new(s);
        let s_ref = d.clone().into_inner();
        (d, s_ref)
    }

    pub fn create_app() -> (App<impl ServiceFactory<ServiceRequest, Config=(), Response=ServiceResponse, Error=actix_web::Error>>, Arc<State>) {
        let (data, state) = State::create_data();
        (App::new().app_data(data.clone()), state)
    }

    pub async fn insert_computer_handle(&self, id: ComputerID) -> ComputerHandle {
        let handle = ComputerHandle::new(id.clone());
        let mut computer_registry = self.computer_registry.lock().await;
        computer_registry.insert(id, handle.clone());
        handle
    }

    pub async fn get_computer_handle(&self, id: &ComputerID) -> Option<ComputerHandle> {
        let computer_registry = self.computer_registry.lock().await;
        computer_registry.get(id).cloned()
    }

    pub async fn insert_program(&self, id: ProgramID, handler: Arc<dyn Program>) {
        let mut programs_registry = self.programs_registry.lock().await;
        programs_registry.insert(id, handler);
    }

    pub async fn insert_constructable_program(&self, program: impl ConstructableProgram) {
        self.insert_program(program.name(), Arc::new(program)).await;
    }

    pub async fn run_program_on_computer(&self, program_id: String, computer: ComputerHandle, arg: Value) -> Result<()> {
        let program = {
            let programs_registry = self.programs_registry.lock().await;
            programs_registry.get(&program_id).cloned().context("Program not found")?
        };
        program.program(&(computer, arg)).await
    }

    #[cfg(feature = "dylib")]
    pub async fn load_programs(&self, name: String) {
        // i have no idea what the fuck i am doing
        println!("fuck 1");
        let progs = get_programs_from_file(name);
        println!("fuck 2");
        let _program_registry = self.programs_registry.lock().await;
        println!("fuck 3");
        for (_name, prog) in progs {
            println!("fuck 4");
            // program_registry.insert(name, prog);
            println!("fuck 5");
            let prog_addr = prog.deref();
            println!("fuck 6 {:p}", prog_addr);
            let comp_reg = self.computer_registry.lock().await;
            let comp = comp_reg.get(&"test".to_string()).cloned().unwrap();
            println!("fuck 7");
            prog_addr.program(&(comp, Value::Null)).await.unwrap();
        }
        println!("fuck fuck fuck");
    }
}
