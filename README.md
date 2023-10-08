# Actin' Up

## Motivation 

A small, first run, implementation of the Actor pattern in Rust!

The challenge was the following: 

1. Create a tokio task that contains a hashmap (can choose what data it maps). This will be the "server".

2. The server recieves messages via a tokio MPSC channel. This message communicates what operation is to be perfomred on the hashmap. For now the operations are:
    - Insert key-val pair
    - Return a clone of a value using a key
    - Remove a value and return it

3. *Bonus*: Add a fourth operation that allows the caller to perform an arbitrary query on a value in the map and return the result of that query. In otherwords,
        give the server something like a `F: FnOnce(&Value) -> T` (or almost any T) and return `T` to the caller.


## Notes on Implemenation 

I worked to allow for a generic set of data to be mapped across our system and believe that it will continue to serve well assuming the `Message` type is flexible enough given the various trait bounds placed on it.

In practice though, the data we are working with is `&str` (string slices). The "bonus" returns a `T` of `QueryResponse` with the resaoning being this would let us expand the options later on without requiring the entire setup to be modified since it would just be new variant on the model. 

## Thoughts for the future

- [ ] Consider switching to an bounded channel and then implemeting a mailbox
    - this will help prevent OOM issues due to the buffering of messages.
- [ ] Investigate the feasibility of implementing and using an "Oracle" to help nodes find each other. 
    - This could lead to either actors all supporting the network, or simply passing the message on to a specified node that would then return a response on the appropriate channel embedded in the `Message`.