function writeToClipboard(text) {
  return navigator.clipboard.writeText(text);
}

function copyToClipboard(elementId) {
  let textContent = document.querySelector(`#${elementId}`).textContent.trim();

  writeToClipboard(textContent)
    .then(function () {
      displayCopiedMessage(elementId);
    })
    .catch(function (error) {
      console.error("Could not copy text:", error);
    });
}

function displayCopiedMessage(elementId) {
  let copiedMessage = document.querySelector(`#${elementId}` + `-message`);
  copiedMessage.style.display = "inline";
  setTimeout(function () {
    copiedMessage.style.display = "none";
  }, 2000);
}
