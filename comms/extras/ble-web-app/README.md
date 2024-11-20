# `ble-web-app`

A web application for interacting with a local BLE embedded product.

`../ble/`: source code for the BLE device


## Requirements

- `node.js` 
- `npm`

If you wish a ready package, see [`mp > web`](https://github.com/akauppi/mp/tree/main/web) (GitHub).

## Steps

```
$ npm install
```

That installs the dependencies listed in `package.json`.

```
$ npm run dev
[...]
Forced re-optimization of dependencies

  VITE v5.4.11  ready in 27092 ms

  ➜  Local:   http://localhost:5173/
  ➜  Network: use --host to expose
  ➜  press h + enter to show help
```

If you are using Multipass for virtualization, that `localhost` is within your VM, not the host. There are two ways to access it:

**A. Use your VM's IP**

```
[host]$ mp info {vm-name}
[...]
IPv4:           192.168.64.149
[...]
```

With that IP, open [`http://192.168.64.149:5173`](http://192.168.64.149:5173).

This approach is easier, but you need to remember to use the VM's IP. Also, the IP is bound to change at times.


**B. Port forward 5173 to the host**

This approach makes the port `5173` usable - as `localhost:5173` - from your host. However, it requires you to:

- run `sudo` on the host
- leave a window open for the duration of the port forward

Unfortunately, Multipass does not have port forwarding built-in. Other virtualization tools do, so you will have an easier ride with them, in this regard at least.

Follow the instructions [within the `mp` repo](https://github.com/akauppi/mp/tree/main/web#using-installing-a-cli):

```
$ sudo ssh -i /var/root/Library/Application\ Support/multipassd/ssh-keys/id_rsa -L 5173:localhost:5173 ubuntu@192.168.64.149

# keep the terminal open
```

Open [`localhost:5173`](http://localhost:5173).



  
# or start the server and open the app in a new browser tab
npm run dev -- --open
```

## Building

To create a production version of your app:

```bash
npm run build
```

You can preview the production build with `npm run preview`.

> To deploy your app, you may need to install an [adapter](https://svelte.dev/docs/kit/adapters) for your target environment.



---
## Deployment (optional)

The application is made with deployment to Cloudflare Pages in mind, but you can easily change the SvelteKit adapter to your choosing - or just try it locally.
