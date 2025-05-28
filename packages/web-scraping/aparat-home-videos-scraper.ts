import puppeteer from "puppeteer";

async function aparatHomeVideosScraper() {
  const browser = await puppeteer.launch({
    headless: true,
    args: ["--no-sandbox", "--disable-setuid-sandbox"],
  });

  try {
    const page = await browser.newPage();
    await page.setUserAgent(
      "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/114.0.0.0 Safari/537.36",
    );

    await page.goto("https://www.aparat.com/", {
      waitUntil: "networkidle0",
      timeout: 600000,
    });

    const videos = await page.evaluate(() => {
      const cards = document.querySelectorAll(".grid-item");
      const items: { title: string; link: string }[] = [];

      cards.forEach((card) => {
        const titleEl = card.querySelector(".title .label-text");
        const linkEl = card.querySelector(".link");

        const title = titleEl?.textContent?.trim() || "";
        const link = linkEl?.getAttribute("href") || "";

        if (title && link) {
          items.push({
            title,
            link: link.startsWith("http")
              ? link
              : `https://www.aparat.com${link}`,
          });
        }
      });

      return items;
    });

    return JSON.stringify(videos);
  } catch (error: any) {
    return JSON.stringify({
      error: "Failed to scrape Aparat",
      details: error.message,
    });
  } finally {
    await browser.close();
  }
}

(async () => {
  const videos = await aparatHomeVideosScraper();
  console.log(videos);
})();
