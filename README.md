# simple-server
![Rust workflow](https://github.com/aradwann/simple-server/actions/workflows/rust.yml/badge.svg)

 a simple multithreaded TCP/HTTP server built with Rust

## components
### HTTP server 
which listens to the incoming TCP/HTTP requests and handle them 

#### TCP Listener: 
listens to incoming tcp connections and bound be specified address
#### connection handler: 
handles the incoming tcp streams / requests by triggering a specified functionality
#### pool: 
a thread pool containing a specified number of threads to delegate the request handling work to

### ThreadPool
which have a pool of workers that are waiting for jobs to do, thus enabling handling more than on job at a time
channels [multi-producer single consumer] are used as a way of communication between threads

#### ThreadPool:
workers: a vector of workers own the recieving end of the channel and waiting for jobs to handle
sender end of the channel: used by the thread pool to send jobs to workers

#### worker:
has an id and a thread 
the thread is spawned and loops over the receiving end of the channel waiting for jobs to handle 
once it receives a job, it executes it 
