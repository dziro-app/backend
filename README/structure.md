# 🧬 Project Structure


The project tries to use SOLID principles to have a better organization and implementation code. Every module may be componed taking in consideration the following concepts.

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

