const darkButton = document.getElementById("dark");
const lightButton = document.getElementById("light");
const solarButton = document.getElementById("solar");
const body = document.body;

const theme = localStorage.getItem("theme");
const isSolar = localStorage.getItem("isSolar");

const setSolar = () => {
    body.classList.add("solar");
    solarButton.style.cssText = `
        --bg-solar: white;
    `;
    solarButton.innerText = "normalize";
    localStorage.setItem("isSolar", true);
}

if (theme) {
    body.classList.add(theme);
    isSolar && setSolar();
}

darkButton.onclick = () => {
    body.classList.replace("light", "dark");
    localStorage.setItem("theme", "dark");
};

lightButton.onclick = () => {
    body.classList.replace("dark", "light");
    localStorage.setItem("theme", "light");
};

solarButton.onclick = () => {
    if (body.classList.contains("solar")) {
        body.classList.remove("solar");
        solarButton.style.cssText = `
            --bg-solar: var(--yellow);
        `;
        solarButton.innerText = "solarize";
        localStorage.removeItem("isSolar", true);
    } else {
        setSolar();
    }
};