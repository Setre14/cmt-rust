use std::path::Path;
use std::fs;
use clap::{Subcommand};

use crate::config::app;
use crate::config::base;
use crate::util::exec;

#[derive(Subcommand)]
pub enum Command {
    /// Init app config and clone git config
    Init {
        /// Git clone url
        url: String,

        /// Destination for git clone, default: ~/.config/cmt-rust
        #[arg(short, long)]
        dest: Option<String>,

        /// Branch to checkout otherwise default branch is used
        #[arg(short, long)]
        branch: Option<String>,

        /// Remove folder if destination already exists
        #[arg(short, long)]
        force: bool,
    },

    /// Commit and push all changes in the git config
    Update {
        /// Message for the git commit
        #[arg(short, long)]
        message: Option<String>,
    },

    /// Open git repo in VS Code
    Code {},
    /// Open git repo in NVim
    Nvim {},
}

pub fn handle_command(command: &Command) {
    match command {
        Command::Init { url, dest, branch, force } => {
            init(url, dest, branch, *force);
        },
        Command::Update { message } => {
            update(message);
        },
        Command::Code {} => {
            open_code();
        },
        Command::Nvim {} => {
            open_nvim();
        }
    }
}

pub fn init(url: &String, dest: &Option<String>, branch: &Option<String>, force: bool) {
    let mut app_conf = app::get_conf();

    let git_config_dir = dest.clone().unwrap_or(app_conf.git_config_dir.clone());
    // match dest {
    //     Some(x) => git_config_dir = x.clone(),
    //     None => git_config_dir = app_conf.git_config_dir.clone(),
    // }

    let git_branch = branch.clone().unwrap_or(app_conf.git_branch.clone());
    // match branch {
    //     Some(x) => git_branch = x.clone(),
    //     None => git_branch = app_conf.git_branch.clone(),
    // }

    log::info!("git config init: {}, {}", git_config_dir, git_branch);

    if Path::new(&git_config_dir).is_dir() {
        if force {
            let _ = fs::remove_dir_all(Path::new(&git_config_dir));
        } else {
            log::info!("dir exists");
            std::process::exit(1);
        }
    }

    exec::status("git", ["clone", url, &git_config_dir]);

    exec::status_in_dir("git", ["checkout", "-b", &git_branch], &git_config_dir);
    
    app_conf.git_clone_url = url.clone();
    app_conf.git_config_dir = git_config_dir.clone();
    app_conf.git_branch = git_branch.clone();

    base::save_conf(&app_conf);
}

pub fn update(message: &Option<String>) {
    let app_conf = app::get_conf();

    let commit_message = message.clone().unwrap_or("Cmt: Automatic update".to_string());

    log::debug!("Commit message for update: {}", commit_message);

    exec::status_in_dir("git", ["add", "."], &app_conf.git_config_dir);
    let result = exec::status_in_dir("git", ["commit", "-m", &commit_message], &app_conf.git_config_dir);
    if result {
        exec::status_in_dir("git", ["push"], &app_conf.git_config_dir);
    }
}

pub fn open_code() {
    let app_conf = app::get_conf();
    let git_config_dir = app_conf.git_config_dir;

    exec::status("code", [git_config_dir.as_str()]);
}


pub fn open_nvim() {
    let app_conf = app::get_conf();
    let git_config_dir = app_conf.git_config_dir;

    exec::status("nvim", [git_config_dir.as_str()]);
}

// require 'thor'

// require_relative 'config'
// require_relative '../util/log'

// class GitConfig < Thor
//   desc 'init', 'Init'
//   option :force, type: :boolean, default: false
//   def init(repo)
//     force = options['force']

//     if File.exist? Config::CONFIG_DIR
//       if force
//         FileUtils.rm_rf(Config::CONFIG_DIR)
//       else
//         Log::LOGGER.error("Config already exists at '#{Config::CONFIG_DIR}'")
//         abort("Config already exists at '#{Config::CONFIG_DIR}'")
//       end
//     end

//     `git clone #{repo} #{Config::CONFIG_DIR}`
//   end

//   desc 'update', 'update'
//   option :message, default: 'Update config'
//   def update
//     message = options['message']
//     Dir.chdir(Config::CONFIG_DIR) do
//       changes = `git status --porcelain=v1`
//       num_changes = changes.split.length

//       if num_changes.zero?
//         Log::LOGGER.error("No changes to commit in '#{Config::CONFIG_DIR}'")
//         abort("No changes to commit in '#{Config::CONFIG_DIR}'")
//       else
//         Log::LOGGER.info('Commit changes')
//         system 'git', 'add', '.'
//         system 'git', 'commit', '-m', "'#{message}'"
//         system 'git', 'push'
//       end
//     end
//   end

//   desc 'status', "Show git status in '#{Config::CONFIG_DIR}'"
//   def status(*args)
//     Dir.chdir(Config::CONFIG_DIR) do
//       system 'git', 'status', *args
//     end
//   end

//   desc 'pull', "Pull newest changes in '#{Config::CONFIG_DIR}'"
//   def pull(*args)
//     Dir.chdir(Config::CONFIG_DIR) do
//       system 'git', 'pull', *args
//     end
//   end

//   desc 'exec', "Execute any git command in '#{Config::CONFIG_DIR}'"
//   def exec(*args)
//     Dir.chdir(Config::CONFIG_DIR) do
//       system 'git', *args
//     end
//   end

//   desc 'code', "Open '#{Config::CONFIG_DIR}' in code"
//   def code(*args)
//     Log::LOGGER.info("Open '#{Config::CONFIG_DIR}' in code")
//     Dir.chdir(Config::CONFIG_DIR) do
//       system 'code', '.', *args
//     end
//   end

//   desc 'nvim', "Open '#{Config::CONFIG_DIR}' in nvim"
//   def nvim(*args)
//     Log::LOGGER.info("Open '#{Config::CONFIG_DIR}' in nvim")
//     Dir.chdir(Config::CONFIG_DIR) do
//       system 'nvim', '.', *args
//     end
//   end
// end