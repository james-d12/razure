# Razure
This project generates rust code from the OpenAPI specifications from Azure.


## Features
- Reads all openapi specification files from azure repository.
- Parses the basic structure including:
  - Definitions
  - Parameters
  - Swagger Info
  - Paths

## Todo

- Schema model to represent: https://swagger.io/specification/v2/#schema-object
- Update models to reflect property types and if they are required correctly.
- Support referencing other definitions / parameters when creating the Rust Structs, e.g.:

    ```rust
    pub struct Thing {
        pub name: String,
        pub age: i8,
    }
    
    pub struct BiggerThing {
        pub thing: Thing,
        /// other properties
    }
    ```
- Add parsing support for ``tags``, ``securityDefinitions`` and ``security``.
- Of course more unit tests 😁