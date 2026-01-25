document.addEventListener('DOMContentLoaded', () => {
    const main = document.querySelector('main');
    if (!main) return;

    if (main.dataset.error === 'true') {
        const errorDiv = document.getElementById('create-user-error');
        if (errorDiv) errorDiv.style.display = 'block';
        
        const usernameInput = document.getElementById('username');
        if (usernameInput) usernameInput.classList.add('input-error');
        
        const passwordInput = document.getElementById('password');
        if (passwordInput) passwordInput.classList.add('input-error');
    }

    if (main.dataset.success === 'true') {
        const successDiv = document.getElementById('create-user-success');
        if (successDiv) successDiv.style.display = 'block';
    }
});
