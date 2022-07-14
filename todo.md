# todo

## general

- Run without GUI (or call run without gui from GUI)
- Save state

## predator

- add split_count to NN
- look a bit around as well.

## prey

- add split count to NN

## issues

- Prey doesn't multiply more than once.

## optimize

## gui

- start button
- take x number of steps
- save state

## profiling

- https://easyperf.net/blog/2019/02/09/Top-Down-performance-analysis-methodology
- https://rust-lang.github.io/packed_simd/perf-guide/prof/linux.html

rust runing with 

```
let begin_time = SystemTime::now();

println!(
    "step took: {:?}",
    SystemTime::now().duration_since(begin_time)
);
```

now and this does not increase for the entire closure after the button click. What happens after? -> The signal is emitted to draw.

solution: https://stackoverflow.com/questions/58358008/update-drawing-function-of-a-drawingarea

https://gtk-rs.org/gtk-rs-core/stable/latest/docs/glib/signal/fn.signal_handler_disconnect.html

https://stackoverflow.com/questions/8037345/how-to-disconnect-a-signal-of-gtk

https://gtk-rs.org/gtk-rs-core/git/docs/glib/signal/struct.SignalHandlerId.html

