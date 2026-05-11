const selectors = [".ad", ".ads", ".advertisement", "[data-ad]", "[aria-label*='advert']"];

function applyCosmetics(): void {
  try {
    const style = document.createElement("style");
    style.textContent = `${selectors.join(",")} { display: none !important; }`;
    document.documentElement.appendChild(style);
  } catch (error) {
    console.error("VoidBlock Firefox cosmetics failed", error);
  }
}

function observeDom(): void {
  if (!document.body) {
    return;
  }

  const observer = new MutationObserver(() => {
    try {
      for (const selector of selectors) {
        document.querySelectorAll(selector).forEach((node) => {
          (node as HTMLElement).style.setProperty("display", "none", "important");
        });
      }
    } catch (error) {
      console.error("VoidBlock Firefox observer failed", error);
    }
  });

  observer.observe(document.body, { childList: true, subtree: true });
}

applyCosmetics();
observeDom();
