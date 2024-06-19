use librustcraft::{program, prog, expose_programs};

program!(ExternalProg, async fn program(&self, (computer, arg): &prog::Args) -> prog::Res {
    println!("Called with: {:?}", arg);
    let res = computer.send("test".to_string()).await?;
    println!("Response: {}", res);
    Ok(())
});

expose_programs![ExternalProg];