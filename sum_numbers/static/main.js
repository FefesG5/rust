function copyToClipboard(elementId) {
  let textArea = document.createElement("textarea");
  textArea.value = document.getElementById(elementId).textContent.trim();
  document.body.appendChild(textArea);
  textArea.select();
  document.execCommand("copy");
  document.body.removeChild(textArea);

  let copiedMessage = document.getElementById(elementId + "-message");
  copiedMessage.style.display = "inline";
  setTimeout(function () {
    copiedMessage.style.display = "none";
  }, 2000);
}
