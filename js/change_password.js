document.addEventListener('DOMContentLoaded', () => {
    const main = document.querySelector('main');
    if (main && main.dataset.error === 'true') {
        const errorDiv = document.getElementById('change-password-error');
        if (errorDiv) errorDiv.style.display = 'block';
        
        const passwordInput = document.getElementById('new-password');
        if (passwordInput) passwordInput.classList.add('input-error');
    }
});
