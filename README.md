# Docker Compose Example for Windows Containers

This Is a minimal example of two Docker containers, on a virtual network talking to eachother.
The example code is in Rust.


This is going to only work on windows.<br/>
Check the DockerFiles inside cont1 and cont2 to make sure that your base windows image is atleast build 1903 for the line `mcr.microsoft.com/windows/servercore:1903`
(Otherwise change it to your version, this setup should work on newer setups already)

In this example, you build each of the projects in your base OS and the executables get copied into their respective containers.

### Requirements
```
cargo 1.46.0        (rust compiler, can get it from rustup.rs)
Docker for windows  (https://docs.docker.com/docker-for-windows/install/)
```

### To build
From the root of this project
`powershell.exe ./build.ps1`

### Usefull Commands
```
docker-compose rm -f 
docker-compose build
docker-compose up
```


