Rust MQTT Bridge POC for Resion.io
=======

Demo code to show:

* Rust lang on a RPI Zero
* Rust lang on a resion.io Dockerfile
* use of config.rs w/ ENV vars

Works fine if you are willing to edit the hardcoded arm6l for zero and arm7l for RPI 3 in the Dockerfile.template (otherwise the host builder arm8l from `uname` will blow things up)


todo: switch the Dockerfile.template to examples for inclusion in multi-container resin.io builds and make a proper Dockerfile for non-iot stuff

