document.addEventListener('DOMContentLoaded', function() {
    let explore = document.getElementById('explore-button');

    if (explore) {
        explore.onclick = function() {
            window.location.href = 'browse.html';
        }
    }
});