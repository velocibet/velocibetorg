const audio = document.querySelector("#bgm");
const image = document.querySelector("#hidden-image");
const visitorCountElement = document.querySelector("#visitor-count");

const navBtn1 = document.querySelector("#nav-btn-1");
const navBtn2 = document.querySelector("#nav-btn-2");
const navBtn3 = document.querySelector("#nav-btn-3");

const content1 = document.querySelector(".content-container-01");
const content2 = document.querySelector(".content-container-02");
const content3 = document.querySelector(".content-container-03");

const imageElements = document.querySelectorAll("#imageElement");

let played = false;

const API_URL = import.meta.env.VITE_API_URL;

async function getVisitorCount() {
    try {
        const response = await fetch(`${API_URL}/visitor`, {
            method: "POST",
            headers: { 'Content-Type': 'application/json' },
            mode: 'cors'
        });
        const data = await response.json();
        updateVisitorCount(data.count);
    } catch (error) {
        console.error("방문자 수를 불러오는데에 오류가 발생했습니다. 오류의 내용은 다음과 같습니다: ", error);
    }
}

function handleImageClick() {
    image.style.display = "none";
    alert("뭘 봐? 이 바보야!");
}

function handleAudioPlay() {
    if (!played) {
        audio.play();
        played = true;
    }
}

function updateVisitorCount(count) {
    if (!visitorCountElement) return;
    visitorCountElement.innerHTML = String(count);
}

function navigationTo(page) {
    content1.style.display = "none";
    content2.style.display = "none";
    content3.style.display = "none";

    if (page === 1) {
        content1.style.display = "flex";
    } else if (page === 2) {
        content2.style.display = "flex";
    } else if (page === 3) {
        content3.style.display = "flex";
    }
}

imageElements.forEach(ele => {
    ele.addEventListener("click", () => {
        window.open(ele.src, "_blank");
    })
})

image.addEventListener("click", handleImageClick);
document.addEventListener("click", handleAudioPlay);
navBtn1.addEventListener("click", () => navigationTo(1));
navBtn2.addEventListener("click", () => navigationTo(2));
navBtn3.addEventListener("click", () => navigationTo(3));

await getVisitorCount();