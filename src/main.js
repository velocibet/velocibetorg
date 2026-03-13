const audio = document.querySelector("#bgm");
const image = document.querySelector("#hidden-image");
const visitorCountElement = document.querySelector("#visitor-count");

const navBtn1 = document.querySelector("#nav-btn-1");
const navBtn2 = document.querySelector("#nav-btn-2");
const navBtn3 = document.querySelector("#nav-btn-3");

const content1 = document.querySelector(".content-container-01");
const content2 = document.querySelector(".content-container-02");

const imageElements = document.querySelectorAll("#imageElement");

const form = document.querySelector('#guestbook-form');
const guestbookEntriesContainer = document.querySelector('.guestbook-chat');

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

    if (page === 1) {
        content1.style.display = "flex";
    } else if (page === 2) {
        content2.style.display = "flex";
    }
}

imageElements.forEach(ele => {
    ele.addEventListener("click", () => {
        window.open(ele.src, "_blank");
    })
})

async function fetchGuestbook() {
    if (!guestbookEntriesContainer) return;
    try {
        const response = await fetch(`${API_URL}/guestbook`, {
            method: "GET",
            headers: { 'Content-Type': 'application/json' },
            mode: 'cors'
        });
        const entries = await response.json();
        
        guestbookEntriesContainer.innerHTML = '';
        await entries.reverse().forEach(entry => {
            const div = document.createElement('div');
            div.className = 'guestbook-entry';
            div.innerHTML = `<strong>${entry.username}</strong>: ${entry.content}`;
            guestbookEntriesContainer.appendChild(div);
        });

        guestbookEntriesContainer.scrollTop = guestbookEntriesContainer.scrollHeight;
    } catch (error) {
        console.error("방명록 불러오기 오류:", error);
    }
}

form.addEventListener('submit', async function(event) {
    event.preventDefault();

    const username = form.username.value.trim();
    const content = form.content.value.trim();

    if (!username || !content) {
        alert("이름과 내용을 모두 입력해주세요.");
        return;
    }

    try {
        await fetch(`${API_URL}/guestbook/submit`, {
            method: "POST",
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ username, content }),
        });

        form.reset();
        await fetchGuestbook();

        alert("메세지를 보내는데 성공했습니다.");
    } catch (error) {
        alert("메시지를 보내는데 실패했습니다.");
        console.error("메시지 전송 오류:", error);
    }
});

image.addEventListener("click", handleImageClick);
document.addEventListener("click", handleAudioPlay);
navBtn1.addEventListener("click", () => navigationTo(1));
navBtn2.addEventListener("click", () => navigationTo(2));
navBtn3.addEventListener("click", () => {
    window.open("https://blog.velocibet.com/", "_blank");
});

await getVisitorCount();
await fetchGuestbook();