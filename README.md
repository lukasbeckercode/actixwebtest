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

## GitHub Workflows 
Since the Project is hosted on GitHub, GitHub Workflows are used as a CI Pipeline. The Pipeline clones the repository, 
runs the Docker Container for the Postgres Database and installs Diesel CLI to set the Database up. For the Database setup, 
the Workflow utilizes the [create_env.sh](create_env.sh) script, which creates a .env file based on the GitHub Secrets stored. This .env file 
is then used for the database name, username and password. Once the database is created, Diesel CLI is used to enter the test data. This is doen 
by the [run_db.sh](run_db.sh) script.   
After this initial Setup, cargo test is run and the test results are saved to the artifact. The database then gets reset and a code coverage 
using tarpaulin is generated and also saved to the artifact. 
This pipeline is invoked each time a push to main is made. It can also be invoked manually. 
The definition file for the pipline can be viewed [here](.github/workflows/main.yml)
## CI/CD Checklist
The provided checklist was used for reference and can be found [here](Checklist.md). 

## Setup 
To use this Project, Docker is required on either MacOS or Linux. The helper Scripts were not ported to work with Windows yet, 
however the project itself is fully containerized so no issues should arise when running on windows.   
First, create a .env file by running [create_env.sh](create_env.sh) like this:   
``` 
./create_env.sh <DB_USERNAME> <DB_PASSWORD>
```
Then, simply start the Project by calling [run_setup_full.sh](run_setup_full.sh)    
If necessary, use ```chmod +x <filename>``` to make the .sh scripts runnable. 

## Usage 
Once the Project is running, it can be used as follows:    
``` 127.0.0.1:7878``` to see the index.html file.   
A POST request to
```127.0.0.1:7878/add_part ```
with some JSON Data like 
``` JSON
{
    "id": 9,
    "description": "Part 9",
    "num_actual": 8,
    "num_expected": 12 
}
```
adds a part to the inventory.    
A GET request to 
```127.0.0.1:7878/<PART_ID>```
retrieves the part with <PART_ID> and delivers a response as follows: 
```
Part No. 1 found! 
Description: Test 2
Current Delta: 4
```


A DELETE request to
```127.0.0.1:7878/<PART_ID>```
deletes the part with <PART_ID>

## Known Limitations
- There is no UPDATE functionality yet. The main focus for this project was on Docker and the CI Pipeline. 
- There is no Frontend. It is best to test the functionality of the Project using Postman or similar tools. 
- The CI Pipeline doesn't fail if Unittests are failing. This is because I could not find a cargo test plugin for GitHub Workflows. 
- Error Messages to the user. I need to dive deeper into rusts error handling system to provide good error messages and HTTP Status codes to the user. 

