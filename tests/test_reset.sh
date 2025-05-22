#!/bin/zsh
# Test script for message number reset functionality

# Build the tool
cargo build

# Example YAML with numbering
cat > test_messages.yaml << EOL
settings:
  module_doc: "API messages for the test service"
  output_file: "test_messages.rs"
  serialization_framework: "bincode"

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
- category: Project Management
  messages:
  - name: NewProj
    description: Create a new project
    number: 10
    request:
      fields:
        project_name:
          type: String
          description: Name of the project
        project_start_date:
          type: NaiveDate
          description: Project start date
    response:
      fields:
        project_name:
          type: String
          description: Name of the project
        project_start_date:
          type: NaiveDate
          description: Project start date
        project_id:
          type: i64
          description: Unique identifier for the new project
  - name: DelProj
    description: Delete existing project and its simulations
    number: 12
    request:
      fields:
        project_id:
          type: i64
          description: Project identifier to delete
    response:
      fields:
        project_id:
          type: i64
          description: Project identifier that was deleted
  - name: GetProj
    description: Get project base data
    number: 13
    request:
      fields:
        project_id:
          type: i64
          description: Project identifier to retrieve
    response:
      fields:
        num_tasks:
          type: i64
          description: Number of tasks in the project
        project_id:
          type: i64
          description: Project identifier
        project_name:
          type: String
          description: Name of the project
        revision:
          type: i64
          description: Project revision number
        project_start_date:
          type: NaiveDate
          description: Project start date
  - name: DelAllProj
    description: Delete all stored projects and simulations
    number: 14
    request:
      fields: {}
    response:
      fields:
        placeholder:
          type: i64
          description: Placeholder value
EOL

echo "Testing reset numbering functionality..."
./target/debug/genrmsg -v3 -i test_messages.yaml -R 

# Show the updated YAML
echo "\nUpdated YAML file with reset numbering:"
# cat test_messages.yaml

echo "\nTest completed."
