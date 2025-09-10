# DevfileRs

[devworkspace-generator](https://github.com/devfile/devworkspace-generator/tree/main)

Why a rust rewrite?

The need come from the fact that the current implementation of devworkspace-generator is in nodejs, a nodejs implementation needs a lot of dependencies and the nodejs runtime to be installed on the system, this is not always possible or desirable. If i want to use devfileRs in a cli tool, the size of the binary is important, a rust implementation can be compiled to a static binary with no dependencies, this is not possible with nodejs.

## Features

- [ ] devfile-schema
  - [x] schema validation
  - [x] devfile context
  - [x] devfile finder/insert default components
  - [x] editor download
  - [x] resolver
    - [x] github
    - [x] bitbucket
    - [x] bitbucket server
  - [x] Generate
  - [ ] main.ts
- [ ] rewrite des test unitaire en 1 pour 1 si possible

## Reminder

- CRD DevWorkspaceTemplate ==> Editor definition
- CRD DevWorkspace ==> Workspace definition
- Devfile ==> Workspace definition and must be stored in the project with the name `devfile.yaml` or `.devfile.yaml`
