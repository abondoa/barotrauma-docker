use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "mod_manager", about = "Manage your Barotrauma mods!")]
enum Opt {
    Install {
        #[structopt(name = "USERNAME", about = "The steam user to use to install the mods")]
        username: String,
        #[structopt(name = "ID", about = "A list of mod IDs to install")]
        mods: Vec<u64>,
    },
    Remove {
        #[structopt(name = "ID", about = "A list of mod IDs to remove")]
        mods: Vec<u64>,
    },
}

fn main() {
    let opt = Opt::from_args();
    match opt {
        Opt::Install { username, mods } => println!("Installing {:?} as {}", mods, username),
        _ => (),
    }
}
