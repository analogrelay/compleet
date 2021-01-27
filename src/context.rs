use std::{borrow::Cow, error::Error, path::PathBuf};

#[derive(Debug)]
pub struct CompletionContext
{
    working_dir: PathBuf,
    git_dir: PathBuf
}

impl CompletionContext {
  pub fn new(working_dir: PathBuf, git_dir: PathBuf) -> CompletionContext {
    CompletionContext { working_dir, git_dir }
  }

  pub fn complete_line<'a>(&'a self, line: &str, position: usize) -> Result<impl IntoIterator<Item = Cow<'a, str>>, Box<dyn Error>> {
    println!("completing '{}' at position {}", line, position);
    Ok(vec![])
  }
}