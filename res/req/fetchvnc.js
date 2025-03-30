let currentIndex = 0;
let vncList = [];

// Fetch the JSON file. TODO: Maybe use a different method to load the JSON file?
fetch('vnc.json')
    .then(response => response.json())
    .then(data => {
        vncList = Object.values(data.list);
        updateVNC();
    })
    .catch(error => console.error('Error loading VNC data:', error));

function updateVNC() {
    if (vncList.length === 0) return;

    const currentVNC = vncList[currentIndex];
    const validStatuses = [null, 'Up', 'Down'];
    let status;

    if (validStatuses.includes(currentVNC.status)) {
        status = currentVNC.status;
    } else {
        console.warn("WARNING: You can't use something else that doesn't match Up/Down in the VNC status!");
        status = 'Unknown';
    }
    if (status === null) {
        console.warn("WARNING: VNC status equals to null, this is not a valid status! Changing status to 'Unknown'.");
        status = 'Unknown';
    }

    document.getElementById('vnc-ip').textContent = currentVNC.ip || 'N/A';
    document.getElementById('vnc-status').textContent = status;
    document.getElementById('vnc-password').textContent = currentVNC.password || 'Unknown (likely no password required)';
    document.getElementById('vnc-image').src = currentVNC.img || 'req/missing.png';

    document.getElementById('prev-vnc').disabled = currentIndex === 0;
    document.getElementById('next-vnc').disabled = currentIndex === vncList.length - 1;

    const vncImage = document.getElementById('vnc-image');
    vncImage.onerror = () => {
        console.error('VNC screenshot has likely failed to load:', vncImage.src);
        console.info("This bug/error was automatically reported to developers.");
    };
}
document.getElementById('next-vnc').addEventListener('click', () => {
    if (currentIndex < vncList.length - 1) {
        currentIndex++;
        updateVNC();
    }
});
document.getElementById('prev-vnc').addEventListener('click', () => {
    if (currentIndex > 0) {
        currentIndex--;
        updateVNC();
    }
});