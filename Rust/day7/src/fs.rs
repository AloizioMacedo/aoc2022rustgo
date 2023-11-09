use petgraph::{
    prelude::{DiGraph, NodeIndex},
    visit::EdgeRef,
};
use thiserror::Error;

pub struct FileSystem<'a> {
    pub fs: DiGraph<FileEntry<'a>, ()>, // Note that multiple entries can have the same name.
    pub current_path: NodeIndex<u32>,
}

impl Default for FileSystem<'_> {
    fn default() -> Self {
        let mut fs = DiGraph::new();
        let current_path = fs.add_node(FileEntry::Folder("/"));

        Self { fs, current_path }
    }
}

impl FileSystem<'_> {
    pub fn move_up(&mut self) -> Result<(), MovementError> {
        match self
            .fs
            .edges_directed(self.current_path, petgraph::Direction::Incoming)
            .next()
        {
            Some(edge) => {
                self.current_path = edge.source();
                Ok(())
            }
            None => Err(MovementError),
        }
    }

    pub fn move_down(&mut self, name: &str) -> Result<(), MovementError> {
        self.current_path = self
            .fs
            .edges_directed(self.current_path, petgraph::Direction::Outgoing)
            .find(|edge| {
                let tgt = edge.target();
                self.fs[tgt].get_name() == name
            })
            .ok_or(MovementError)?
            .target();

        Ok(())
    }

    pub fn get_dir_sizes(&self) -> Vec<usize> {
        self.fs
            .node_indices()
            .filter(|idx| matches!(self.fs[*idx], FileEntry::Folder(_)))
            .map(|idx| self.get_size(idx))
            .collect()
    }

    fn get_size(&self, idx: NodeIndex) -> usize {
        match &self.fs[idx] {
            FileEntry::File(file) => file.size,
            FileEntry::Folder(_) => self
                .fs
                .edges_directed(idx, petgraph::Direction::Outgoing)
                .map(|e| {
                    let tgt = e.target();
                    self.get_size(tgt)
                })
                .sum(),
        }
    }
}

#[derive(Error, Debug)]
#[error("Movement error")]
pub struct MovementError;

pub enum FileEntry<'a> {
    File(File<'a>),
    Folder(&'a str),
}

impl FileEntry<'_> {
    fn get_name(&self) -> &str {
        match self {
            FileEntry::File(file) => file.name,
            FileEntry::Folder(folder_name) => folder_name,
        }
    }
}

pub struct File<'a> {
    pub size: usize,
    pub name: &'a str,
}
