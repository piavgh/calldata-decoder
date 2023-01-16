# Calldata Decoder

![]("./auto_raw_calldata.png")

A program to automate the progress discussed in [DeGatchi](https://twitter.com/DeGatchi)'s article, [Reverse The EVM: Raw Calldata](https://degatchi.com/articles/reading-raw-evm-calldata).

By using the knowledge of how method selectors and dynamic and static variables are encoded we are able to create heuristics to detect where the parameters are meant to be.

We guess potential types by using the knowledge of how types are typically represented in calldata and what similarities they share between one another.
