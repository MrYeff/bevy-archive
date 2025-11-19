use clap::Parser;
use shared::prelude::*;

#[derive(Debug, Parser, Clone)]
struct Args {
    #[command(flatten)]
    worker: WorkerArgs,
    #[command(flatten)]
    redis_render_worker: RedisAccessArgs<{ RedisDesignation::RenderWorkerCache as u32 }>,
}

fn main() {
    println!("Hello, world!");
}
