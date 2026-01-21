// static/portfolio.js

const chatWindow = document.getElementById('chat-window');
const toggleBtn = document.getElementById('chat-toggle-btn');
const closeBtn = document.getElementById('close-chat');
const chatForm = document.getElementById('chat-form');
const messagesContainer = document.getElementById('chat-messages');
const userInput = document.getElementById('user-input');

// Open/Close logic
toggleBtn.addEventListener('click', () => chatWindow.classList.remove('hidden'));
closeBtn.addEventListener('click', () => chatWindow.classList.add('hidden'));

// Send logic
chatForm.addEventListener('submit', async (e) => {
    e.preventDefault();
    const question = userInput.value.trim();
    if (!question) return;

    addMessage(question, 'user');
    userInput.value = '';
    
    // Add loading spinner
    const loadingId = addMessage("Thinking...", 'bot');

    try {
        const res = await fetch('/chat', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ question })
        });
        
        const answer = await res.text();
        const loadingEl = document.getElementById(loadingId);
        
        if (loadingEl) {
            // Check for API errors in the text
            if (answer.includes("Gemini API Error")) {
                loadingEl.style.color = "#ff6b6b";
            }
            loadingEl.innerText = answer;
        }
    } catch (err) {
        console.error(err);
        const loadingEl = document.getElementById(loadingId);
        if (loadingEl) loadingEl.innerText = "Error: Brain is offline.";
    }
});

function addMessage(text, sender) {
    const div = document.createElement('div');
    div.classList.add('message', sender);
    div.innerText = text;
    div.id = 'msg-' + Math.random().toString(36).substr(2, 9);
    messagesContainer.appendChild(div);
    messagesContainer.scrollTop = messagesContainer.scrollHeight;
    return div.id;
}