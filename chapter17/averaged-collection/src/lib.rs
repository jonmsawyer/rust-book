pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    pub fn new() -> AveragedCollection {
        AveragedCollection {
            list: vec![],
            average: 0.0f64,
        }
    }
    
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn average_is_correct() {
        let mut avg = AveragedCollection::new();
        
        avg.add(1);
        avg.add(2);
        avg.add(3);
        
        assert_eq!(2.0f64, avg.average());
        
        avg.add(4);
        
        assert_eq!(2.5f64, avg.average());
    }
    
    #[test]
    fn correctly_removes_item() {
        let mut avg = AveragedCollection::new();
        
        avg.add(1);
        avg.add(2);
        avg.add(3);
        
        assert_eq!(2.0f64, avg.average());
        
        avg.add(4);
        
        assert_eq!(2.5f64, avg.average());
        
        if let Some(num) = avg.remove() {
            assert_eq!(num, 4);
            assert_eq!(2.0f64, avg.average());
        }
        else {
            panic!("Some(num) != 4");
        }
    }
}
