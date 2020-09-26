const hamburgerButton = document.getElementById("hamburger-button");
const navList = document.getElementById("nav-bar-list");

hamburgerButton.addEventListener("click", () => {
    navList.classList.toggle("open");
});
