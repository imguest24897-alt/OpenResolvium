document.addEventListener('DOMContentLoaded', function() {
    let explore = document.getElementById('explore-button');
    let viewMeOnGitHub = document.getElementById('view-me-on-github');

    if (explore) {
        explore.onclick = function() {
            window.location.href = 'browse.html';
        }
    }
    if (viewMeOnGitHub) {
        viewMeOnGitHub.onclick = function() {
            window.location.href = 'https://github.com/imguest24897-alt/OpenResolvium';
        }
    }
});