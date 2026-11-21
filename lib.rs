use mod_api::{self, Command, CommandSource, Flow, Hooks, RegisterCommandsEvent, RunModCommandEvent, ServerHost, Text};

#[derive(Default)]
struct Mod;

#[mod_api::horizon_mod]
impl Hooks for Mod {
    fn register_commands(&self, _host: &mut dyn ServerHost, event: &mut RegisterCommandsEvent<'_>) {
        event.commands.push(Command::new("fly"));
    }

    fn run_mod_command(&self, host: &mut dyn ServerHost, event: &mut RunModCommandEvent<'_>) -> Flow {
        if event.command.literals.first().map(|s| s.as_str()) != Some("fly") {
            return Flow::Continue;
        }
        let CommandSource::Player { player, .. } = event.source else {
            return Flow::Cancel;
        };
        let player = player.clone();
        if let Some(mut a) = player.abilities(&*host) {
            let on = !a.may_fly;
            a.may_fly = on;
            a.flying = on;
            player.set_abilities(host, a);
            let msg = if on { "Fly: ON" } else { "Fly: OFF" };
            let _ = player.tell(host, Text::literal(msg));
        }
        Flow::Cancel
    }
}
