document.getElementById('article-form').addEventListener('submit', function(e) {
    const fileInput = document.getElementById('image-input');
    const errorDiv = document.getElementById('image-error');
    
    if (fileInput.files && fileInput.files[0]) {
        const file = fileInput.files[0];
        // Only validate images
        if (file.type.startsWith('image/')) {
            e.preventDefault(); // Stop form submission to check image
            
            const reader = new FileReader();
            reader.onload = function(event) {
                const img = new Image();
                img.onload = function() {
                    if (this.width === 820) {
                        errorDiv.style.display = 'none';
                        // Use a flag or remove listener to avoid infinite loop on submit()
                        const form = document.getElementById('article-form');
                        // Create a temporary submit that doesn't trigger the event listener
                        HTMLFormElement.prototype.submit.call(form);
                    } else {
                        errorDiv.style.display = 'block';
                        errorDiv.textContent = 'Obrázek musí mít šířku přesně 820 px. Aktuální šířka: ' + this.width + ' px.';
                        fileInput.scrollIntoView({ behavior: 'smooth', block: 'center' });
                    }
                };
                img.src = event.target.result;
            };
            reader.readAsDataURL(file);
        }
    }
});
