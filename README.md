# Testaun

Very minimalist procedural macro to help with tests, when you have to run some piece of code right **before** and **after** the actual test. You know, some sort of **setup** and **teardown** kind of thing.

I wrote it with educational purpose as I'm exploring Rust's [procedural macros](https://doc.rust-lang.org/reference/procedural-macros.html) feature. But to be honest it kinda works and I may get to use it on [Moy Sekret](https://github.com/leandrosilva/moy-sekret). Let's see.

## How To

As usual, add the dependency to `Cargo.toml`:

```toml
    [dependencies]
    testaun = "0.1.0"
```

Once you have done that, it is pretty straightforward and there is a standalone test that shows it plain and simple.

```Rust
    // Import the damn crate
    //

    extern crate testaun;
    use testaun::testaun_case;

    // Those functions are mandatory to be able to use #[testaun]. You'll see an
    // error if they're not provided in the current scope.
    //

    fn testaun_before() {
        println!("[BEFORE]");
    }

    fn testaun_after() {
        println!("[AFTER]");
    }

    // Try it out just like below in order to see [BEFORE] and [AFTER] printed out:
    // $ cargo test -- --nocapture
    //

    #[test]
    #[testaun_case]
    fn should_work_fine() {
        assert!(1 == 1, "oh damn! it should be the other way around");
    }

    #[test]
    fn should_work_as_normal() {
        assert_eq!(1, 1);
    }
```

So you can use `#[testaun]` or not. If you go ahead use it, you must provide two functions `testaun_before` and `testaun_after`. And that's it.

> What a testaun Joe?

> Yeah. Big one, dude.

## Copyright

Leandro Silva <<leandrodoze@gmail.com>>
