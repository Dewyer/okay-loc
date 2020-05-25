use regex::Regex;
use std::path::PathBuf;

pub struct Excludor<'r>
{
    extra_regex:Option<Regex>,
    gitignore_files:Vec<gitignore::File<'r>>
}

impl<'r> Excludor<'r>
{
    pub fn new(extra_regex:Option<Regex>) -> Self
    {
        Self
        {
            extra_regex,
            gitignore_files:vec![],
        }
    }

    pub fn append_gitignore(&mut self,gipath:&PathBuf) -> Result<(),anyhow::Error>
    {
        println!("Added gitignore !, {}",gipath.to_str().unwrap());
        let leak:&'r PathBuf = Box::leak(Box::new(gipath.clone()));
        self.gitignore_files.push(gitignore::File::new(leak)?);

        Ok(())
    }

    pub fn is_file_included(&self,path:&PathBuf) -> bool
    {
        if let Some(regex) = &self.extra_regex
        {
            if let Some(path_ss) = path.to_str()
            {
                if !regex.is_match(path_ss)
                {
                    return false;
                }
            }
        }
        for git in self.gitignore_files.iter()
        {
            if git.is_excluded(path).unwrap()
            {
                return false;
            }
        }

        true
    }
}