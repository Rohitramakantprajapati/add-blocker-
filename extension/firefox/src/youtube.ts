function clickSkipButtons(): void {
  const selectors = [".ytp-ad-skip-button", ".ytp-skip-ad-button"];
  for (const selector of selectors) {
    const button = document.querySelector<HTMLButtonElement>(selector);
    if (button) {
      button.click();
      return;
    }
  }
}

function observeYouTube(): void {
  try {
    const observer = new MutationObserver(() => {
      try {
        clickSkipButtons();
      } catch (error) {
        console.error("VoidBlock Firefox YouTube observer failed", error);
      }
    });

    observer.observe(document.body, { childList: true, subtree: true });
    clickSkipButtons();
  } catch (error) {
    console.error("VoidBlock Firefox YouTube setup failed", error);
  }
}

if (document.body) {
  observeYouTube();
} else {
  window.addEventListener("DOMContentLoaded", observeYouTube, { once: true });
}
