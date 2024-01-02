# ActixWeb Inventory 
## Objective
Create a lightweight and fast API to manage a fictional inventory using rusts ActixWeb. 
## Infrastructure 
Rust is a relatively new Programming language focused on memory safety. It uses a special type-system which enables it to 
work without a Garbage Collector. Rust was used as the base programming language in this project. Actix is a web framework 
for rust, offering all of rusts benefits while also being very performant. 
In this project, Actix is used to provide an API and handle incoming GET, POST and DELETE requests.    
Diesel is an ORM and Query Builder for rust. It not only handles Database operations in the project itself, but also handles Creation and 
management, such as migration using Diesel CLI. The Project uses a Postgres Database.    
The Database is a Docker Container defined in [Dockerfile.db](Dockerfile.db). It uses the official Postgres Image as a base, sets 
Database username and password via a .env file (which is later used to work with GitHub Secrets) and exposes Port 5432.    
The API itself can be run locally but also has a [Docker image](Dockerfile), so a container can be used. This Dockerfile 
uses the latest version of the official rust image and copies the project files into the container.   
To neatly tie these two containers together, Docker Compose is used. The [Docker Compose file](docker-compose.yml) gets 
the database username and password from the .env file and binds the database to a volume. It also exposes the API to port 7878.   

## GitHub Actions 

## CI/CD Checklist
The provided checklist was used for reference and can be found [here](Checklist.md). 
## Setup 
TBD

## Usage 
TBD

## Known Limitations
TBD