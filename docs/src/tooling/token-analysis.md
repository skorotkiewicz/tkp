# Token Analysis (AI/LLM)

## Benchmark Results

Tested with GPT-4o tokenizer (tiktoken), comparing the same Fibonacci program:

| Language | Tokens |
|----------|--------|
| Python | 54 |
| JavaScript | 69 |
| TKP | 88 |

## Why toki pona Uses More Tokens

LLM tokenizers use BPE (Byte Pair Encoding):

1. Start with raw bytes
2. Find the most frequent byte pairs in training data
3. Merge them into single tokens
4. Repeat

Since training data is predominantly English:
- `function` → appears billions of times → merged into 1 token
- `pali` → rarely appears → stays as 2-3 byte-level tokens

## Per-Keyword Comparison

| TKP | Tokens | English | Tokens |
|-----|--------|---------|--------|
| `pali` | 2 | `function` | 1 |
| `pana` | 2 | `return` | 1 |
| `ijo` | 2 | `let` | 1 |
| `ante` | 3 | `else` | 1 |
| `pini` | 3 | `break` | 1 |
| `lon` | 1 | `while` | 1 |
| `kin` | 1 | `true` | 1 |

## This Is a Tokenizer Problem, Not a toki pona Problem

If BPE were trained on a toki pona-heavy corpus, `pali` could be a single token. The inefficiency comes from training data distribution, not from the script itself.

Relevant work:
- Ukrainian LLM "Lapa" replaced 80K tokens and achieved 1.5x efficiency for Ukrainian text
- Custom BPE training on toki pona programming text could close the gap
