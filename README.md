# genrmsg - Rust Message Generator

A tool for generating Rust message definitions from YAML specifications.

Written by: Johan Persson <johan162@gmail.com>


## Features

- Generate message structs with serialization/deserialization support
- Automatic message numbering and prefix-naming
- Generate documentation in Markdown format
- Locking message numbers for API stability
- Automatically genrate Trait code for `ToStr` and `FromStr` in used enums

## Usage

### Basic Usage

```
genrmsg -i messages.yaml -o src/messages.rs
```

### Generationg message name with prefix

A common standard in message definition is to generate a common prefix to all messages, e.g. `M001_`, `M002_`, etc.
The generator can add such a prefix to all messages automatically with the `-m` option.

```
genrmsg -i messages.yaml -o src/messages.rs -m
```


### Lock Message Numbering

To maintain API compatibility when adding new messages, you can lock the numbering:

```
genrmsg -i messages.yaml -o src/messages.rs -l
```

This will update the YAML file with explicit message numbers added to each message definition, ensuring that future additions 
won't change the numbering of existing messages.

### Reset Message Numbering

When making a major version change and you're willing to break API compatibility, 
you can reset all message numbers:

```
genrmsg -i messages.yaml -o src/messages.rs -R
```

This will:
1. Remove all explicit message numbers from the YAML file
2. Update the YAML file
3. Reassign message numbers based on the current order
4. Generate the updated code



*Note:* The output file can also be specified in the YAML file. Any command line option will override the one in the YAML file.

### Generate Markdown documentation of all messages

A table in Markdown format with message names and payload for both request and response is generated with the `-t` option:
as follows

```
genrmsg -i messages.yaml -o src/messages.rs -t
```

The generated tables are separated per defined category for ease of reading.
The generated markdown file will have the same name as the output file but with an `*.md` extension.

Example of generated table:

| # | Description | Request Name | Request Payload | Reply Name | Reply Payload |
|----|-------------|----------------|------------|---------------|-------------|
| 1 | Create a new project | `M001_NewProj` | `project_name`: `String`<br>`project_start_date`: `NaiveDate` | `M001_NewProj_Reply` | `project_id`: `i64`<br>`project_name`: `String`<br>`project_start_date`: `NaiveDate` |
| 2 | Delete existing project and its simulations | `M005_DelProj` | `project_id`: `i64` | `M005_DelProj_Reply` | `project_id`: `i64` |
| 3 | Get project base data | `M006_GetProj` | `project_id`: `i64` | `M006_GetProj_Reply` | `num_tasks`: `i64`<br>`project_id`: `i64`<br>`project_name`: `String`<br>`project_start_date`: `NaiveDate`<br>`revision`: `i64` |


### Validate the input YAML file

You can validate the input YAML file with the `-y` option:

```
genrmsg -i messages.yaml -o src/messages.rs -y
```

This will not generate any message structure.



### Options

```
  -i, --input <FILE>     Sets the input YAML file
  -o, --output <FILE>    Sets the output Rust file (overrides the one in YAML if specified)
  -p, --prefix <PREFIX>  Sets the message prefix [default: M]
  -s, --start <NUMBER>   Starting number for message numbering (defaults to 1) [default: 1]
  -d, --digits <DIGITS>  Number of digits to use for the message number (defaults to 3) [default: 3]
  -m, --prefix-messages  Enable prefixing of message names (disabled by default)
  -t, --mdtable          Generate a Markdown table documenting all messages. Same name as the input file with .md extension
  -v, --verbose <LEVEL>  Sets the verbosity level, 0=Error, 1=Warn, 2=Info, 3=Debug, 4=Trace (default is 0) [default: 0]
  -y, --validate-yaml    Validate that the YAML file follows the schema but don't generate code
  -l, --lock-yaml        Lock the last generated prefix numbers in YAML so all messages get the same number next time
  -R, --reset-numbering  Reset all message numbering (for major version changes)
  -h, --help             Print help
  -V, --version          Print version
```

## Example YAML Format

As of now all sections on the top level, i.e. `settings`, `common_structs`, `enums`, `error_message`, and `messages` must exist. Otherwise a format error will be raised.

The generic structure of mandatory fields is as follows:

```
settings:
  output_file: path_to_output_file
  module_doc: description of the module
  serialization_framework: [bincode | serde]

common_structs:
  struct1:
    description:
    fields:
      field1:
        type:
        description:

enums:
  enum1:
    description: Format for importing project data
    variants:
    - variant1
    - variant2
    implement_traits:
    - Display
    - FromStr

error_message:
  description: Error description
  fields:
    error_message:
      type: String
      description: Human-readable error message
    error_id:
      type: u32
      description: Unique error identifier

messages:
  - category: category1
    messages:
      - name: first_message
        description: "The first message"
        request:
          fields:
            field1:
              type_name: type1
              description: "field description"
            field2:
              type_name: type2
              description: "field description"
        response:
          fields:
            response_field1:
              type_name: type3
              description: "field description"
            response_field2:
              type_name: type4
              description: "field description"
```

An actual example is as follows:


```yaml
settings:
  output_file: ./msg_lib.rs
  module_doc: Test message defintion
  serialization_framework: bincode

common_structs:
  SimulationResult:
    description: Simulation result data structure
    fields:
      num_iterations:
        type: i64
        description: Number of iterations in the simulation
      proj_id:
        type: i64
        description: Project identifier
      percentiles:
        type: '[i64; 20]'
        description: Stores p99, p95, ... p5
      task_distribution:
        type: TaskDistribution
        description: Type of distribution used
      sim_id:
        type: i64
        description: Unique identifier for the simulation
      created_at:
        type: DateTime<Utc>
        description: Timestamp when simulation was created
      proj_revision:
        type: i64
        description: Project revision at simulation time

enums:
  ImportFormat:
    description: Format for importing project data
    variants:
    - yaml
    - csv
    - json
    implement_traits:
    - Display
    - FromStr

error_message:
  description: Generic error message for all possible errors
  fields:
    error_message:
      type: String
      description: Human-readable error message
    error_id:
      type: u32
      description: Unique error identifier

messages:
  - category: Authentication
    messages:
      - name: Login
        description: "User login request"
        request:
          fields:
            username:
              type_name: String
              description: "User name"
            password:
              type_name: String
              description: "User password"
        response:
          fields:
            success:
              type_name: bool
              description: "Login success status"
            token:
              type_name: Option<String>
              description: "Authentication token"
```

## License

MIT
