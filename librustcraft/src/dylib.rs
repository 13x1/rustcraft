use std::sync::Arc;
use dlopen2::wrapper::{Container, WrapperApi};
use crate::prog::{ConstructableProgram, Program};

pub type ProgramListEntry = (String, Arc<dyn Program>);
pub type ProgramList = Vec<ProgramListEntry>;

#[derive(WrapperApi)]
struct Api {
    get_programs: fn() -> ProgramList
}

pub(crate) fn get_programs_from_file(name: String) -> ProgramList {
    let container: Container<Api> = match unsafe { Container::load(name) } {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error loading library: {}", e);
            return vec![]
        }
    };
    container.get_programs()
}

#[allow(dead_code)] // don't know why it's not finding the usage
pub fn prog_into_list(prog: impl ConstructableProgram) -> ProgramListEntry {
    (prog.name(), Arc::new(prog))
}

#[macro_export]
macro_rules! expose_programs {
    // takes a list of ConstructablePrograms and creates the get_programs function
    ($($prog:expr),*) => {
        #[no_mangle]
        pub fn get_programs() -> librustcraft::dylib::ProgramList {
            vec![$(librustcraft::dylib::prog_into_list($prog)),*]
        }
    };
}