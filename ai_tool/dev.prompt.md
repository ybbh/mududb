## Development Requirements:

### ER Diagram:
    Provide an Entity-Relationship diagram using PlantUML.


### DDL Specifications:

Use SQLite syntax with the following limitations:

1. Foreign keys, indexes, and timestamp types are not supported.

2. Auto-increment keys are not supported.

Replacements:

1. Represent timestamps using the i64 type.

2. Use UUID strings instead of auto-increment keys.

3. Use integer numeric types for atomic timestamp fields.

## Rust Implementation:

Provide Mudu procedures implemented in Rust.

## Frontend and Backend

1. Implement a responsive web page using React and TypeScript.

2. The backend is MuduDB, which provides a set of JSON APIs.

3. Each Mudu procedure serves as a JSON API endpoint.

4. Each API expects a JSON request body containing the arguments for the Mudu procedure.

5. The response body is the JSON representation of the procedure’s return value.

### Show all the source code

Provide all source code for both the frontend and backend projects.

Show the project folder structure.