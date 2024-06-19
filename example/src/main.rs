use actix_web::{App, HttpServer};
use anyhow::Result;
use librustcraft::{net, program, prog, state};
use librustcraft::serde_json::Value;

#[actix_web::main]
async fn main() -> Result<()> {
    // env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    let example_json = Value::Array(vec!(Value::Bool(true), Value::String("hello".to_string())));


    let (data, state) = state::State::create_data();
    let server_handle = tokio::spawn(HttpServer::new(move || {
        App::new()
            // .wrap(Logger::default())
            // .wrap(Logger::new("%a %{User-Agent}i"))
            .app_data(data.clone())
            .service(net::callback)
            .service(net::fetch_tasks)
    })
        .bind(("127.0.0.1", 9999))?
        .run());

    let handle = state.insert_computer_handle("test".to_string()).await;

    state.load_programs("example_program.dll".into()).await;

    program!(Program1, async fn program(&self, (computer, arg): &prog::Args) -> prog::Res {
        println!("Called with: {:?}", arg);
        let res = computer.send("test".to_string()).await?;
        println!("Response: {}", res);
        Ok(())
    });
    
    state.insert_constructable_program(Program1).await;


    state.run_program_on_computer("ExternalProg".into(), handle, example_json).await?;

    server_handle.await??;
    Ok(())
}
