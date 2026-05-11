const blockedSelectors = [
  ".ad",
  ".ads",
  ".advertisement",
  "[data-ad]",
  "[aria-label*='advert']",
];

function injectStyle(): void {
  try {
    const style = document.createElement("style");
    style.textContent = `${blockedSelectors.join(",")} { display: none !important; visibility: hidden !important; }`;
    document.documentElement.appendChild(style);
  } catch (error) {
    console.error("VoidBlock style injection failed", error);
  }
}

function observeDom(): void {
  if (!document.body) {
    return;
  }

  const observer = new MutationObserver(() => {
    try {
      for (const selector of blockedSelectors) {
        document.querySelectorAll(selector).forEach((element) => {
          const htmlElement = element as HTMLElement;
          htmlElement.style.setProperty("display", "none", "important");
        });
      }
    } catch (error) {
      console.error("VoidBlock DOM observer failed", error);
    }
  });

  observer.observe(document.body, { childList: true, subtree: true });
}

injectStyle();
observeDom();
