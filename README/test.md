# 🔬 Tests

Tests are organized by modules and intention, for example:
> ***Module***: whishlists

- **Unit test**:  
  - *Normally located in the same file of the tested code*.  
  - *Test function name should use **ut** prefix: ut_test_\<name>*

- **Integration tests**:  
  - *Normally located at the tests directory at root project*.  
  - *Test function name should use **it** prefix: it_test_\<name>* 

🔬 **Run all tests**
```bash
$ cargo test
```

🔬 **Run all module unit tests**
```bash
$ cargo test whishlists::
```

🔬 **Run all module integration tests**
```bash
$ cargo test --test whishlists
```

🔬 **Show output**
```bash
$ cargo test -- --show-output
```

---

## **📖 Available module tests**

- mongo
- whishlists

---

## **⚠️ Important**

The app containes *expensive tests*.

Before runing them, be sure to have a mongodn similar to the one specified under docker/compose.yml file.

🔬 **Run expensive tests**
```bash
$ cargo test -- --ignored
```