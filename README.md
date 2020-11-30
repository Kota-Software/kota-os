# Kota OS

## Bare Bones

Based on:

- https://lowenware.com/blog/osdev/aarch64-bare-metal-program-in-rust/
- https://os.phil-opp.com/freestanding-rust-binary/

### Running

- `rustup override set nightly`
- `rustup target add aarch64-unknown-none`
  - May not be necessary now that we've got the `.json` target spec
- `cargo build --release`
  - ![](https://firebasestorage.googleapis.com/v0/b/firescript-577a2.appspot.com/o/imgs%2Fapp%2FKota-OS%2FOIcgeQDkUi.png?alt=media&token=a975677e-94c9-4908-8ed0-adb409fab8ed)
- ```shell
  qemu-system-aarch64 -machine virt \
  	-m 1024M \
  	-cpu cortex-a53 \
  	-nographic \
  	-kernel target/aarch64-unknown-none/release/kota_os
  ```

  - Should output `Hello World!`
