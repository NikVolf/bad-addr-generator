# bad address generator

Generates ethereum addresses starting with with `0x00bad0` until stopped. Utilizes all cores!

```
cargo run --release

...
300000 generated
Found bad ass address:
secret:  f995cb781bdaa55746052eee0d799b2fec10a1799542c075539ff468a1d8d6cd
public:  3bdc6f33b255844011d12e77f1c5f38072b6a9cf2940d192d03c15bdb26145a7bbefe8ca138118f3d83e8f3a4978a0115c2a6149376f557449f219178456a545
address: 00bad0cf2aa977124565b9860ecc5a64e46e410f
400000 generated
...
```

expected time to get bad address is 16,000,000 with 50% prob.