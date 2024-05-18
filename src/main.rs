mod parsing;
use crate::parsing::parsing::ProgramConfig;

fn main() {
    let configs :Vec<ProgramConfig> = ProgramConfig::new("./confs/taskmaster_confs.yaml");
    for config in configs {
        println!("{:?}\n", config);
    }
}
