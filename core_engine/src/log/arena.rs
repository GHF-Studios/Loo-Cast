use std::{
    sync::atomic::{AtomicU32, Ordering},
    sync::Arc,
};

use dashmap::DashMap;
use parking_lot::{Mutex, RwLock, RwLockReadGuard};

use crate::functions::now_since_start_ns;

/* ---------- node + type system ---------- */

pub type NodeIdx = u32;
const NONE: u32 = 0;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Level {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

#[derive(Clone, Debug)]
pub struct Log {
    pub ts:  u64,
    pub lvl: Level,
    pub msg: Arc<str>,
}

/* ---------- span hierarchy ---------- */

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum LocKind {
    Crate,
    Module,
    File,
    Line,
    SubModule,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TreeKind {
    Span,
    Loc(LocKind),
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum FileViewMode {
    Lines,
    SubModules,
}

struct Node {
    kind: TreeKind,
    name_tok: u32, // for all except Line
    line: u32,     // Line nodes only
    col: u16,      // Line nodes only
    parent: Option<NodeIdx>,
    first_child: AtomicU32,
    next_sib: AtomicU32,
    logs: RwLock<Vec<Log>>, // only Line nodes fill this
}

impl Node {
    fn new(kind: TreeKind, name_tok: u32, parent: Option<NodeIdx>) -> Self {
        Self {
            kind,
            name_tok,
            line: 0,
            col: 0,
            parent,
            first_child: AtomicU32::new(NONE),
            next_sib: AtomicU32::new(NONE),
            logs: RwLock::new(Vec::new()),
        }
    }

    fn new_line(parent: Option<NodeIdx>, line: u32, col: u16) -> Self {
        Self {
            kind: TreeKind::Loc(LocKind::Line),
            name_tok: 0,
            line,
            col,
            parent,
            first_child: AtomicU32::new(NONE),
            next_sib: AtomicU32::new(NONE),
            logs: RwLock::new(Vec::new()),
        }
    }
}

/* ---------- interned strings ---------- */

#[derive(Default)]
pub struct Interns {
    map: DashMap<Arc<str>, u32>,
    seq: AtomicU32,
}

impl Interns {
    pub fn get(&self, s: &str) -> u32 {
        if let Some(tok) = self.map.get(s).map(|e| *e) {
            return tok;
        }
        let tok = self.seq.fetch_add(1, Ordering::Relaxed);
        self.map.insert(Arc::<str>::from(s), tok);
        tok
    }

    pub fn lookup(&self, s: &str) -> Option<u32> {
        self.map.get(s).map(|e| *e)
    }

    pub fn str_for(&self, tok: u32) -> Arc<str> {
        self.map
            .iter()
            .find(|kv| *kv.value() == tok)
            .map(|kv| kv.key().clone())
            .unwrap_or_else(|| Arc::<str>::from("<unknown>"))
    }
}

/* ---------- the dual arena ---------- */

pub struct Arena {
    vec: Mutex<Vec<Node>>,
    ints: Interns,
    by_file: DashMap<u32, Vec<NodeIdx>>, // file-tok → [line-node]
}

impl Arena {
    pub fn new() -> Self {
        Self {
            vec: Mutex::new(Vec::with_capacity(2048)),
            ints: Interns::default(),
            by_file: DashMap::new(),
        }
    }

    /// Inserts a log message into both trees.
    #[allow(clippy::too_many_arguments)]
    pub fn insert_log(
        &self,
        span_path: &[&str],
        crate_name: &str,
        module_path: &[&str],
        file_path: &str,
        line: u32,
        col: u16,
        lvl: Level,
        msg: impl Into<Arc<str>>,
    ) {
        let mut span_parent = None;
        for seg in span_path {
            span_parent = Some(self.child(span_parent, TreeKind::Span, self.ints.get(seg)));
        }

        let mut loc_parent = Some(self.child(None, TreeKind::Loc(LocKind::Crate), self.ints.get(crate_name)));
        for seg in module_path {
            loc_parent = Some(self.child(loc_parent, TreeKind::Loc(LocKind::Module), self.ints.get(seg)));
        }

        let file_tok = self.ints.get(file_path);
        loc_parent = Some(self.child(loc_parent, TreeKind::Loc(LocKind::File), file_tok));
        let line_node = self.child_line(loc_parent, line, col);

        self.by_file.entry(file_tok).or_default().push(line_node);

        let log = Log {
            ts: now_since_start_ns(),
            lvl,
            msg: msg.into(),
        };

        self.node(line_node).logs.write().push(log);
    }

    
    pub fn insert_span(&self, span_path: &[&str]) {
        let mut parent = None;
        for seg in span_path {
            parent = Some(self.child(parent, TreeKind::Span, self.ints.get(seg)));
        }
    }

    /* ---------- traversal + lookup ---------- */

    pub fn roots(&self) -> Vec<NodeIdx> {
        let vec = self.vec.lock();
        (0..vec.len())
            .filter(|&i| self.root_predicate(&vec[i]))
            .map(|i| i as NodeIdx)
            .collect()
    }

    fn root_predicate(&self, node: &Node) -> bool {
        match node.kind {
            TreeKind::Span => node.parent.is_none(),
            TreeKind::Loc(LocKind::Crate) => node.parent.is_none(),
            _ => false,
        }
    }

    pub fn kind(&self, idx: NodeIdx) -> TreeKind {
        self.node(idx).kind
    }

    pub fn name_tok(&self, idx: NodeIdx) -> u32 {
        self.node(idx).name_tok
    }

    pub fn line_col(&self, idx: NodeIdx) -> Option<(u32, u16)> {
        let node = self.node(idx);
        if let TreeKind::Loc(LocKind::Line) = node.kind {
            Some((node.line, node.col))
        } else {
            None
        }
    }

    pub fn logs(&self, idx: NodeIdx) -> RwLockReadGuard<'_, Vec<Log>> {
        self.node(idx).logs.read()
    }

    pub fn child_iter(&self, idx: NodeIdx) -> ChildIter<'_> {
        ChildIter {
            arena: self,
            cur: self.first_child(idx),
        }
    }

    pub fn tok_str(&self, tok: u32) -> Arc<str> {
        self.ints.str_for(tok)
    }

    /* ---------- internals ---------- */

    fn node(&self, idx: NodeIdx) -> &Node {
        unsafe { &*(&self.vec.lock()[idx as usize] as *const Node) }
    }

    fn child(&self, parent: Option<NodeIdx>, kind: TreeKind, tok: u32) -> NodeIdx {
        if parent.is_none() {
            let vec = self.vec.lock();
            if let Some((i, _)) = vec.iter().enumerate().find(|(_, n)| {
                n.parent.is_none() && n.kind == kind && n.name_tok == tok
            }) {
                return i as NodeIdx;
            }
            drop(vec);
        }

        if let Some(p) = parent {
            let mut cur = self.first_child(p);
            while let Some(i) = cur {
                let n = self.node(i);
                if n.kind == kind && n.name_tok == tok {
                    return i;
                }
                cur = self.next_sib(i);
            }
        }

        self.new_node(parent, kind, tok)
    }

    fn child_line(&self, parent: Option<NodeIdx>, line: u32, col: u16) -> NodeIdx {
        if let Some(p) = parent {
            let mut cur = self.first_child(p);
            while let Some(i) = cur {
                let n = self.node(i);
                if let TreeKind::Loc(LocKind::Line) = n.kind {
                    if n.line == line && n.col == col {
                        return i;
                    }
                }
                cur = self.next_sib(i);
            }
        }

        self.new_line_node(parent, line, col)
    }

    fn new_node(&self, parent: Option<NodeIdx>, kind: TreeKind, tok: u32) -> NodeIdx {
        let mut vec = self.vec.lock();
        let idx = vec.len() as NodeIdx;
        vec.push(Node::new(kind, tok, parent));
        drop(vec);

        if let Some(p) = parent {
            self.splice_child(p, idx);
        }

        idx
    }

    fn new_line_node(&self, parent: Option<NodeIdx>, line: u32, col: u16) -> NodeIdx {
        let mut vec = self.vec.lock();
        let idx = vec.len() as NodeIdx;
        vec.push(Node::new_line(parent, line, col));
        drop(vec);

        if let Some(p) = parent {
            self.splice_child(p, idx);
        }

        idx
    }

    fn splice_child(&self, parent: NodeIdx, child: NodeIdx) {
        let head = &self.node(parent).first_child;
        loop {
            let cur = head.load(Ordering::Acquire);
            self.node(child).next_sib.store(cur, Ordering::Relaxed);
            if head
                .compare_exchange(cur, child + 1, Ordering::AcqRel, Ordering::Acquire)
                .is_ok()
            {
                break;
            }
        }
    }

    fn first_child(&self, idx: NodeIdx) -> Option<NodeIdx> {
        let v = self.node(idx).first_child.load(Ordering::Acquire);
        if v == NONE {
            None
        } else {
            Some(v - 1)
        }
    }

    fn next_sib(&self, idx: NodeIdx) -> Option<NodeIdx> {
        let v = self.node(idx).next_sib.load(Ordering::Acquire);
        if v == NONE {
            None
        } else {
            Some(v - 1)
        }
    }
}

impl Default for Arena {
    fn default() -> Self {
        Self::new()
    }
}

pub struct ChildIter<'a> {
    arena: &'a Arena,
    cur: Option<NodeIdx>,
}

impl<'a> Iterator for ChildIter<'a> {
    type Item = NodeIdx;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.cur?;
        self.cur = self.arena.next_sib(out);
        Some(out)
    }
}
