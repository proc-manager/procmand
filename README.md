# procmand

Daemon that manages processes in a container-like environment. 


```shell
sudo nsenter --target <PID> --pid --mount --uts --preserve-credentials /bin/sh
```


## Todo 

### Prepare Namespaces

- [x] Mnt
- [x] Pid
- [x] Uts
- [x] User
- [x] Net
- [ ] Cgroups

### Exception Handling 

Currently, we just panic using expect. 
Once the namespaces are properly set up, need to write the exception propagation. 

