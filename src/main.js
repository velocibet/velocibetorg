const audio = document.querySelector("#bgm");
const image = document.querySelector("#hidden-image");
const visitorCountElement = document.querySelector("#visitor-count");
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

image.addEventListener("click", handleImageClick);
document.addEventListener("click", handleAudioPlay);

await getVisitorCount();