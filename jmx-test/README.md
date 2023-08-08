# Build & Run
```
podman build . -t jmxtest:latest

podman run -p 9991:9991/tcp localhost/jmxtest:latest
```
# test metrics
```
wget https://github.com/jiaqi/jmxterm/releases/download/v1.0.1/jmxterm-1.0.1-uber.jar

echo "get -s -b com.jmx.test.basic:name=simple *" | java -jar jmxterm-1.0.1-uber.jar -l localhost:9991 -v silent -n
```