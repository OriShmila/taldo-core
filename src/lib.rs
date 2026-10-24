//! TALDO'S CORE. It only enforces Taldo's principles; it never proposes and never acts on the world. Small, tested, frozen: a change to
//! it is a deliberate step by a person, never something the graph does to itself. Everything else — keys and how they are kept,
//! screens, workers, versions of what is built — lives inside the graph, on top of it.
//!
//! THE PRINCIPLES, and where the core enforces each:
//!   1. IDENTITY — each part has its own identity, and every act is verified as its own.             `identity::verify`
//!   2. TWIN — everything is recorded once, never rewritten — except what its owner deletes, which
//!      is erased after the waiting they set: only its ids and kinds stay.            `twin::fold`, one act once
//!   3. UNCONTROLLED — what doesn't depend on you, whether it hit you or you chose it, is only
//!      measured.                                                                                     an entry by the node that saw it
//!   4. CONTROLLED — what depends on you can be reviewed and reversed. Choosing something
//!      uncontrolled is itself controlled, and reaches the authority before it's done.                held by the level above (5)
//!   5. HOLD — each level holds the one below.                                                        `twin`: acceptances, what is in
//!   6. GROWTH — what's held grows from what ran and worked.                                          held only by the level above
//!   7. OWED — whatever doesn't hold is owed to someone. It may stay open, but never unseen.          `twin::owed`
//!   8. REACH — everything goes to the address that should get it, and its arrival is recorded.      `twin::owed` (the address), `twin::reached`
//!   9. CORE — the core only enforces these principles. It never proposes and never acts on the world.
//!
//! THE SEAMS (`seams`): what an environment must provide, and the one contract every call obeys — definitions only, never called
//! here; every adapter passes the conformance tests (`conformance`), and so do the in-memory fakes tests run on (`fakes`).
//!
//! Not yet in the core: growth's stages before a hold (seen, established) — today only what the level above holds is held.
pub mod conformance;
pub mod entry;
pub mod fakes;
pub mod identity;
pub mod rows;
pub mod seams;
pub mod snapshot;
pub mod twin;
