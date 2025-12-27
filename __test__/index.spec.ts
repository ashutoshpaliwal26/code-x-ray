import test from 'ava'
import { scanProject } from '../index'
import path from 'path'
import { fileURLToPath } from 'url'

// --- ESM COMPATIBILITY FIX ---
// In ES Modules, __dirname doesn't exist. We must recreate it.
const __filename = fileURLToPath(import.meta.url)
const __dirname = path.dirname(__filename)
// -----------------------------

// Helper to get absolute path to fixtures
const FIXTURE_PATH = path.join(__dirname, 'fixtures')

test('Engine: Should scan directory and find files', (t) => {
  const result = scanProject(FIXTURE_PATH)
  
  // 1. Check basic stats
  t.is(typeof result.durationMs, 'number')
  // We expect at least 2 files (math.ts and script.py)
  t.true(result.filesScanned >= 2, 'Should find at least math.ts and script.py')
})

test('Parser: Should find TypeScript symbols', (t) => {
  const result = scanProject(FIXTURE_PATH)
  
  const tsFile = result.files.find((f) => f.path.includes('math.ts'))
  t.truthy(tsFile, 'math.ts should be in the report')
  
  if (!tsFile) return

  const classSymbol = tsFile.symbols.find((s) => s.name === 'Calculator')
  const funcSymbol = tsFile.symbols.find((s) => s.name === 'sqrt')

  t.truthy(classSymbol, 'Should detect "Calculator" class')
  t.truthy(funcSymbol, 'Should detect "sqrt" function')
})

test('Parser: Should find Python symbols', (t) => {
  const result = scanProject(FIXTURE_PATH)
  
  const pyFile = result.files.find((f) => f.path.includes('script.py'))
  t.truthy(pyFile, 'script.py should be in the report')

  if (!pyFile) return

  const defSymbol = pyFile.symbols.find((s) => s.name === 'process_data')
  t.truthy(defSymbol, 'Should detect "process_data" function')
})

test('Resilience: Should return empty list for non-existent path', (t) => {
  const result = scanProject('./imaginary/folder')
  t.is(result.filesScanned, 0)
  t.is(result.files.length, 0)
})