document.addEventListener('DOMContentLoaded', () => {
    const main = document.querySelector('main');
    if (main && main.dataset.error === 'true') {
        const errorDiv = document.getElementById('login-error');
        if (errorDiv) errorDiv.style.display = 'block';
        
        const usernameInput = document.getElementById('username');
        if (usernameInput) usernameInput.classList.add('input-error');
        
        const passwordInput = document.getElementById('password');
        if (passwordInput) passwordInput.classList.add('input-error');
    }
});
