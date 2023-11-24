const url = new URL(window.location.href);
const searchParams = url.searchParams;

const uuid = searchParams.has("uid") && referrer.includes(window.location.host) ? searchParams.get("uid") : crypto.randomUUID();

let urlWithoutUid = url.searchParams.delete("uid");

history.replaceState({}, "", urlWithoutUid);
let datetime_entered = Date.now();
let datetime_entered_isostring = new Date().toISOString();

document.addEventListener("load", () => {
let links = document.querySelectorAll('a');

for (let i = 0; i < links.length; i++) {
	if (links[i].href.includes(window.location.host)) {
		links[i].href += "?uid=" + uuid;
	}
}
})

document.addEventListener("visibilitychange", send_logs(), false);

async function send_logs() {
	if (!Document.hidden) {	
	let res = await fetch("{{domain}}/push", {
		method: "POST",
		keepalive: true,
		headers: {
			"content-type":"application/json",
		},
		body: JSON.stringify({
			"uri": window.location.href,
			"sessionUuid": uuid,
			"domain": window.location.host,
			"dateTime": datetime_entered_isostring,
			"duration": 0	
		}),
	});
	}
} 
