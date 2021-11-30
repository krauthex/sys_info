# `sys_info`
This is primarily meant as an exercise to learn Rust, but should at least serve the purpose of being used as a tool that gathers additional information when used as a custom fact for ansible.
For this, compile the code via `cargo build --release`, then place the binary `target/release/sys_info` into the `/etc/ansible/facts.d/` folder on the machines you want the additional information, and add the `.fact` extension, such that ansible knows to use it (e.g. `sys_info.fact`). 

You can test it locally with `ansible localhost -m setup`, and search the output for the `ansible_local` section, which will then contain a dictionary with the information:

```
...
"ansible_local": {
    "sys_info": [
	{
	    "cmd": "python",
	    "status": 0,
	    "stderr": "",
	    "stdout": "Python 3.9.5"
	},
	{
	    "cmd": "python2",
	    "status": 127,
	    "stderr": "sh: line 1: python2: command not found",
	    "stdout": ""
	},
	{
	    "cmd": "conda",
	    "status": 0,
	    "stderr": "",
	    "stdout": "conda 4.10.3"
	},
	{
	    "cmd": "docker",
	    "status": 0,
	    "stderr": "",
	    "stdout": "client: 20.10.10, api: 1.41"
	}
    ]
},
...
```

In this case, I don't have `python2` installed on my machine, hence the exit code `127`. 
