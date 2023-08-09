# Build & Run
```bash

# download java
curl -O -L https://download.oracle.com/java/20/latest/jdk-20_linux-x64_bin.rpm
rpm -i jdk-20_linux-x64_bin.rpm

# compile and create jar
curl -O https://raw.githubusercontent.com/ydguala/demos/main/jmx-test/SimpleJmx.java
curl -O https://raw.githubusercontent.com/ydguala/demos/main/jmx-test/manifest.mf
javac SimpleJmx.java
jar cfm SimpleJmx.jar manifest.mf *

# execute jmx server
## local ()
java -Dcom.sun.management.jmxremote=true -Djava.rmi.server.hostname=192.168.3.106 -Dcom.sun.management.jmxremote.port=9991 -Dcom.sun.management.jmxremote.ssl=false -Dcom.sun.management.jmxremote.registry.ssl=false -Dcom.sun.management.jmxremote.authenticate=false -Djava.net.preferIPv4Stack=true -jar SimpleJmx.jar

## container
podman build . -t jmxtest:latest

podman run -p 9991:9991/tcp localhost/jmxtest:latest
```

# Push to docker
```bash
podman push localhost/jmxtest:latest docker.io/ydguala/jmxtest:latest
```

# test metrics
```bash

# download Java
curl -O -L https://download.oracle.com/java/20/latest/jdk-20_linux-x64_bin.rpm
rpm -i jdk-20_linux-x64_bin.rpm

# download jmxterm to check jmx metrics
curl -O -L https://github.com/jiaqi/jmxterm/releases/download/v1.0.1/jmxterm-1.0.1-uber.jar

# create pod
kubectl apply -f https://raw.githubusercontent.com/ydguala/demos/main/jmx-test/resources/svc-deploy.yaml

# forward to localhost
kubectl port-forward jmx-test-74957d96b4-596lr 9991:9991
Forwarding from 127.0.0.1:9991 -> 9991

# test local
echo "get -s -b com.jmx.test.basic:name=simple *" | java -jar jmxterm-1.0.1-uber.jar -l localhost:9991 -v silent -n

# test remote (server should be using -Djava.rmi.server.hostname=192.168.3.106 )
echo "get -s -b com.jmx.test.basic:name=simple *" | java -jar jmxterm-1.0.1-uber.jar -l  192.168.3.106:9991 -v silent -n

# response
Paradigma tests
```
