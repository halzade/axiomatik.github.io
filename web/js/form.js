const form = document.getElementById('article-form');
form.setAttribute('novalidate', true);

form.addEventListener('submit', async function(e) {
    e.preventDefault();
    const form = e.target;
    let isValid = true;

    // Reset previous errors
    form.querySelectorAll('.error').forEach(el => el.classList.remove('error'));
    form.querySelectorAll('.error-message').forEach(el => el.style.display = 'none');

    // Basic required validation
    form.querySelectorAll('.required').forEach(input => {
        if (!input.value.trim()) {
            showError(input);
            isValid = false;
        }
    });

    // Image validation and scaling
    const fileInput = document.getElementById('image-input');
    let finalImageData = null;

    if (fileInput.files && fileInput.files[0]) {
        const file = fileInput.files[0];
        if (file.type.startsWith('image/')) {
            try {
                const img = await loadImage(file);
                if (img.width < 820) {
                    showError(fileInput, `Obrázek musí mít šířku alespoň 820 px. Aktuální šířka je: ${img.width} px.`);
                    isValid = false;
                } else {
                    // It is exactly 820px, we don't need to scale, but we'll use the original file
                    finalImageData = file;
                }
            } catch (err) {
                showError(fileInput, "Nepodařilo se načíst obrázek.");
                isValid = false;
            }
        }
    }

    if (!isValid) {
        const firstError = form.querySelector('.error');
        if (firstError) {
            firstError.scrollIntoView({ behavior: 'smooth', block: 'center' });
        }
        return;
    }

    // Submit the form
    const formData = new FormData(form);
    if (finalImageData) {
        // Replace the original image with the scaled one or the original if it was 820px
        formData.set('image', finalImageData, fileInput.files[0].name);
    }

    try {
        await fetch(form.action, {
            method: 'POST',
            body: formData
        });
    } catch (err) {
        console.error(err);
        alert("Při odesílání formuláře došlo k chybě.");
    }
});

function showError(input, customMessage) {
    input.classList.add('error');
    // Find error message in the same container or next sibling
    let errorDiv = input.parentElement.querySelector('.error-message');
    if (!errorDiv && input.nextElementSibling && input.nextElementSibling.classList.contains('error-message')) {
        errorDiv = input.nextElementSibling;
    }
    
    if (errorDiv) {
        if (customMessage) errorDiv.textContent = customMessage;
        errorDiv.style.display = 'block';
    } else {
        console.warn('Error div not found for input:', input);
    }
}

function loadImage(file) {
    return new Promise((resolve, reject) => {
        const reader = new FileReader();
        reader.onload = e => {
            const img = new Image();
            img.onload = () => resolve(img);
            img.onerror = reject;
            img.src = e.target.result;
        };
        reader.onerror = reject;
        reader.readAsDataURL(file);
    });
}
