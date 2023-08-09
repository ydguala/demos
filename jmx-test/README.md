# Build & Run
```
podman build . -t jmxtest:latest

podman run -p 9991:9991/tcp localhost/jmxtest:latest

# Push to docker
```
podman push localhost/jmxtest:latest docker.io/ydguala/jmxtest:latest
```

```
# test metrics
```
https://download.oracle.com/java/20/latest/jdk-20_linux-x64_bin.rpm
rpm -i jdk-20_linux-x64_bin.rpm


wget https://github.com/jiaqi/jmxterm/releases/download/v1.0.1/jmxterm-1.0.1-uber.jar
curl -O -L https://github.com/jiaqi/jmxterm/releases/download/v1.0.1/jmxterm-1.0.1-uber.jar

oc port-forward jmx-test-74957d96b4-596lr 9991:9991
Forwarding from 127.0.0.1:9991 -> 9991

echo "get -s -b com.jmx.test.basic:name=simple *" | java -jar jmxterm-1.0.1-uber.jar -l localhost:9991 -v silent -n
```
