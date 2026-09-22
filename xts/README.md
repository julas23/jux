# xTree-Silver (`xts`)

**File manager for JUX.**

A file manager built around a tree view, aimed at people who navigate deep directory
structures and work with remote storage as a matter of routine.

## Scope

- Tree and list navigation, tabs and split panes
- Thumbnails, sorting, filtering and search
- Trash, bulk operations, archive handling
- Removable media and network shares

## Status

**Early planning.** No implementation yet.

## Known hard parts

These are the expensive pieces, recorded now so the estimate stays honest:

- **Remote filesystems.** There is no GVFS equivalent in Rust. Network shares — NFS,
  SMB, SFTP — need a deliberate answer rather than an assumption.
- **Thumbnails** mean a decoder for every format worth previewing, plus a cache that
  respects the freedesktop spec.
- **Change notification** at scale, without one watch per file.
- **Mounting** via `udisks2`, including encrypted volumes.

## Open questions

- What the "exclusive features" actually are. The tree view is the seed of an identity,
  not the whole of it — this needs a concrete list before implementation starts.
