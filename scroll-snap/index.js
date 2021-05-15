const container = document.querySelector(".swipe-container");

container.addEventListener("touchend", handleSwipe);

function handleSwipe() {
    const minDist = 80;

    const swipeDist = container.scrollLeft - container.clientWidth;

    if (swipeDist < minDist * -1) {
        console.log("swiped right", `${swipeDist}px`);
    } else if (swipeDist > minDist) {
        console.log("swiped left", `${swipeDist}px`);
    } else {
        console.log("not enough", `${swipeDist}px`);
    }
}
