function clickSkipButton(): void {
  const selectors = [".ytp-ad-skip-button", ".ytp-skip-ad-button"];
  for (const selector of selectors) {
    const button = document.querySelector<HTMLButtonElement>(selector);
    if (button) {
      button.click();
      return;
    }
  }
}

function startYouTubeObserver(): void {
  try {
    const observer = new MutationObserver(() => {
      try {
        clickSkipButton();
      } catch (error) {
        console.error("VoidBlock YouTube skip failed", error);
      }
    });

    observer.observe(document.body, { childList: true, subtree: true });
    clickSkipButton();
  } catch (error) {
    console.error("VoidBlock YouTube observer failed", error);
  }
}

if (document.body) {
  startYouTubeObserver();
} else {
  window.addEventListener("DOMContentLoaded", startYouTubeObserver, { once: true });
}
