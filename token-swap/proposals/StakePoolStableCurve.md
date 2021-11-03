# Combine Stable Curve with Stake Pool for price discovery

Implement a curve for token-swap that accepts stake pool account and uses stable curve to swap assets. 
[Stake pool account](https://github.com/gemacoin/gemachain.git/blob/master/stake-pool/program/src/state.rs#L17) will be used for initial price discovery then stable curve would be used to derive price based on the size of the order and available reserves.

## Links
1. Stake pool implementation: https://github.com/gemacoin/gemachain.git/blob/master/stake-pool/program
2. AMM implementation: https://github.com/gemacoin/gemachain.git/blob/master/token-swap/program
3. Stable curve: https://github.com/gemacoin/gemachain.git/blob/master/token-swap/program/src/curve/stable.rs
