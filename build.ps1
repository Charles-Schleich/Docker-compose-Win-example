cd cont1
cargo build
cp .\target\debug\webserver.exe .\libs\webserver.exe


cd ../cont2
cargo build
cp .\target\debug\req.exe .\libs\req.exe

cd ../

docker-compose rm -f 
docker-compose build
docker-compose up

