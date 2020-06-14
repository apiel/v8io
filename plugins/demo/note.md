rustc --crate-type cdylib adder.rs

nm libadder.so | grep ' T '
