document.addEventListener('DOMContentLoaded', function () {
    const btnEdit = document.getElementById('btn-edit');
    const btnSave = document.getElementById('btn-save');
    const authorDisplay = document.getElementById('author-name-display');
    const authorInput = document.getElementById('author-name-input');

    if (btnEdit && btnSave && authorDisplay && authorInput) {
        btnEdit.addEventListener('click', function () {
            authorDisplay.style.display = 'none';
            authorInput.style.display = 'inline-block';
            btnEdit.style.display = 'none';
            btnSave.style.display = 'inline-block';
            authorInput.focus();
        });
    }
});
