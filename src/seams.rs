//! THE SEAMS: what an environment must provide for Taldo to run there, and the one contract every call obeys. Definitions only — the
//! core never calls a seam (principle 9: it never acts on the world); the app's main wires an adapter per seam, and every adapter is
//! accepted only when it passes the core's conformance tests (`conformance`).
//!
//! THE CALL CONTRACT, the same for every seam:
//!   WHO      — every call carries the node it is made for (`Who`), a node's id in the twin, already proven by the adapter
//!   MAY      — a call made for someone on what is not theirs (an account) is a MOVEMENT (`Move`: for whom, on what, read or write).
//!              Whether it may happen is the twin's alone: the adapter asks it (`Record::may`) before it moves anything, and obeys —
//!              on no, nothing goes out, and the twin's why is the Fail. Nothing between the twin and the adapter decides
//!   HAPPENED — what a call did is answered whole (`Done`), and what it could not do is a `Fail` with why — never an empty value; a
//!              movement that happened is handed to the twin (`Record::happened`), written as its one act, with what it cost
//!   COST     — every answer says what the call used (`Cost`), measured by the adapter, its own way
use std::collections::BTreeMap;

/// WHO a call is made for: a node of the twin, by its id
pub type Who = String;

/// A MOVEMENT FOR SOMEONE: who it is made for (`who`, a node), the place not theirs it acts on (`on`: an account, a node of the twin),
/// and how — `is` "read" (what it holds comes from it) or "write" (something goes to it) — said in words (`says`)
#[derive(Debug, Clone, PartialEq)]
pub struct Move {
    pub who: Who,
    pub on: String,
    pub is: String,
    pub says: String,
}
/// THE RECORD, AS AN ADAPTER REACHES IT: the twin alone says whether a movement may happen (`may`: yes, or why not — writing
/// nothing), and takes what happened (`happened`: the movement written as its one act, with what it cost — or why the record
/// refused it). The adapter obeys: it asks before, and hands what happened after; it decides nothing itself
pub trait Record: Send + Sync {
    fn may(&self, m: &Move) -> Result<(), String>;
    fn happened(&self, m: &Move, cost: &Cost) -> Result<(), Unwritten>;
}

/// AN ENTRY NOT WRITTEN: the twin refused it — a door said no, with why — or it could not be kept (a fault, with why). Never told
/// apart by its words
#[derive(Debug, Clone, PartialEq)]
pub enum Unwritten {
    Refused(String),
    Failed(String),
}
impl Unwritten {
    /// IN WORDS, where it must be words (an agent's answer, a start that stopped, a seam's fault): which it was, and why — the one way
    /// it is said
    pub fn words(&self) -> String {
        match self {
            Unwritten::Refused(why) => format!("refused: {why}"),
            Unwritten::Failed(why) => format!("could not be kept: {why}"),
        }
    }
}

/// WHAT A CALL USED, as its adapter measured it — a value it could not measure is None, never 0
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Cost {
    pub ms: Option<u64>,
    pub bytes: Option<u64>,
    pub memory: Option<u64>,
    pub money: Option<f64>,
    /// WHAT A PROVIDER COUNTS A CALL IN, as it said it — tokens in and out, minutes, messages — by the unit's name; a unit it did not
    /// say is absent, never 0. It goes to the twin with the movement that happened (`Record::happened`)
    pub units: BTreeMap<String, f64>,
}

/// A CALL'S ANSWER: its value, and what it used
#[derive(Debug, Clone, PartialEq)]
pub struct Done<T> {
    pub value: T,
    pub cost: Cost,
}

/// WHAT A CALL COULD NOT DO, and why: the seam and call it came from, and whether trying again can help
#[derive(Debug, Clone, PartialEq)]
pub struct Fail {
    pub seam: &'static str,
    pub call: &'static str,
    pub why: String,
    /// None: trying again cannot help. Some(reason): it may, and why
    pub retry: Option<String>,
}
impl std::fmt::Display for Fail {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}.{}: {}", self.seam, self.call, self.why)
    }
}

pub type Answer<T> = Result<Done<T>, Fail>;

/// UNDO, as an adapter declares it for one of its acts: it can be undone (and how), or it cannot
#[derive(Debug, Clone, PartialEq)]
pub enum Undo {
    Yes(String),
    None,
}

// ─── FILES: things kept by name, in versions ───

/// ONE REVISION OF AN ITEM: its number in the item's history (1 first) and the hash of its bytes
#[derive(Debug, Clone, PartialEq)]
pub struct Revision {
    pub n: u64,
    pub sha256: String,
}
/// SOMETHING NEW LANDED: an item and its new version
#[derive(Debug, Clone, PartialEq)]
pub struct Change {
    pub item: String,
    pub version: Revision,
}
pub trait Files {
    /// a new version of `item`. A `kept` item is write-once: a second put is a Fail
    fn put(&self, who: &Who, item: &str, bytes: &[u8], kept: bool) -> Answer<Revision>;
    /// the newest version's bytes, or the version `n`'s
    fn get(&self, who: &Who, item: &str, n: Option<u64>) -> Answer<Vec<u8>>;
    /// every version of `item`, oldest first
    fn versions(&self, who: &Who, item: &str) -> Answer<Vec<Revision>>;
    /// the items whose names start with `prefix`
    fn list(&self, who: &Who, prefix: &str) -> Answer<Vec<String>>;
    /// every change under `prefix` after the `after`-th change this adapter saw there (0: all of them) — in order, none missed
    fn watch(&self, who: &Who, prefix: &str, after: u64) -> Answer<Vec<Change>>;
    /// what deleting does here: whether an item deleted can come back
    fn delete_undo(&self) -> Undo;
    fn delete(&self, who: &Who, item: &str) -> Answer<()>;
}

// ─── STORE: the record's rows, the twin's state, and the one writer ───

/// THE STORE IS THE TWIN'S SOURCE (`rows::Source`: an entry by its number, a page of them, the one taken with a signature) — the one
/// way the core already reads entries kept as rows — plus who may write, and the twin's state kept whole (`snapshot`)
pub trait Store: crate::rows::Source {
    /// take the writer's lease for `holder`: refused while another holds it; the same holder taking it again keeps it
    fn lease(&self, holder: &str) -> Answer<()>;
    /// give it up: refused when `holder` does not hold it
    fn release(&self, holder: &str) -> Answer<()>;
    /// KEEP, AS ONE — all of it, or none: entries the twin took, as rows by their number (`g`), each above every row kept; the rows a
    /// purge among them erased (`Twin::forgets`), forgotten — gone whole, found by nothing, and no byte of them left in any file or copy
    /// the store keeps (a journal, a log, a page freed) once the keep returns;
    /// and the twin's state after them — as the lease's holder; refused for anyone else, for an entry not above every row, and for a
    /// row to forget that is not kept. No row is ever changed. The number of the last row kept
    fn keep(&self, holder: &str, entries: &[serde_json::Value], forget: &[i64], state: &[u8]) -> Answer<i64>;
    /// the twin's state written last, whole
    fn state(&self) -> Answer<Option<Vec<u8>>>;
}

// ─── CLOCK: time, and timers that outlive a restart ───

pub trait Clock {
    /// now, in ms
    fn now(&self) -> u64;
    /// a timer named `name` that fires at `at` (ms) — kept, so it fires after a restart too; the same name again moves it
    fn at(&self, name: &str, at: u64) -> Answer<()>;
    /// every timer due by now, each once, taken off as it is given
    fn due(&self) -> Answer<Vec<String>>;
}

// ─── RUN: work started outside the program ───

/// what to start: a program kept in Files (by item and version) or one the environment provides, with its arguments
#[derive(Debug, Clone, PartialEq)]
pub struct Job {
    pub who: Who,
    pub program: String,
    pub args: Vec<String>,
    pub env: BTreeMap<String, String>,
}
#[derive(Debug, Clone, PartialEq)]
pub struct Ended {
    /// the code it ended with — none when a signal ended it
    pub code: Option<i32>,
    /// THE SIGNAL THAT ENDED IT, when one did: 9 (SIGKILL) is what a machine sends what it runs when it runs out of memory — unless the
    /// job was stopped. A job killed is said as killed, never as an end with no code
    pub signal: Option<i32>,
    pub out_tail: String,
}
pub trait Run {
    fn start(&self, job: &Job) -> Answer<u64>;
    fn alive(&self, handle: u64) -> Answer<bool>;
    /// stop it and everything it started
    fn stop(&self, handle: u64) -> Answer<Ended>;
    /// its end, once it ended — None while it runs
    fn ended(&self, handle: u64) -> Answer<Option<Ended>>;
    /// what this adapter started and still runs
    fn running(&self) -> Answer<Vec<u64>>;
}

// ─── SANDBOX: where a level runs ───

/// A LEVEL RUNS IN A SANDBOX, closed off from everything but what it is given: exactly the code its parent sealed, by hash — never
/// code it chose, never changing itself. Its parent (above it, a source — never another sandbox) runs a hash, replaces it with
/// another, stops it. Each adapter is one kind of box: a microVM on a server for a whole Taldo (a person's, a client's — the wall
/// between people), code inside one person's Taldo (its own levels). What a level is told and answers is the same in every box.
/// Every call carries `given`, as its parent says it now: where the level runs is the parent's, never kept by the box
pub trait Sandbox {
    /// run `code` (a hash its parent sealed) as `level`, given `given` (what it may reach, as its parent says) — what runs there now
    /// is replaced by it; the same hash again changes nothing
    fn run(&self, level: &str, code: &str, given: &serde_json::Value) -> Answer<()>;
    /// TELL a running level what happened (the record changed, a message came): what it answered — its proposals, or why it could not
    fn tell(&self, level: &str, given: &serde_json::Value, event: &serde_json::Value) -> Answer<serde_json::Value>;
    /// stop it: nothing of it runs after
    fn stop(&self, level: &str, given: &serde_json::Value) -> Answer<()>;
    /// the hash running as `level` now — none when nothing does
    fn running(&self, level: &str, given: &serde_json::Value) -> Answer<Option<String>>;
    /// EVERYTHING THIS BOX RUNS at the place `place` names (for a box on a server: its host; in-process: anything) — each `{ level,
    /// code, vm? }`: where it runs, as `run` was given it — so its parent sees what runs, never only what it remembers it started
    fn list(&self, place: &serde_json::Value) -> Answer<Vec<serde_json::Value>>;
}

// ─── NET IN: being reached ───

/// A REQUEST that reached Taldo: who made it (proven by the adapter, or None when nothing proved it), and what it asks
#[derive(Debug, Clone, PartialEq)]
pub struct Request {
    pub who: Option<Who>,
    pub method: String,
    pub path: String,
    pub headers: Vec<(String, String)>,
    pub body: Vec<u8>,
}
#[derive(Debug, Clone, PartialEq)]
pub struct Response {
    pub status: u16,
    pub headers: Vec<(String, String)>,
    pub body: Vec<u8>,
}
/// what answers every request: the app's, handed to the adapter by main
pub type Handler = std::sync::Arc<dyn Fn(Request) -> Response + Send + Sync>;
pub trait NetIn {
    /// answer every request with `handler` from now on; the address it is reached at
    fn serve(&self, handler: Handler) -> Answer<String>;
    /// stop being reached: a request after this is not answered
    fn stop(&self) -> Answer<()>;
}

// ─── NET OUT: calling out, a secret put in at the edge ───

/// HOW A SECRET GOES INTO A CALL: as a bearer token, as the password of a basic login under `user`, or as the value of a header of
/// its own (`x-api-key`) — every one applied at the edge. A provider's own way (a signature of the call) is its adapter's
#[derive(Debug, Clone, PartialEq)]
pub enum Auth {
    Bearer,
    Basic(String),
    Header(String),
}
pub trait NetOut {
    /// `r` sent to `url` (its path and query are `url`'s; `r.path` is not read), and its answer whole — with the secret Keys keeps as
    /// `secret` put in as `auth`, here at the edge: it never reaches the caller, and an answer is never changed to show it. A call
    /// made for someone (`m`) is a movement: asked of the record before, handed to it after; a secret goes out only in one — a call
    /// with a secret and no movement is a Fail, never sent
    fn call(&self, m: Option<&Move>, url: &str, r: Request, secret: Option<(&str, Auth)>) -> Answer<Response>;
}

// ─── MESSAGE: reaching a person where they are ───

pub trait Message {
    /// what a device of the person is reached at, by this adapter (a push subscription's public key, say) — what that device needs
    /// to let it reach it
    fn address(&self) -> Answer<String>;
    /// reach `to` (an address a device gave, as this adapter reads it) with `says` — or None: only that something waits, nothing
    /// of the person's carried by whoever delivers it
    fn send(&self, who: &Who, to: &serde_json::Value, says: Option<&str>) -> Answer<()>;
}

// ─── RENDER: drawing a page, as a person would see it ───

/// A PAGE AS DRAWN: its picture (PNG), the words it shows, and what went wrong on it (a script's error, a request that failed)
#[derive(Debug, Clone, PartialEq)]
pub struct Drawn {
    pub png: Vec<u8>,
    pub text: String,
    pub errors: Vec<String>,
}
pub trait Render {
    /// the page at `url` drawn `width` by `height` once it settles — with `script` run in it before its own, when given (what the
    /// page needs to be drawn as someone: a ticket, say) — or why it could not be
    fn draw(&self, who: &Who, url: &str, width: u32, height: u32, script: Option<&str>) -> Answer<Drawn>;
}

// ─── SOURCE: reading the person's world ───

/// ONE THING IN A FOLDER: its name, whether it is a folder itself, its size, and when it last changed (ms since 1970)
#[derive(Debug, Clone, PartialEq)]
pub struct Listed {
    pub name: String,
    pub folder: bool,
    pub bytes: u64,
    pub changed: u64,
}
/// A SOURCE IS EVERY OUTSIDE SYSTEM, where it is — the machine Taldo runs on (what it has and uses), a folder on the person's machine,
/// GitHub, a model (Anthropic), a provider's machines
/// (OVH), Google, a calendar, a bank — never copied in. What it HAS is read by path (the source's own, '/' between names, "" its top;
/// nothing outside it is read); what it DOES for someone — a model's answer, a server made — is an act, a movement: asked of the record
/// before, handed to it after
pub trait Source {
    /// what is at `path`, by name
    fn list(&self, who: &Who, path: &str) -> Answer<Vec<Listed>>;
    /// the bytes at `path`
    fn read(&self, who: &Who, path: &str) -> Answer<Vec<u8>>;
    /// what it does, asked `what` with `input`, for the movement `m`: its answer — or why not (a no from the record, nothing asked, an act
    /// it does not do)
    fn act(&self, m: &Move, what: &str, input: &serde_json::Value) -> Answer<serde_json::Value>;
}

// ─── BACKUP: the record and the files, kept where Taldo is not ───

/// BACKUP: what Taldo keeps of the owner's — the record's segments, their things' folders — sealed by the adapter and kept at a second
/// company, so a lost machine loses nothing; read back exactly, standing alone when the machine is gone. The place never holds what was
/// put in plain
pub trait Backup {
    /// `bytes` kept as `name` (again: the newest replaces it)
    fn keep(&self, name: &str, bytes: &[u8]) -> Answer<()>;
    /// what was kept as `name`, opened — exactly as put
    fn restore(&self, name: &str) -> Answer<Vec<u8>>;
    /// every name kept under `prefix`
    fn list(&self, prefix: &str) -> Answer<Vec<String>>;
}

// ─── KEYS: signing as a node ───

pub trait Keys {
    /// the public key of `name`'s key (SEC1, hex), made if there is none — with `presence`, a key that signs only with a person there
    /// (a passkey): the key the twin then marks `presence`
    fn key(&self, name: &str, presence: bool) -> Answer<String>;
    /// `bytes` signed with `name`'s key — the person asked with `reason` when the key needs them there — as the twin reads a
    /// signature: an entry's `sig` (`{ key, sig }`)
    fn sign(&self, name: &str, bytes: &[u8], reason: &str) -> Answer<serde_json::Value>;
    /// the key is gone: nothing signs with it again
    fn forget(&self, name: &str) -> Answer<()>;
    /// A SECRET OF THE PERSON'S — an account's token — kept under `name` (a second keep replaces it). It is never handed back: only
    /// applied to a call at the edge (Net out), so it reaches no session and no model
    fn keep_secret(&self, name: &str, secret: &[u8]) -> Answer<()>;
    /// whether a secret is kept under `name`
    fn has_secret(&self, name: &str) -> Answer<bool>;
    /// the secret is gone
    fn forget_secret(&self, name: &str) -> Answer<()>;
}

// ─── IDENTITY: proving who or what something outside is ───

/// A KIND OF IDENTITY, as the adapter is given it: its name and its way — for an open kind (fields, OAuth, agent) the description
/// at its `identity` node in the record; for a locked kind (a person, a device, Taldo) the adapter's own code knows it
#[derive(Debug, Clone, PartialEq)]
pub struct Kind {
    pub name: String,
    pub way: serde_json::Value,
}
/// WHAT WAS PROVEN: who it is at its source (an account's name, an email, a device's key), the public key it signs with when it has
/// one, and the names Keys now keeps its secrets under — never a secret itself
#[derive(Debug, Clone, PartialEq)]
pub struct Proven {
    pub subject: String,
    pub key: Option<String>,
    pub kept: Vec<String>,
}
/// IDENTITY: one way in for everything outside — an account, a sign-in, a device, an agent. The adapter proves it the way its kind
/// works and says who it is; whether it may do anything is the twin's alone, by grants
pub trait Identity {
    /// what the person does to prove it: the fields to fill, the page to open, the code to show
    fn begin(&self, kind: &Kind) -> Answer<serde_json::Value>;
    /// prove `name` of `kind` with what was offered: its secrets kept by Keys at the edge, never handed back — or why not
    fn prove(&self, kind: &Kind, name: &str, offered: &serde_json::Value) -> Answer<Proven>;
}

// ─── AGENT: a session of an agent at work ───

/// WHAT A SESSION DID, as its runtime told it: what it said, a step it took (a tool, with what it was given), its turn ended, or it
/// ended
#[derive(Debug, Clone, PartialEq)]
pub enum Heard {
    Said(String),
    Step { tool: String, says: String },
    TurnEnded,
    Ended(String),
}
/// what hears a session, as it happens: the session's id and what it did
pub type Hear = std::sync::Arc<dyn Fn(&str, Heard) + Send + Sync>;
pub trait Agent {
    /// a session started in `dir` with `words`, everything it does told to `hear` as it happens: its id
    fn start(&self, dir: &str, words: &str, hear: Hear) -> Answer<String>;
    /// more words to a session, as its next turn: refused while it is still at work on the turn before
    fn say(&self, session: &str, words: &str) -> Answer<()>;
    /// A SESSION FROM BEFORE taken up again — one this runtime does not hold now (it restarted, a new version of Taldo) — in `dir`,
    /// with `words` as its next turn, everything it does told to `hear`
    fn resume(&self, session: &str, dir: &str, words: &str, hear: Hear) -> Answer<()>;
    /// whether it is at work on a turn now
    fn working(&self, session: &str) -> Answer<bool>;
    /// stop it: its turn ends now, and it takes no more words
    fn stop(&self, session: &str) -> Answer<()>;
}
