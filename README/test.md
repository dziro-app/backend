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


🔬 **Show output**
```bash
$ cargo test -- --show-output
```

---

## **📖 Available module tests**

- users
- user_repository
- whishlists
- collection_repository

