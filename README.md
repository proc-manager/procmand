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
- [ ] User
- [ ] Net
- [ ] Cgroup? 
