use std::collections::HashMap;
use std::iter::once;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Directory<'a> {
  dirs: HashMap<&'a str, Directory<'a>>,
  files: HashMap<&'a str, usize>
}

impl<'a> Directory<'a> {
	pub fn new() -> Self {
    Self {
      dirs: HashMap::new(),
      files: HashMap::new(),
    }
  }

	pub fn add_dir(&mut self, name: &'a str) {
    self.dirs.insert(name, Self::new());
  }

  pub fn add_file(&mut self, name: &'a str, size: usize) {
    self.files.insert(name, size);
  }

	pub fn dir(&self, path: &[&'a str]) -> Option<&Self> {
		if let Some(dir_name) = path.first() {
			self.dirs.get(dir_name).and_then(|dir| dir.dir(&path[1..]))
		} else {
			Some(self)
		}
	}

	pub fn dir_mut(&mut self, path: &[&'a str]) -> Option<&mut Self> {
		if let Some(dir_name) = path.first() {
			self.dirs.get_mut(dir_name).and_then(|dir| dir.dir_mut(&path[1..]))
		} else {
			Some(self)
		}
	}

  pub fn size(&self) -> usize {
    let files: usize = self.files.iter().map(|(_, size)| *size).sum();
    let dirs: usize = self.dirs.iter().map(|(_, dir)| dir.size()).sum();
    files + dirs
  }

	pub fn dirs(&self) -> impl Iterator<Item = &Directory<'a>> {
		self.dirs.iter().map(|(_, dir)| dir)
	}

	pub fn recur(&'a self) -> Box<dyn Iterator<Item = &Directory<'a>> + 'a>  {
		Box::new(once(self).chain(self.dirs().flat_map(|dir| dir.recur())))
	}
}