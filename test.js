const { scanProject, astOfFile } = require('./index.js');

console.log("🚀 Scanning current directory...");
const result = scanProject("./");

const singleFileAst = astOfFile("./src/lib.rs");
console.dir(singleFileAst, { depth: null , colors : true});

// Print summary
console.log(`✅ Scanned ${result.filesScanned} files in ${result.durationMs.toFixed(2)}ms`);

// Find a symbol to prove it works
const rustFile = result.files.find(f => f.path.includes("lib.rs"));
if (rustFile) {
    console.log("\nFound Rust Symbols in lib.rs:");
    console.table(rustFile.symbols);
}