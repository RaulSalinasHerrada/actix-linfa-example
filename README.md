# Serving Linfa with actix-web

This project serves as an example to serve Machine Learning models using `Linfa` and `Linfa-trees` and `ndarray` to deal with the data.

**HUGE** shoutouts to [Kyle Kosic](https://github.com/kykosic), as its project was used a template to build this one.

## Serving the model on local

This should make it run.

```
cd server
cargo run
```

## Client testing on local

To test the model all you need to do is

```
python client/client.py
```