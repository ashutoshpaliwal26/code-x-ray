<div align="center">

# ⚡️ F I L E - C O N T E X T - T R E E

**The High-Performance AST Scanner for the Modern Web**

[![CI Status](https://img.shields.io/github/actions/workflow/status/YOUR_USERNAME/file-context-tree/CI.yml?style=for-the-badge&logo=github&labelColor=18181b&color=22c55e)](https://github.com/YOUR_USERNAME/file-context-tree/actions)
[![NPM Version](https://img.shields.io/npm/v/file-context-tree?style=for-the-badge&logo=npm&labelColor=18181b&color=cb3030)](https://www.npmjs.com/package/file-context-tree)
[![Built With Rust](https://img.shields.io/badge/Powered_By-Rust-orange?style=for-the-badge&logo=rust&labelColor=18181b&color=ef4444)](https://www.rust-lang.org/)
[![License](https://img.shields.io/npm/l/file-context-tree?style=for-the-badge&labelColor=18181b&color=3b82f6)](LICENSE)

<br />

**Give your AI Agents "Eyes" into your codebase.**
<br />
*Scan 10,000 files in seconds. Extract symbols instantly. Zero overhead.*

[Report Bug](https://github.com/YOUR_USERNAME/file-context-tree/issues) · [Request Feature](https://github.com/YOUR_USERNAME/file-context-tree/issues)

</div>

---

## 🔮 The Problem
Node.js is fantastic, but it struggles with heavy computation. Trying to parse thousands of TypeScript or Python files in a single thread is slow, fragile, and memory-intensive.

## ⚡️ The Solution: **Code X-Ray**
We moved the heavy lifting to **Rust**.
By bridging Node.js with a compiled Rust binary, we bypass the event loop entirely, utilizing **Parallel Computing** to scan your project at the speed of disk I/O.

### 🔥 Why Developers Choose X-Ray

| 🚀 Blazing Performance | 🛡️ Bulletproof Safety | 🧠 Intelligent Parsing |
| :--- | :--- | :--- |
| Uses `Rayon` to multithread across all CPU cores. Scans **~1.5ms per file**. | Sandboxed file access. Respects `.gitignore` automatically. No crashes. | Powered by **Tree-Sitter**. Understands code structure, not just regex matches. |

---

## 📦 Installation

Add it to your project with a single command. It detects your OS (Windows/Linux/Mac) and downloads the correct optimized binary automatically.

```bash
npm install file-context-tree

```
💻 Developer Experience
We designed Code X-Ray to feel like a native part of your toolchain. It’s fully typed, asynchronous-ready, and zero-config.

1. The "Hello World" Scan
Get a complete map of your project in 3 lines of code.

TypeScript
```
import { scanProject } from 'file-context-tree';
```
## 🚀 Fire up the engine (scans recursively)
```
console.time("✨ Magic Time");
const context = scanProject("./src");
console.timeEnd("✨ Magic Time");

console.log(`\n📦 Scanned ${context.files_scanned} files in ${context.duration_ms}ms`);
```

### 2. Powerful Filtering
Don't just scan—understand. Filter the raw AST data to find exactly what you need.

TypeScript
```
// Example: Find all 'TODO' comments or specific function definitions
const context = scanProject("./src");

// 🔍 Filter for TypeScript functions only
const functions = context.files
  .filter(f => f.language === 'TypeScript')
  .flatMap(f => f.symbols)
  .filter(s => s.kind === 'function');

console.table(functions.map(fn => ({
  Name: fn.name,
  Location: `L${fn.start.row}:${fn.start.column}`,
  Signature: fn.signature || '(unknown)'
})));
```
### 3. The "Context" Payload
The engine returns a clean, highly-structured JSON object optimized for LLM Context Windows (GPT-4, Claude, Llama 3).

<details> <summary><b>👀 Click to view sample JSON Output</b></summary>

JSON
```
{
  "root_dir": "./src",
  "stats": {
    "duration_ms": 142.5,
    "files_processed": 45,
    "threads_active": 12
  },
  "files": [
    {
      "path": "src/services/auth.ts",
      "language": "TypeScript",
      "size": 2048,
      "symbols": [
        {
          "name": "authenticateUser",
          "kind": "function",
          "range": { 
            "start": { "row": 15, "col": 0 }, 
            "end": { "row": 25, "col": 1 } 
          },
          "signature": "async (token: string) => Promise<User>"
        }
      ]
    }
  ]
}
```
</details>
