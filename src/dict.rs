use std::ptr::eq;

#[derive(Debug, Clone)]
struct Dict<TK, TV> {
    pub key: Vec<TK>,
    pub value: Vec<TV>,
}

#[allow(unused)]
impl<TK, TV> Dict<TK, TV>
where
    TK: Default + Clone + Sized,
    TV: Default + Clone + Sized,
{
    pub fn new(cap: i32) -> Self {
        Self {
            key: Vec::new(),
            value: Vec::new(),
        }
    }

    pub fn add(&mut self, key: TK, value: TV) {
        self.key.push(key);
        self.value.push(value);
    }

    pub fn add_range(&mut self, src_key: Vec<TK>, src_value: Vec<TV>, begin: usize, end: usize) {
        for i in begin..end {
            self.key.push(src_key[i].clone());
            self.value.push(src_value[i].clone());
        }
    }

    pub fn remove(&mut self, i: usize) {
        self.key.remove(i);
        self.value.remove(i);
    }

    pub fn rm(&mut self, key: &TK) {
        let mut i: usize = 0;
        for k in self.key.iter() {
            if eq(k, key) {
                self.key.remove(i);
                self.value.remove(i);
                break;
            }
            i += 1;
        }
    }

    pub fn remove_range(&mut self, src_key: Vec<TK>, src_value: Vec<TV>, begin: usize, end: usize) {
        for i in begin..end {
            self.key.remove(i);
            self.value.remove(i);
        }
    }
}
