const video = document.querySelector('video');
const container = video.parentElement;

let ended = false;
container.addEventListener('mouseenter', () => {
    if (!ended) {
        video.play();
    }
});

container.addEventListener('mouseleave', () => {
    video.pause();
});

video.addEventListener('ended', () => {
    video.pause();
    ended = true;
});
