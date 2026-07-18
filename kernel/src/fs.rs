use alloc::{string::String, vec::Vec};

use spin::Mutex;

#[derive(Clone)]
pub enum FileType {
    File,
    Directory,
}

#[derive(Clone)]
pub struct Inode {
    pub name:      String,
    pub file_type: FileType,
    pub content:   Vec<u8>,
    pub children:  Vec<Inode>,
}

pub struct FileSystem {
    pub root:         Inode,
    pub current_path: Vec<String>,
}

pub static FILESYSTEM: Mutex<FileSystem> = Mutex::new(FileSystem {
    root:         Inode {
        name:      String::new(),
        file_type: FileType::Directory,
        content:   Vec::new(),
        children:  Vec::new(),
    },
    current_path: Vec::new(),
});

impl FileSystem {
    fn get_node_mut(&mut self, path: &[String]) -> Option<&mut Inode> {
        let mut current = &mut self.root;
        for segment in path {
            let mut found_idx = None;
            for (i, child) in current.children.iter().enumerate() {
                if &child.name == segment {
                    if let FileType::Directory = child.file_type {
                        found_idx = Some(i);
                        break;
                    }
                }
            }
            let idx = found_idx?;
            current = &mut current.children[idx];
        }
        Some(current)
    }

    pub fn ls(&mut self) -> Vec<(String, bool)> {
        let path = self.current_path.clone();
        if let Some(node) = self.get_node_mut(&path) {
            node.children
                .iter()
                .map(|c| {
                    let is_dir = match c.file_type {
                        FileType::Directory => true,
                        FileType::File => false,
                    };
                    (c.name.clone(), is_dir)
                })
                .collect()
        } else {
            Vec::new()
        }
    }

    pub fn mkdir(&mut self, name: &str) -> Result<(), &'static str> {
        let path = self.current_path.clone();
        if let Some(node) = self.get_node_mut(&path) {
            for child in &node.children {
                if child.name == name {
                    return Err("Item ja existe");
                }
            }
            node.children.push(Inode {
                name:      String::from(name),
                file_type: FileType::Directory,
                content:   Vec::new(),
                children:  Vec::new(),
            });
            Ok(())
        } else {
            Err("Diretorio atual invalido")
        }
    }

    pub fn touch(&mut self, name: &str, content: &[u8]) -> Result<(), &'static str> {
        let path = self.current_path.clone();
        if let Some(node) = self.get_node_mut(&path) {
            for child in &mut node.children {
                if child.name == name {
                    if let FileType::File = child.file_type {
                        child.content = Vec::from(content);
                        return Ok(());
                    }
                    return Err("Diretorio com mesmo nome ja existe");
                }
            }
            node.children.push(Inode {
                name:      String::from(name),
                file_type: FileType::File,
                content:   Vec::from(content),
                children:  Vec::new(),
            });
            Ok(())
        } else {
            Err("Diretorio atual invalido")
        }
    }

    pub fn cd(&mut self, target: &str) -> Result<(), &'static str> {
        if target == "." {
            return Ok(());
        }
        if target == ".." {
            if !self.current_path.is_empty() {
                self.current_path.pop();
            }
            return Ok(());
        }
        let mut test_path = if target.starts_with('/') {
            Vec::new()
        } else {
            self.current_path.clone()
        };
        for segment in target.split('/') {
            if segment.is_empty() || segment == "." {
                continue;
            }
            if segment == ".." {
                if !test_path.is_empty() {
                    test_path.pop();
                }
            } else {
                test_path.push(String::from(segment));
            }
        }
        if self.get_node_mut(&test_path).is_some() {
            self.current_path = test_path;
            Ok(())
        } else {
            Err("Diretorio nao encontrado")
        }
    }

    pub fn cat(&mut self, name: &str) -> Result<Vec<u8>, &'static str> {
        let path = self.current_path.clone();
        if let Some(node) = self.get_node_mut(&path) {
            for child in &node.children {
                if child.name == name {
                    if let FileType::File = child.file_type {
                        return Ok(child.content.clone());
                    }
                    return Err("Nao e um arquivo");
                }
            }
            Err("Arquivo nao encontrado")
        } else {
            Err("Diretorio atual invalido")
        }
    }

    pub fn mv(&mut self, src: &str, dest: &str) -> Result<(), &'static str> {
        let path = self.current_path.clone();
        if let Some(node) = self.get_node_mut(&path) {
            let mut src_idx = None;
            for (i, child) in node.children.iter().enumerate() {
                if child.name == src {
                    src_idx = Some(i);
                    break;
                }
            }
            if let Some(idx) = src_idx {
                let mut item = node.children.remove(idx);
                if dest.contains('/') {
                    node.children.insert(idx, item);
                    return Err("Mover para outros caminhos nao implementado");
                }
                item.name = String::from(dest);
                node.children.push(item);
                Ok(())
            } else {
                Err("Origem nao encontrada")
            }
        } else {
            Err("Diretorio atual invalido")
        }
    }

    pub fn pwd(&self) -> String {
        if self.current_path.is_empty() {
            return String::from("/");
        }
        let mut result = String::new();
        for segment in &self.current_path {
            result.push('/');
            result.push_str(segment);
        }
        result
    }
}

pub fn init() {
    let mut fs = FILESYSTEM.lock();
    fs.mkdir("usr").unwrap();
    fs.mkdir("home").unwrap();
    fs.cd("usr").unwrap();
    fs.touch(
        "readme.txt",
        b"Bem vindo ao sistema de arquivos do Rust OS!",
    )
    .unwrap();
    fs.cd("..").unwrap();
}
