use rand::{distributions::Uniform, prelude::Distribution, rngs::SmallRng, SeedableRng};

use super::Command;

pub struct Rm;

impl Command for Rm {
    fn execute(&self, tokens: &Vec<&str>, terminal: &xterm_js_rs::Terminal) -> bool {
        let has_dest = tokens.iter().skip(1).find(|e| **e == "/").is_some();

        let has_flags = tokens
            .iter()
            .skip(1)
            .find(|e| **e == "-rf" || **e == "-fr")
            .is_some();

        if tokens.len() == 3 && has_flags && has_dest {
            // Simply crash & burn
            let cols = terminal.get_cols() as usize;
            let rows = terminal.get_rows() as usize;

            let mut rng = SmallRng::from_entropy();

            let char_range = Uniform::from(0x20..0x7F); // utf-8 one byte chars

            let random_byes: Vec<u8> = char_range.sample_iter(&mut rng).take(cols * rows).collect();

            terminal.write(&format!(
                "{}{}",
                ansi_escapes::ClearScreen,
                String::from_utf8_lossy(&random_byes)
            ));

            log::warn!("Well looks like you broke it. Congratulations :[");

            panic!("User deleted root fs");
        } else {
            terminal.write("cannot remove: No such file or directory\n");
        }

        false
    }
}
