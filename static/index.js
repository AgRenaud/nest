
async function search() {
    const results = document.getElementById("results")
    results.innerHTML = "";
    const response = await fetch("/simple/", {
        method: 'GET',
        headers: {'Content-Type': 'text/html'},
    });
    const json = await response.json();
    results.innerHTML = "";
    for (package of json['package']) {
        let item = document.createElement("span");

        item.appendChild(document.createTextNode(package['name']));
        item.appendChild(document.createElement("br"));

        results.appendChild(item);
    }
}

await search();
