// inspect_context.js
const fs = require('fs');
const { scanProject } = require('./index.js');

console.log("🔍 Scanning project to generate Context Map...");

// 1. Run the Rust Scanner
const result = scanProject("./");

// 2. Pretty Print the Result
// null, 2 means "indent with 2 spaces" -> makes it readable
const jsonOutput = JSON.stringify(result, null, 2);

// 3. Save to a file so you can open it in VS Code
const outputFile = "context_dump.json";
fs.writeFileSync(outputFile, jsonOutput);

console.log(`\n✅ Context Dump saved to: ${outputFile}`);
console.log("--------------------------------------------------");
console.log("👇 Here is a preview of what the AI sees (First file):");
console.log("--------------------------------------------------");

// Preview the first file found (if any)
if (result.files.length > 0) {
    // Find a non-empty file to show interesting data
    const meaningfulFile = result.files.find(f => f.symbols.length > 0) || result.files[0];
    console.log(JSON.stringify(meaningfulFile, null, 2));
}