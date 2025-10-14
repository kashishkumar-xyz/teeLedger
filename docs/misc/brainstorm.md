# Initial planning and ideation
>
> What needs to be achieved with this project

- Maintain simple records of owes & dues for personal records and bookkeeping
- Phase01: Only monetary value based bookkeeping
- Phase02: Add product equivalent monetary value tracking, with integrated simple inventory system
  - inventory system to just track amount bought and its price, with output showing costprice, % profit margin - price calculator, pricepoint price calculator etc.needs

> Things to think 💭

- data mutable or immutable
  - (?) why not have both modes
  - lets keep it immutable, but instead of showing mutable or immutable modes, we have normal mode and show-history option
      `show-history option will just expand each transaction entry and display its history`
- **How will tx-settlement be handled?** that is how will we record when  someone pays us back or viceversa
  - tick it off from that persons displayed balance sheet?
  - and / or just do opposite sign(flow)

## Project Technical needs

1. Database: sqlite
2. Data Structure: TBD, maybe linked-list
3. AI helper: opencode w/ geminiCLI
4. Language: JS
5. non-server based for security reasons, app must be self-contained local, with industry-standard encryption

### Proposed Tx DTO syntax (standardized)

> `{ person, tx_dir, amount, <_metadata_> }`

```text
  person: refers to the other party with whom the transaction occured.

  tx_dir: the flow of transaction, this also identifies tx type. 
    . if (+); the person owes us money, we are due to recieve it
    . if (-); we owe the person money, we have to pay them
    . if (0); add a new user to our db and initialize their account and balance sheet
        . when adding tx for a person who doesnt exist in db, then a (0) tx will automatically be inserted to their individual-scoped ledger followed by the valued (+/-) entry
        . (0) entry must not carry a value, any in the field will show warning and be treated as tx-notes
    . if (*); entry with no monetary effect on the ledger/balance, its just to add tx-notes and related qualitative data

```

> standardized rule implies that all transactions irrespective of the intent must follow the syntax.

## Data Structures ideation

1. Standard txDTO

```javascript
{
  PERSON: _uniqueStrign,
  TX_DIR: _enum(0, +, -, *),
  AMOUNT: _unsigint,
  METADATA: _OBJECT {timestamps, notes, ...}
}
```

2. Account

```javascript
genrateEntry(txDTO) = signedInt | amount  
PERSON_ACC [+amount , -amount, +amount]
BALANCE = PERSON_ACC.sum()
```
