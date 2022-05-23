#  Dziro backend

## Project Structure


It tries to use SOLID principles to have a better organization and implementation code. Eah module may be componed with the following concepts.

- **Infrastructure layer** *(aka **Infra**)*   
Everything related to the entrypoint of the program such as: 
  - api endpoints
  - cli commands.

- **Application layer** *(aka **app**)*  
Here are the services will help us to manage the entities in our program like users, messages, roles, files, etc... It should not have framework, database, notification or other external dependencies. It contains the behaviour and business logic.

- **Domain**  
This scope has the internal structures for mapping with the db. Idealy as in the Application layer, it should not have external dependencies but it also should define de Traits for persistant managent or similar.

- **Dtos**  
(Data transfer objects) As it's name suggest, these stucts will help us receive data and pass it to our services or stortage implementations.

## Tests

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

