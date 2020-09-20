
function initialize() {
    document.querySelectorAll("main > button").forEach((button) => {
        button.addEventListener("click", () => {
            fetch(`/press/${button.dataset.uuid}`, {
                method: "POST"
            });
        });
    });
}

window.addEventListener("DOMContentLoaded", initialize);
