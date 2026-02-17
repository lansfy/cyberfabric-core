# File Parser Module

File parsing module for CyberFabric / ModKit.

## Overview

The `cf-file-parser` crate implements the `file-parser` module and registers REST routes.

Parsing backends currently include:

- Plain text
- HTML
- PDF
- DOCX
- Images
- Stub parser (fallback)

## Configuration

```yaml
modules:
  file-parser:
    config:
      max_file_size_mb: 100
      # Required. Only files under this directory are accessible via parse-local.
      # Symlinks that resolve outside this directory are also blocked.
      allowed_local_base_dir: /data/documents
```

### Security: Local Path Restrictions

The `parse-local` endpoints validate requested file paths before any filesystem access:

1. Paths containing `..` components are always rejected.
2. The requested path is canonicalized (symlinks resolved) and must fall under `allowed_local_base_dir`.
3. `allowed_local_base_dir` is **required** — the module will fail to start if it is missing or the path cannot be resolved.

## License

Licensed under Apache-2.0.
