# JSON Remix

My small contribution to the _Rewrite it in Rust!_ ~bandwagon~ movement. The original was written in [Typescript](https://github.com/vimcommando/json-remix)

## About

This is a Rust command-line interface (CLI) tool that provides four commands for manipulating JSON and NDJSON files: `split`, `merge`, `bundle`, and `unbundle`.

Each command can accept input or output from files, directories, or from standard input/output wherever relevant.

After being rewritten in Rust it is roughly 2x as fast as the Typescript implementation and can handle files larger than 512MB.

### Installation

This is published to [crates.io](https://crates.io/crates/jsrmx) so you can simply do a global install with:

```sh
cargo install jsrmx
```

Then `jsrmx` is executable from your shell

```sh
jsrmx --help
```

## Usage

There are four commands:

1. `merge` - merges multiple JSON objects into a single large JSON object
2. `split` - splits a single JSON object into multiple JSON objects by top-level keys
3. `bundle` - bundles multiple JSON objects ito an NDJSON (newline-delimited JSON) series
4. `unbundle` - unbundles an NDJSON series into a collection of separate JSON objects

All command support the option:
- `-d`, `--drop` - Drop output properties that match the comma-separated JSON paths

### merge

```sh
jsrmx merge <dir> [output]
```

#### Arguments

- `<dir>` - Required input directory
- `[output]` - Optional output file name (default `-` for stdout)

#### Options

- `-c`, `--compact` - Compact single-line output objects
- `-f`, `--filter` - regular expression to filter output keys
- `-p`, `--pretty` - Pretty-print output objects (default)
- `-t`, `--trim` - File extension to trim from object key names

#### Examples

Given a directory named `letters` with six files:

```
letters/alpha.json
letters/bravo.json
letters/charlie.json
letters/delta.json
letters/echo.json
letters/foxtrot.json
```

Where each file contains a few properties:

```jsonc
// cat alpha.json
{
  "uppercase": "A",
  "lowercase": "a",
  "position": 1
}
```

We can `merge` all the files into a single file:

```sh
jsrmx merge letters/ letters.json
```

So the contents of `letters.json` looks like:

```jsonc
// cat letters.json
{
  "alpha": {
    "lowercase": "a",
    "position": 1,
    "uppercase": "A"
  },
  "bravo": {
    "lowercase": "b",
    "position": 2,
    "uppercase": "B"
  },
  "charlie": {
    "lowercase": "c",
    "position": 3,
    "uppercase": "C"
  },
  "delta": {
    "lowercase": "d",
    "position": 4,
    "uppercase": "D"
  },
  "echo": {
    "lowercase": "e",
    "position": 5,
    "uppercase": "E"
  },
  "foxtrot": {
    "lowercase": "f",
    "position": 6,
    "uppercase": "F"
  }
}
```

Note the keys get sorted and have the `.json` extension trimmed from their names.

### split

```sh
jsrmx split [input] [output]
```

#### Arguments

- `[input]` - Optional input file name or `-` for `stdin` (default `-`)
- `[output]` - Optional output directory or `-` for `stdout`(default `-`)

#### Options

- `-c`, `--compact` - Compact single-line output objects
- `-f`, `--filter` - regular expression to filter output keys
- `-p`, `--pretty` - Pretty-print output objects (default)

#### Examples

We can split one file (or object through `stdin`) into individually-named files:

```sh
jsrmx split letters.json letters/
```

Given a the following single-object JSON file:

```jsonc
{
  "alpha": {
    "uppercase": "A",
    "lowercase": "a",
    "position": 1
  },
  "bravo": {
    "uppercase": "B",
    "lowercase": "b",
    "position": 2
  },
  // ... 3 entries omitted
  "foxtrot": {
    "uppercase": "F",
    "lowercase": "f",
    "position": 6
  }
}
```

The output files created will be:

```
letters/alpha.json
letters/bravo.json
letters/charlie.json
letters/delta.json
letters/echo.json
letters/foxtrot.json
```

Where each file contents will be the value from the large JSON:

```jsonc
// cat alpha.json
{
  "uppercase": "A",
  "lowercase": "a",
  "position": 1
}
```

If output to `stdout` the object will keep the top-level key as a parent object. Using `--filter` can extract specific keys.

```sh
jsrmx split --filter delta big_object.json -
```

```jsonc
{
  "delta": {
    "uppercase": "D",
    "lowercase": "d",
    "position": 4
  }
}
```

Combined with `--compact` this can convert a large object into an `.ndjson` file.

```sh
jsrmx split --compact big_object.json > letters.ndjson
```

```jsonc
// cat letters.ndjson
{"foxtrot":{"lowercase":"f","position":6,"uppercase":"F"}}
{"bravo":{"lowercase":"b","position":2,"uppercase":"B"}}
{"charlie":{"lowercase":"c","position":3,"uppercase":"C"}}
{"delta":{"lowercase":"d","position":4,"uppercase":"D"}}
{"echo":{"lowercase":"e","position":5,"uppercase":"E"}}
{"alpha":{"lowercase":"a","position":1,"uppercase":"A"}}
```

### bundle

```sh
jsrmx bundle <dir> [output]
```

#### Arguments

- `<dir>` - Required target input directory
- `[output]` - Optional output filename or `-` for stdout (default `-`)

#### Options

- `-e`, `--escape` -  List of field path to convert from nested JSON to an escaped string

#### Examples

We can convert a directory of `.json` files into a single `.ndjson` (newline-delimited JSON) file:

```sh
jsrmx bundle letters/ letters.ndjson
```

Given the files in the input directory:

```
letters/alpha.json
letters/bravo.json
letters/charlie.json
letters/delta.json
letters/echo.json
letters/foxtrot.json
```

With each file containing:

```jsonc
// cat alpha.json
{
  "letter": {
    "lowercase": "a",
    "uppercase": "A"
  },
  "name": "alpha",
  "position": 1
}
```

The output `letters.ndjson` will contain:

```jsonc
{"name":"alpha","letter":{"uppercase":"A","lowercase":"a"},"position":1}
{"name":"bravo","letter":{"uppercase":"B","lowercase":"b"},"position":2}
{"name":"charlie","letter":{"uppercase":"C","lowercase":"c"},"position":3}
{"name":"delta","letter":{"uppercase":"D","lowercase":"d"},"position":4}
{"name":"echo","letter":{"uppercase":"E","lowercase":"e"},"position":5}
{"name":"foxtrot","letter":{"uppercase":"F","lowercase":"f"},"position":6}
```

> NOTE: the filenames are not retained when bundling `.ndjson` files.

### unbundle

```sh
jsrmx unbundle [options] [intput] [output]
```

#### Arguments

- `[input]` - Optional input file name (default `-` for stdin)
- `[output]` - Optional output directory name (default `-` for stdout)

#### Options

- `-c`, `--compact` - Compact single-line output objects
- `-n`, `--name` - A list of JSON paths to use for filenames (uses first non-null)
- `-p`, `--pretty` - Pretty-print output objects (default)
- `-t`, `--type` - A JSON path to use for filename suffix (before extension)
- `-u`, `--unescape` - List of field paths to convert from escaped string to nested JSON

#### Example

Unbundling a file (or `stdin`) to a directory (or `stdout`):

```sh
jsrmx unbundle letters.ndjson letters/
```

Given the file `letters.ndjson`:

```jsonc
{"name":"alpha","letter":{"uppercase":"A","lowercase":"a"},"position":1}
{"name":"bravo","letter":{"uppercase":"B","lowercase":"b"},"position":2}
{"name":"charlie","letter":{"uppercase":"C","lowercase":"c"},"position":3}
{"name":"delta","letter":{"uppercase":"D","lowercase":"d"},"position":4}
{"name":"echo","letter":{"uppercase":"E","lowercase":"e"},"position":5}
{"name":"foxtrot","letter":{"uppercase":"F","lowercase":"f"},"position":6}
```

Unbundling with:

```sh
jsrmx unbundle letters.ndjson letters/
```

Will create these files:

```sh
letters/object-000001.json
letters/object-000002.json
letters/object-000003.json
letters/object-000004.json
letters/object-000005.json
letters/object-000006.json
```

The contents of each file will be pretty-printed by default:

```jsonc
// cat letters/object-000001.json
{
  "name": "alpha",
  "letter": {
    "uppercase": "A",
    "lowercase": "a"
  },
  "position": 1
}
```

Using the `--compact` option we can keep them as single-line entries:

```sh
jsrmx unbundle --compact letters.ndjson letters/
```

```jsonc
// cat letter/object-000001.json
{"name":"alpha","letter":{"uppercase":"A","lowercase":"a"},"position": 1}
```

For descriptive filenames, use the `--name` option. For a filename made of `${name}.json` run:

```sh
jsrmx unbundle --name=name letters.ndjson letters/
```

Will output `${name}.json` filenames:

```
letters/alpha.json
letters/bravo.json
letters/charlie.json
letters/delta.json
letters/echo.json
letters/foxtrot.json
```

Name values will work on nested values as long as the JSON path is `.` delimited. Periods in the key names will not resolve properly.

```sh
jsrmx unbundle --name=letter.lowercase letters.ndjson letters/
```

Will output `${letter.lowercase}.json` filenames:

```
letters/a.json
letters/b.json
letters/c.json
letters/d.json
letters/e.json
letters/f.json
```
