FROM docker.io/openjdk:8
COPY . /usr/src/myapp
WORKDIR /usr/src/myapp
RUN javac SimpleJmx.java
RUN jar cfm SimpleJmx.jar manifest.mf SimpleJmx.class SimpleJmx\$Simple.class SimpleJmx\$SimpleMBean.class

CMD ["java", \
"-Dcom.sun.management.jmxremote=true", \
"-Djava.rmi.server.hostname=127.0.0.1", \
"-Dcom.sun.management.jmxremote.host=0.0.0.0", \
"-Dcom.sun.management.jmxremote.port=9991", \
"-Dcom.sun.management.jmxremote.rmi.port=9991", \
"-Dcom.sun.management.jmxremote.ssl=false", \
"-Dcom.sun.management.jmxremote.registry.ssl=false", \
"-Dcom.sun.management.jmxremote.authenticate=false", \
"-Djava.net.preferIPv4Stack=true", \
"-jar", \
"SimpleJmx.jar"]

EXPOSE 9991