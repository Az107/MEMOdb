# MEMOdb TCP comunication specification

## stages
1. Connection

2. RERL (Request Eval Reply Loop)

### Commands
- [ ] *SET <KEY> <VALUE>:* Create or update a key value
- [ ] *GET <KEY> <INDEX...>:*
- [ ] *DEL <KEY>*:

### Example
```
s: MEMOdb vX.Y.Z
c: list
s: Ok: [Collection_name]
c: SELECT Collection_name
s: Ok
c: SET KEY VALUE
s: Ok: KEY created
c: GET KEI
s: Err: KEI do not exist
c: GET KEY
s: Ok: Value
```
