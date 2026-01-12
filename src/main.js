const audio = document.getElementById("bgm");
let played = false;

const image = document.getElementById("hidden-image");
image.addEventListener("click", () => {
    image.style.display = "none";
    alert("뭘 봐? 이 바보야!");
})

document.addEventListener("click", () => {
if (!played) {
    audio.play();
    played = true;
}
});