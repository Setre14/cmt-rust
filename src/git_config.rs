use std::path::Path;
use std::fs;
use clap::{Subcommand};
use crate::util::command_line::CommandLine;

use crate::config::app_config;
use crate::config::env_config;
use crate::config::pkgm_config;
use crate::util::exec::Exec;
use crate::config::config_reader;

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

        /// Branch to checkout otherwise default branch is used
        #[arg(short, long)]
        track: Option<String>,

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

    /// Pull latest changes from origin
    Pull {},

    /// Remove duplicates in global and system config
    Cleanup {},

    /// Open git repo in VS Code
    Code {},
    /// Open git repo in NVim
    Nvim {},
}

pub fn handle_command(command: &Command) {
    match command {
        Command::Init { url, dest, branch, track, force } => {
            init(url, dest, branch, track, *force);
        },
        Command::Update { message } => {
            update(message);
        },
        Command::Pull {} => {
            pull();
        },
        Command::Cleanup {} => {
            cleanup();
        },
        Command::Code {} => {
            open_code();
        },
        Command::Nvim {} => {
            open_nvim();
        }
    }
}

pub fn init(url: &String, dest: &Option<String>, branch: &Option<String>, track: &Option<String>, force: bool) {
    let mut app_conf = app_config::get_conf();

    let git_config_dir = dest.clone().unwrap_or(app_conf.git_config_dir.clone());
    let git_branch = branch.clone().unwrap_or("main".to_string());
    let config_track = track.clone().unwrap_or(app_conf.git_branch.clone());

    log::info!("git config init: {}, {}", git_config_dir, git_branch);

    if Path::new(&git_config_dir).is_dir() {
        if force {
            let _ = fs::remove_dir_all(Path::new(&git_config_dir));
        } else {
            log::info!("dir exists");
            std::process::exit(1);
        }
    }

    let git_clone = CommandLine::create("git", ["clone", url, &git_config_dir].to_vec());
    Exec::status(&git_clone);

    let git_checkout = CommandLine::create("git", ["checkout", &git_branch].to_vec());
    let result = Exec::status_in_dir(&git_checkout, &git_config_dir);

    if !result {
        let git_create_branch = CommandLine::create("git", ["checkout", "-b", &git_branch].to_vec());
        Exec::status_in_dir(&git_create_branch, &git_config_dir);
    } 
    
    app_conf.git_clone_url = url.clone();
    app_conf.git_config_dir = git_config_dir.clone();
    app_conf.git_branch = git_branch.clone();
    app_conf.track = config_track.clone();

    config_reader::save_conf(&app_conf);
}

pub fn update(message: &Option<String>) {
    let app_conf = app_config::get_conf();

    let commit_message = message.clone().unwrap_or("Cmt: Automatic update".to_string());

    log::debug!("Commit message for update: {}", commit_message);

    let git_add = CommandLine::create("git", ["add", "."].to_vec());
    Exec::status_in_dir(&git_add, &app_conf.git_config_dir);

    let git_commit = CommandLine::create("git", ["commit", "-m", &commit_message].to_vec());
    let result = Exec::status_in_dir(&git_commit, &app_conf.git_config_dir);
    if result {
        let git_push = CommandLine::create("git", ["push"].to_vec());
        Exec::status_in_dir(&git_push, &app_conf.git_config_dir);
    }
}

pub fn auto_update(message: &Option<String>) {
    let app_conf = app_config::get_conf();

    if app_conf.git_auto_sync {
        update(message);
    }
}

pub fn pull() {
    let app_conf = app_config::get_conf();

    let git_add = CommandLine::create("git", ["pull"].to_vec());
    let result = Exec::status_in_dir(&git_add, &app_conf.git_config_dir);

    if !result {
        log::error!("Could not pull {}", &app_conf.git_config_dir);
        panic!();
    }
}

pub fn auto_pull() {
    let app_conf = app_config::get_conf();

    if app_conf.git_auto_sync {
        pull();
    }
}

pub fn cleanup() {
    env_config::cleanup();
    pkgm_config::cleanup();
}

pub fn open_code() {
    let app_conf = app_config::get_conf();
    let git_config_dir = app_conf.git_config_dir;

    let code = CommandLine::create("code", [git_config_dir.as_str()].to_vec());
    Exec::status(&code);
}


pub fn open_nvim() {
    let app_conf = app_config::get_conf();
    let git_config_dir = app_conf.git_config_dir;


    let nvim = CommandLine::create("nvim", [git_config_dir.as_str()].to_vec());
    Exec::status(&nvim);
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