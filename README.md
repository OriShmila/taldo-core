# taldo-core

The core of [Taldo](https://usetaldo.com): the part that decides who may do what with your data, and nothing else.

Taldo keeps a business's work as one record: its people, its clients, their cases, and the devices and agents that act for them.
This crate is the small piece underneath that every Taldo runs on. It is open so anyone can check the rules that guard a client's
data, instead of taking our word for them.

## What it does

- **One signed record.** Everything is a node in one graph (a person, an org, a case, a device, an agent), made only under its
  parent. Every entry is signed by whoever made it (P-256 ECDSA, or a passkey), checked when it is written, and never rewritten.
  The owner of a node is the person at the top of its chain.
- **Permissions as grants.** Acting on what is not yours needs a grant from its owner: to whom, which action, on which node (or
  everything below it), until when, within what budget. A blocked node signs nothing.
- **Deleting, for real.** A delete hides a node and everything under it at once. After the waiting its owner chose, it is erased:
  only ids and kinds stay, so what others wrote about it still reads.
- **Getting back in.** Someone who loses their keys takes their place back only through ways they granted themselves in advance
  (a synced passkey, a person they trust, another device, a recovery code). Never through us.
- **Who is asking.** A request from a screen or an agent is proven like an entry: signed for this exact method, path and body,
  within a window of now, never replayed.
- **The seams.** The core never touches the world. It defines what an environment must provide (keys, files, storage, a clock,
  network in and out, messages, rendering, the machine, outside sources, agents, versions, backup) and the conformance tests every
  implementation must pass. In-memory fakes pass them too, so the rules are tested in seconds.

## What it is not

It is not the product. Taldo's screens, its workflows, and the adapters that do the real work on a Mac or a server live elsewhere.
This crate holds only the rules, and it changes rarely, by a deliberate step.

## Use

```toml
[dependencies]
taldo-core = { git = "https://github.com/OriShmila/taldo-core.git" }
```

```sh
cargo test
```

## License

MIT. See [LICENSE](LICENSE).
