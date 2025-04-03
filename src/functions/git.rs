use chrono::{Duration, Utc};
use git2::{Error, Repository};
use std::{env, process::Command};

#[derive(PartialEq)]
pub enum Method {
    COMMIT,
    PUSH,
}

pub fn commit_or_push(method: Method, msg: Option<&str>) -> Result<(), Error> {
    let t = Utc::now() + Duration::hours(8);
    let t_str = t.format("%Y-%m-%d %H:%M:%S").to_string();

    let repo_path = env::current_dir().unwrap();
    let repo = Repository::open(repo_path)?;
    let head = repo.head()?.peel_to_commit()?;
    let binding = repo.head()?;
    let branch = binding.shorthand().unwrap_or("unknown");

    let mut index = repo.index()?;
    let status = repo.statuses(None)?;

    let is_dirty = status.iter().any(|s| {
        s.status().is_wt_new() || s.status().is_wt_modified() || s.status().is_index_modified()
    });

    if is_dirty {
        if method == Method::COMMIT {
            index.add_all(["*"].iter(), git2::IndexAddOption::DEFAULT, None)?;
            index.write()?;

            let tree_id = index.write_tree()?;
            let tree = repo.find_tree(tree_id)?;

            let sig = repo.signature()?;
            let commit_msg = msg.unwrap_or("Auto commit");

            let commit_id = repo.commit(Some("HEAD"), &sig, &sig, commit_msg, &tree, &[&head])?;

            let commit_id_hex = &commit_id.to_string()[..7];
            println!("{} {} {}", t_str, &commit_id_hex[..7], commit_msg);
        }
    }

    if method == Method::PUSH {
        Command::new("git")
            .args(["push", "origin", "main"])
            .output()
            .expect("Failed to execute git push");

        println!("{} Changes were pushed to the {} branch.", t_str, branch);
    }

    Ok(())
}
