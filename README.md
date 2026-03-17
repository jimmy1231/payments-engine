### Run
```
cargo run -- transaction.csv
```

### Test
```
cargo test
```

### Assumptions
- allow an account's available balance to go negative in disputes, which the client would need to pay back later
- only deposite transactions can be disputed given the requirement that the funds exchanged in the transaction is moved from the available amount to be held (i.e. locking the deposited funds until the dispute is resolved)
- can we double dispute/chargeback/resolve a transaction?
    - we cannot double chargeback
- a transaction can be disputed more than once, and resolved more than once since resolutions does not result in a change to total account balance, but can only be __chargedback once__ since a chargeback subtracts funds from the account.

### Transaction State

A `state` is kept by each deposit/withdrawal transaction to keep track of whether it is being actively disputed, and to prevent multiple chargebacks. During processing, a withdrawal / deposit transaction can take 3 states - `Normal`, `Disputed`, `Chargeback`. This diagram illustrates the state machine used by the engine:
- **Normal** is the default state.
- A transaction can be moved to **Disputed** via `dispute`.
- A disputed transaction can be **resolved** back to **Normal** or **charged back** (terminal state).

```mermaid
stateDiagram-v2
    [*] --> Normal

    Normal --> Disputed : dispute
    Disputed --> Normal : resolve
    Disputed --> Chargeback : chargeback

    Chargeback --> [*]
```