/*
    Project: Catherine Framework (https://github.com/azazelm3dj3d/catherine)
    Author: azazelm3dj3d (https://github.com/azazelm3dj3d)
    License: BSD 2-Clause
*/

const { invoke } = window.__TAURI__.tauri

// Decode a string
const decodeString = () => {
    invoke('decode_string', {
        methodName: document.getElementById("methodName").value,
        encodedData: document.getElementById("encodedData").value
    }).then((response) => {
        document.getElementById("res_0").innerHTML = response;
    })
}

// Dump host system information
const sysInfo = () => {
    invoke('sys_info').then((response) => {
        document.getElementById("res_1").innerHTML = response;
    })
}

// Defang a string
const defangString = () => {
    invoke('defang_string', {
        defangValue: document.getElementById("defangValue").value
    }).then((response) => {
        document.getElementById("res_2").innerHTML = response;
    })
}

// WHOIS url lookup
const whoisLookup = () => {
    invoke('whois_search', {
        whoisUrl: document.getElementById("whoisUrl").value
    }).then((response) => {
        document.getElementById("res_3").innerHTML = response;
    })
}

// Clears the WHOIS results to cleanup the UI
const clearWhoIs = () => {
    document.getElementById("res_3").innerHTML = "";
}

// Malicious domain search
// const malSearch = () => {
//     invoke('malicious_search', {
//         malUrl: document.getElementById("malUrl").value
//     }).then((response) => {
//         document.getElementById("res_4").innerHTML = response;
//     })
// }

// Identify an unknown string
const idString = () => {
    invoke('id_string', {
        idStr: document.getElementById("idStr").value
    }).then((response) => {
        document.getElementById("res_5").innerHTML = response;
    })
}

// Attempt to crack an unknown hash
const crackHash = () => {
    invoke('crack_hash', {
        hashCracker: document.getElementById("hashCracker").value
    }).then((response) => {
        document.getElementById("res_6").innerHTML = response;
    })
}

// Generate a domain for DNS squatting
const domainGen = () => {
    invoke('domain_gen', {
        domainStr: document.getElementById("domainStr").value
    }).then(() => {
        document.getElementById("res_7").innerHTML = "Check terminal";
    })
}

// Extract a zip file contents
const zipExtract = () => {
    invoke('extract_zip', {
        extractZipFile: document.getElementById("extractZipFile").value
    }).then(() => {
        document.getElementById("res_8").innerHTML = "Check terminal";
    })
}

// Exit application
const exitCatherine = () => {
    invoke("exit_catherine")
}