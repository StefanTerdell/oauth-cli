use crate::cli::Cli;
use crate::APP_NAME;
use anyhow::*;
use clap::CommandFactory;
use clap_complete::{generate as clap_generate, Shell};
use std::fs::{self, create_dir_all};
use std::io::stdout;

pub fn generate(shell: Option<Shell>) -> Result<()> {
    let shell = shell.unwrap_or(Shell::from_env().context("Unable to load shell from ENV")?);

    clap_generate(shell, &mut Cli::command(), APP_NAME, &mut stdout());

    Ok(())
}

pub fn install() -> Result<()> {
    let shell = Shell::from_env().context("Unable to get shell from env")?;
    let home_dir = dirs::home_dir().context("Could not find home directory")?;

    let completions_dir = match &shell {
        Shell::Bash => home_dir.join(".bash_completion.d"),
        Shell::Zsh => home_dir.join(".zsh/completions"),
        Shell::Fish => home_dir.join(".config/fish/completions"),
        Shell::PowerShell => home_dir.join("Documents/PowerShell/Scripts"),
        _ => home_dir.join("completions"),
    };

    create_dir_all(&completions_dir).context("Ensuring dir structure")?;

    let file_name = match &shell {
        Shell::Zsh => format!("_{APP_NAME}"),
        _ => APP_NAME.to_string(),
    };

    let file_path = completions_dir.join(file_name);

    let mut file = fs::File::create(&file_path).context("Creating completion file")?;

    clap_generate(shell, &mut Cli::command(), APP_NAME, &mut file);

    if shell == Shell::Bash {
        let bash_rc_path = home_dir.join(".bashrc");
        let bash_rc_content = fs::read_to_string(&bash_rc_path).context("Reading .bashrc")?;

        let source_line = format!("source {path}", path = file_path.display());

        if !bash_rc_content
            .lines()
            .any(|line| line.trim() == source_line)
        {
            fs::write(&bash_rc_path, format!("{bash_rc_content}\n\n{source_line}"))
                .context("Writing .bashrc")?;
        };

        println!(
            "Done! Run `$ source {path}` or open a new terminal.",
            path = bash_rc_path.display()
        );
    } else if shell == Shell::Zsh {
        let zsh_rc_path = home_dir.join(".zshrc");
        let zsh_rc_content = fs::read_to_string(&zsh_rc_path).context("Reading .zshrc")?;

        let fpath_line = "fpath=(~/.zsh/completions $fpath)";
        let autoload_line = "autoload -U compinit && compinit";

        let installed = {
            let mut iter = zsh_rc_content.lines().peekable();

            let mut found = false;
            while let Some(line) = iter.next() {
                if line.trim() == fpath_line && iter.peek() == Some(&autoload_line) {
                    found = true;
                    break;
                }
            }

            found
        };

        if !installed {
            fs::write(
                &zsh_rc_path,
                format!("{zsh_rc_content}\n\n{fpath_line}\n{autoload_line}"),
            )
            .context("Writing .zshrc")?;
        };

        println!(
            "Done! Run `$ source {path}` or open a new terminal.",
            path = zsh_rc_path.display()
        );
    } else {
        println!("Autocompletion script written to {path} but can't be automatically installed on {shell}, only on zsh and bash.", path = file_path.display());
    }

    Ok(())
}
