15/Jun/2024
1. This could be a library crate for use later, but now I need to implement the Paxos example.
2. Value transferred by network should be seriallized. But now just use u32 to keep simple.
3. Connection and messages sending need to be asynchronously implememted by Tokio.
4. I need several other instances using this code to be proposer, acceptor and learner. I am not sure if I can place tham in a bin folder like in tokio tutorial while keeping use the lib code.