use anyhow::Result;
use clap::{Args, CommandFactory};
use clap_complete::{generate, shells::{Bash, Elvish, Fish, PowerShell, Zsh}};
use std::io;
use console::style;

#[derive(Args)]
pub struct CompletionCommand {
    /// Shell to generate completion for
    #[arg(value_enum)]
    pub shell: ShellArg,
}

#[derive(clap::ValueEnum, Clone)]
pub enum ShellArg {
    Bash,
    Zsh,
    Fish,
    PowerShell,
    Elvish,
}

impl CompletionCommand {
    pub fn execute(&self) -> Result<()> {
        use crate::cli::Cli;

        let mut cmd = Cli::command();

        match self.shell {
            ShellArg::Bash => {
                generate(Bash, &mut cmd, "oxide", &mut io::stdout());
                println!("\n{}", style("Add to your ~/.bashrc:").cyan());
                println!("  source <(oxide completion bash)");
            }
            ShellArg::Zsh => {
                generate(Zsh, &mut cmd, "oxide", &mut io::stdout());
                println!("\n{}", style("Add to your ~/.zshrc:").cyan());
                println!("  eval \"$(oxide completion zsh)\"");
            }
            ShellArg::Fish => {
                generate(Fish, &mut cmd, "oxide", &mut io::stdout());
                println!("\n{}", style("Add to your ~/.config/fish/config.fish:").cyan());
                println!("  oxide completion fish | source");
            }
            ShellArg::PowerShell => {
                generate(PowerShell, &mut cmd, "oxide", &mut io::stdout());
                println!("\n{}", style("Add to your PowerShell profile:").cyan());
                println!("  oxide completion powershell | Out-String | Invoke-Expression");
            }
            ShellArg::Elvish => {
                generate(Elvish, &mut cmd, "oxide", &mut io::stdout());
                println!("\n{}", style("Add to your elvish rc.elv:").cyan());
                println!("  eval (oxide completion elvish | slurp)");
            }
        }

        Ok(())
    }
}
