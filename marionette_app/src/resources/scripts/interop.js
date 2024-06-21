window.internalRequest = async function(method, data, no_response = false, avoid_log = false) {
    if (!avoid_log) window.requested({ method: method, data: data });

    window.dioxus.send({
        method: method,
        data: data
    });

    if (no_response) return;

    let response = await window.dioxus.recv();
    if (!avoid_log) window.received(response);
    return response;
}

// THE CODE BELOW IS 100% GOING TO BE OVERRIDEN
window.received = function(data) {
    let content = JSON.parse(data);
    if (content.status == "ok") {
        console.log("RECV", content.status, data);
    } else {
        console.log("RECV", content.status, data);
    }
}

window.requested = function(data) {
    data = JSON.stringify(data);
    console.log("CREQ", data);
}

window.error = function(data) {
    // TODO: behavior is undefined currently.
    data = JSON.stringify(data);
    console.log("CERR", data);
}