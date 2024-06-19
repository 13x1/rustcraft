use crate::com_handle::ComputerHandle;

pub use async_trait::async_trait;
pub use serde_json::Value;

pub type Res = anyhow::Result<()>;
pub type Args = (ComputerHandle, Value);


pub type ProgramID = String;

#[async_trait]
pub trait Program: Send + Sync {
    async fn program(&self, args: &Args) -> Res;
}

// subject to change!
pub trait ConstructableProgram: Program + 'static {
    fn name(&self) -> ProgramID;
}

/// boilerplate:
/// ```
/// use librustcraft::{program, prog};
/// program!(ExternalProg, async fn program(&self, (computer, arg): &prog::Args) -> prog::Res {
//     Ok(())
// });
/// ```
#[macro_export]
macro_rules! program {
    ($name:ident, $func:item) => {
        #[derive(Default, Clone)] struct $name;
        #[librustcraft::prog::async_trait] impl librustcraft::prog::Program for $name { $func }
        impl librustcraft::prog::ConstructableProgram for $name { fn name(&self) -> String { stringify!($name).into() } }
    };
}
