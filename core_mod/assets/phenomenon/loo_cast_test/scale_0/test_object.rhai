fn register_phenomenon(ctx) {
    let phenomenon = ctx.start();

    phenomenon.name("Test Object");
    phenomenon.spawn(spawn)

    ctx.end(phenomenon);
}

fn spawn(ctx) {}

fn update(ctx) {}

fn fixedUpdate(ctx) {}