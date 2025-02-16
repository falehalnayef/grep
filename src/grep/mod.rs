use colored::Colorize;



pub struct Grep{
    pattern: String,
    data: String
}


impl Grep {
    

    pub fn new(pattern: String, data: String) -> Self{

        Self {
            pattern,
            data
        }
    }

    pub fn match_pattern(&self) -> Option<String>{

        if self.data.contains(&self.pattern){
            return Some(self.data.replace(
                &self.pattern,
                &self.pattern.red().bold().to_string(),
            ));
        }
        None
      
    }
}