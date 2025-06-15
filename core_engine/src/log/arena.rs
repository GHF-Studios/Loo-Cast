use std::{
    sync::atomic::{AtomicU32, Ordering},
    sync::Arc,
};

use dashmap::DashMap;
use parking_lot::{Mutex, RwLock, RwLockReadGuard};

/* ---------- public basics ---------- */

pub type NodeIdx = u32;
const NONE: u32 = 0;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    Span,
    Module,
    File,
    Line,
}

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
}

/* ---------- the node slab ---------- */

struct Node {
    kind: Kind,
    name_tok: u32, // Span / Module / File
    line: u32,     // Line nodes only
    col: u16,      // Line nodes only
    parent: Option<NodeIdx>,
    first_child: AtomicU32,
    next_sib: AtomicU32,
    logs: RwLock<Vec<Log>>, // only Line nodes fill this
}

impl Node {
    fn new(kind: Kind, name_tok: u32, parent: Option<NodeIdx>) -> Self {
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
            kind: Kind::Line,
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

pub struct Arena {
    vec: Mutex<Vec<Node>>,
    ints: Interns,
    by_file: DashMap<u32, Vec<NodeIdx>>, // file-tok → [line-node]
}

impl Arena {
    pub fn new() -> Self {
        Self {
            vec: Mutex::new(Vec::with_capacity(1024)),
            ints: Interns::default(),
            by_file: DashMap::new(),
        }
    }

    /* ---------- hot-path insert ---------- */

    #[allow(clippy::too_many_arguments)]
    pub fn insert(
        &self,
        span_path: &[&str],
        module_path: &[&str],
        file_path: &str,
        line: u32,
        col: u16,
        lvl: Level,
        msg: impl Into<Arc<str>>,
    ) {
        let mut parent = None;

        for seg in span_path {
            parent = Some(self.child(parent, Kind::Span, self.ints.get(seg)));
        }
        for seg in module_path {
            parent = Some(self.child(parent, Kind::Module, self.ints.get(seg)));
        }

        let file_tok = self.ints.get(file_path);
        parent = Some(self.child(parent, Kind::File, file_tok));

        let leaf = self.child_line(parent, line, col);
        self.by_file
            .entry(file_tok)
            .or_default()
            .push(leaf);

        let log = Log {
            ts: now_ns(),
            lvl,
            msg: msg.into(),
        };
        self.node(leaf).logs.write().push(log);
    }

    /* ---------- queries ---------- */

    pub fn roots(&self) -> Vec<NodeIdx> {
        let vec = self.vec.lock();
        (0..vec.len())
            .filter(|&i| vec[i].kind == Kind::Span && vec[i].parent.is_none())
            .map(|i| i as NodeIdx)
            .collect()
    }

    pub fn child_iter(&self, idx: NodeIdx) -> ChildIter<'_> {
        ChildIter {
            arena: self,
            cur: self.first_child(idx),
        }
    }

    pub fn logs(
        &self,
        idx: NodeIdx,
    ) -> RwLockReadGuard<'_, Vec<Log>> {
        self.node(idx).logs.read()
    }

    pub fn kind(&self, idx: NodeIdx) -> Kind {
        self.node(idx).kind
    }
    pub fn name_tok(&self, idx: NodeIdx) -> u32 {
        self.node(idx).name_tok
    }
    pub fn line_col(&self, idx: NodeIdx) -> Option<(u32, u16)> {
        let n = self.node(idx);
        if n.kind == Kind::Line {
            Some((n.line, n.col))
        } else {
            None
        }
    }

    /// Reverse-lookup an interned token → its `Arc<str>` text.
    pub fn tok_str(&self, tok: u32) -> Arc<str> {
        // Only used by UI code; linear scan is fine.
        self.ints
            .map
            .iter()
            .find(|kv| *kv.value() == tok)
            .map(|kv| kv.key().clone())
            .unwrap_or_else(|| Arc::<str>::from("<unknown>"))
    }

    /* ---------- internals ---------- */

    fn child(&self, parent: Option<NodeIdx>, kind: Kind, tok: u32) -> NodeIdx {
        // │─ NEW: handle the root level ────────────────────────────────────────
        if parent.is_none() {
            // lock the vec just long enough to scan existing roots
            let vec = self.vec.lock();
            if let Some((i, _)) = vec
                .iter()
                .enumerate()
                .find(|(_, n)| n.parent.is_none() && n.kind == kind && n.name_tok == tok)
            {
                return i as NodeIdx; // found; reuse
            }
            // drop lock before we allocate a new node
            drop(vec);
        }
        // │─ OLD path for non-root or not-found root ───────────────────────────
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
                if n.kind == Kind::Line && n.line == line && n.col == col {
                    return i;
                }
                cur = self.next_sib(i);
            }
        }
        self.new_line_node(parent, line, col)
    }

    fn new_node(&self, parent: Option<NodeIdx>, kind: Kind, tok: u32) -> NodeIdx {
        let mut vec = self.vec.lock();
        let idx = vec.len() as NodeIdx;
        vec.push(Node::new(kind, tok, parent));
        drop(vec);

        if let Some(p) = parent {
            self.splice_child(p, idx);
        }
        idx
    }

    fn new_line_node(
        &self,
        parent: Option<NodeIdx>,
        line: u32,
        col: u16,
    ) -> NodeIdx {
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

    /* ---------- utils ---------- */

    fn node(&self, idx: NodeIdx) -> &Node {
        unsafe { &*(&self.vec.lock()[idx as usize] as *const Node) }
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

/* ---------- child iterator ---------- */

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

/* ---------- time util ---------- */

fn now_ns() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos() as u64
}
