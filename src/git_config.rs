use std::path::Path;
use std::fs;

use crate::config::app;
use crate::config::base;
use crate::util::exec;

pub fn init(url: &String, dest: &Option<String>, branch: &Option<String>, force: bool) {
    let mut app_conf = app::get_conf();

    let git_config_dir;
    match dest {
        Some(x) => git_config_dir = x.clone(),
        None => git_config_dir = app_conf.git_config_dir.clone(),
    }

    let git_branch;
    match branch {
        Some(x) => git_branch = x.clone(),
        None => git_branch = app_conf.git_branch.clone(),
    }

    println!("git config init: {}, {}", git_config_dir, git_branch);

    if Path::new(&git_config_dir).is_dir() {
        if force {
            let _ = fs::remove_dir_all(Path::new(&git_config_dir));
        } else {
            println!("dir exists");
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