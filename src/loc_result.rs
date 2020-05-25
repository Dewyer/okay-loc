use std::ops;

pub struct LocResult
{
    pub source:i64,
    pub comment:i64,
    pub todo_count:i64
}

impl LocResult
{
    pub fn new()->Self
    {
        Self
        {
            source:0,
            comment:0,
            todo_count:0
        }
    }

    pub fn get_all(&self) -> i64
    {
        self.comment + self.source
    }
}

impl ops::Add<LocResult> for LocResult
{
    type Output = LocResult;

    fn add(self, rhs: LocResult) -> Self::Output {
        LocResult
        {
            source:self.source + rhs.source,
            comment:self.comment + rhs.comment,
            todo_count:self.todo_count + rhs.todo_count
        }
    }
}