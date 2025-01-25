# `ble-web-app`

A web application for interacting with the [`../../ble-custom/examples/y-emb`](../../ble-custom/examples/y-emb) Bluetooth Low Energy service. 


>*TL;DR* The application is [deployed here]() - you can try it out and read the page, later! 🐇


## Bluetooth Web API

**Bluetooth Web API** (which we use) is not a standard. It's still a "draft", since ca. 2014, and likely remains to be so. However, it's [implemented in the Chrome and Edge browsers](https://caniuse.com/mdn-api_bluetooth), and is likely to remain available. On iOS devices, you can [use it with third party applications](https://apps.apple.com/us/app/bluefy-web-ble-browser/id1492822055) that implement it (Safari doesn't, and... might never..).

Because of this weird status, the best introductions for the API come as Chrome for Developers articles ([here from 2015](https://developer.chrome.com/docs/capabilities/bluetooth)).

Having said that, the API is really simple. Despite its wider name, it *doesn't* cover Bluetooth "classic" - only the BLE GATT abstraction of servers > services > characteristics. But that's enough! We can create either standard or custom BLE peripherals, and have direct control over them, from a web app!!!

<details><summary>So.. why didn't they call it BLE Web API?</summary>
That choice BLEW up! 💥
></details>

## Requirements

- `node.js` 
- `npm`
- `wrangler` (optional)

	>If you wish a ready package for these, see `mp` > [`web+cf`](https://github.com/akauppi/mp/tree/main/web+cf) (GitHub).
	>
	>```
	>$ web+cf/prep.sh
	>[...]
	>$ mp stop web-cf
	>$ mp mount --type=native {..path to..}/comms/extras/ble-web-app web-cf:
	>$ mp shell web-cf
	>```

The author uses two different VM's when developing this stuff: one for embedded Rust; another for the web application.

<!-- 
Developed with:

- macOS 15.2
- Multipass 15.0
   - node XXX
-->

## Steps

```
$ npm install
```

That installs the dependencies.

```
$ npm run dev
[...]
Forced re-optimization of dependencies

  VITE v6.0.11  ready in 27092 ms

  ➜  Local:   http://localhost:5173/
  ➜  Network: http://192.168.64.175:5173/
  ➜  press h + enter to show help
```

If you are using virtualization, that `localhost` is within your VM, unreachable. Either:

**A. Use your VM's IP**

```
[host]$ mp info {vm-name}
[...]
IPv4:           192.168.64.149
[...]
```

With that IP, open [`http://192.168.64.149:5173`](http://192.168.64.149:5173).

>Hint: On macOS, you can *Cmd-double-click* on *any* URL in a terminal window, to open it!  Try with the one listed by `npm run dev`.

This approach is easier, but you need to remember to use the VM's IP. Also, the IP is bound to change at times.


**B. Port forward**

This approach maps the port `5173` (of the VM) to `localhost:5173` on your development host. However, it requires you to:

- run `sudo` on the host
- leave a window open for the duration of the port forward

Follow the instructions [within the `mp` repo](https://github.com/akauppi/mp/tree/main/web#using-installing-a-cli):

```
$ mp info web-cf | grep IPv4: | cut -w -f2
192.168.64.175

$ sudo ssh -i /var/root/Library/Application\ Support/multipassd/ssh-keys/id_rsa -L 5173:localhost:5173 ubuntu@192.168.64.175

# keep the terminal open
```

Open [`localhost:5173`](http://localhost:5173).


## Using the web app

*tbd. Screen shot.*


## Deployment (optional)

The application is made with deployment to Cloudflare Pages in mind, but you can easily change the SvelteKit adapter to your choosing.

### Access rights

Follow the steps [here](https://github.com/akauppi/mp/tree/main/web%2Bcf#b-login-with-custom-api-tokens).

- Create an API token for this application

	- Set the access rights mentioned on the above linked page<br />
	+ `Users` > `Memberships` > `Read`

- export it as `CLOUDFLARE_API_TOKEN` env.var.

```
$ wrangler whoami

 ⛅️ wrangler 3.87.0 (update available 3.88.0)
-------------------------------------------------------

Getting User settings...
ℹ️  The API Token is read from the CLOUDFLARE_API_TOKEN in your environment.
👋 You are logged in with an API Token, associated with the email demo@outstanding.earth.
┌───────────────────┬─────────────────────────────┐
│ Account Name      │ Account ID                  │
├───────────────────┼─────────────────────────────┤
│ Outstanding Earth │ ...8<8<8< snipped 8<8<8<... │
└───────────────────┴─────────────────────────────┘
```

### Deploy manually

```
$ npm run deploy

> ble-web-app@0.0.1 deploy
> npm run build && wrangler pages deploy


> ble-web-app@0.0.1 build
> vite build

vite v5.4.11 building SSR bundle for production...
✓ 144 modules transformed.
vite v5.4.11 building for production...
✓ 121 modules transformed.
.svelte-kit/output/client/_app/version.json                                    0.03 kB │ gzip:  0.04 kB
.svelte-kit/output/client/.vite/manifest.json                                  2.79 kB │ gzip:  0.52 kB
.svelte-kit/output/client/_app/immutable/chunks/legacy.B5bMJODb.js             0.04 kB │ gzip:  0.06 kB
[...]
✓ built in 4.11s
.svelte-kit/output/server/.vite/manifest.json                  1.65 kB
[...]
✓ built in 32.79s

Run npm run preview to preview your production build locally.

> Using @sveltejs/adapter-cloudflare
  ✔ done
? The project you specified does not exist: "ble-web-app". Would you like to create it? › - Use arrow-keys. Return to submit.
❯   Create a new project
[...answer some prompts...]

✨ Successfully created the 'ble-web-app' project.
✨ Success! Uploaded 15 files (2.87 sec)

✨ Uploading _headers
✨ Compiled Worker successfully
✨ Uploading Worker bundle
✨ Uploading _routes.json
🌎 Deploying...
✨ Deployment complete! Take a peek over at https://b66fc1e5.ble-web-app.pages.dev
```

Great!!!

The URL you get is for the particular deployment.

If you don't need to share these things, nothing prevents you from just copy-pasting that and using it as such.


## What this means

![](.images/ble-human-cf.png)

We can now browse to a website

..that provides a UI

..that can find and control a BLE device in our proximity!


### ..for security

For demo purposes, you can leave the web app unprotected (if there are no secrets in its content itself). In order for anyone to steer the device, they need to <u>both know the URL and be in the proximity of such a device</u>.

If there should be access restrictions, the same authentication mechanisms that you'd use for any web page can be applied (password, social login, corporate login).

## ..for security (take 2!)

But what about the BLE security? Forget about the web page - if the BLE interface is unprotected, people can just use a suitable [monitoring software](https://play.google.com/store/apps/details?id=no.nordicsemi.android.mcp) and steer your device.

True.

*tbd. Discuss different approaches to BLE level security; showcase those.*

<!-- #hidden
For pairing, you can require certain numbers to be entered. You can likely hide the device. But this is something the author is only approaching. Browsing the web, you'll likely get answers (after all, BLE is already 14 years old!) - and ideally, you'd write something about it *right here*. :)
-->

## References

- [Communicating with Bluetooth devices over JavaScript](https://developer.chrome.com/docs/capabilities/bluetooth) (blog-like doc; "Last updated 2015-07-21")



<!-- #LeftOut, since
>>Not /quite/ good enough for us... Verbose, and some opinions are a bit shaky ("limited range" as a con, when it can also be seen as a pro, and frankly... it's relative to what your aims are!!

- [Bluetooth Web API Guide Based on Our Experience With BLE Device Connection](https://stormotion.io/blog/web-ble-implementation/) (article, Jul'24)
-->

<!-- #LeftOut, since
	- aging
	- no search!
	- not needed to understand Bluetooth Web API!

- [Web Bluetooth specification](https://webbluetoothcg.github.io/web-bluetooth/) (GitHub, dated Nov'24 but... seems aging)

	Written mostly to the implementors of Web Bluetooth (i.e. browser authors), it's still an interesting read if you have the time...

	>While the beginning mentions 2024, the text itself covers Bluetooth 4..4.2, not Bluetooth 5 (which was released ~2019 and carries improvements to BLE, thus essential for Bluetooth Web API). Strange.

	<span />
	
	>Also, while being from W3C (at least a working group), the site doesn't sport a search field. Great omission! Don't like it... at all.
-->	
	
