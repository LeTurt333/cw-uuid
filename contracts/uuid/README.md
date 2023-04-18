# cw-uuid (WIP)


### Contract that utilizes [Nois Randomness](https://github.com/noislabs) and [uuid-rs](https://github.com/uuid-rs/uuid) to generate [UUIDS](https://datatracker.ietf.org/doc/html/rfc4122)


---
---
---


Notes to self


Issue:
- In cosmos SDK 47, "response.data" is deprecated
- have default fee to ensure that the contract has enough to execute callback if moving to
solution that doesn't use data, & to pay Nois Proxy fee

Note that the "secureness" of the Randomness doesn't really apply here,
meaning it doesn't matter whether someone can pre-compute the UUID generated
because each UUID can only collide with other UUIDs within the calling contract

The main concern is
"What happens if there is a collision in the calling contract with the UUID they are provided?"
"Does the caller's usage of the UUID just fail, and the fee they paid is wasted?"

```
If you were to generate one billion UUIDs per second, 
it would take you approximately 1.684 × 10^20 years, 
or 16,840,000,000,000,000,000 years, to exhaust all the possible UUIDs
```

To mitigate this:
1) There's an incrementer in state which is used on every as a Sub-Randomness Key
2) Fallback value / Get multiple UUIDs (test for gas issues & needs solution)