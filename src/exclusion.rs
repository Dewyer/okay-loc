use regex::Regex;
use std::path::PathBuf;

pub struct Excludor<'r>
{
    extra_regex:Option<Regex>,
    gitignore_files:Vec<gitignore::File<'r>>,
    heuristic_regex:Regex,
    disallowed_ext:Vec<&'static str>,
    verbose:bool
}

impl<'r> Excludor<'r>
{
    pub fn new(extra_regex:Option<Regex>,verbose:bool) -> Self
    {
        Self
        {
            extra_regex,
            gitignore_files:vec![],
            heuristic_regex: Regex::new(r#"(^\..*)|(node_modules)"#).expect("Invalid heuristic regex."),
            disallowed_ext: vec!["json","toml"],
            verbose
        }
    }

    pub fn append_gitignore(&mut self,gipath:&PathBuf) -> Result<(),anyhow::Error>
    {
        if self.verbose
        {
            println!("Added gitignore !, {}",gipath.to_str().unwrap());
        }

        let leak:&'r PathBuf = Box::leak(Box::new(gipath.clone()));
        self.gitignore_files.push(gitignore::File::new(leak)?);

        Ok(())
    }

    pub fn is_file_included(&self,path:&PathBuf) -> bool
    {
        if let Some(extension) = path.extension()
        {
            let extension = extension.to_str().map_or("",|kk| kk);
            if self.disallowed_ext.iter().any(|dis| *dis == extension)
            {
                return false;
            }
        }

        let name = path.file_name().unwrap().to_str().unwrap();
        if self.heuristic_regex.is_match(name)
        {
            return false;
        }

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