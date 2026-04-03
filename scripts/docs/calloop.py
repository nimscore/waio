#!/usr/bin/env python3
"""
Скрипт для скачивания документации calloop в единый Markdown-файл.
"""

import re
import sys
import time
from urllib.parse import urljoin, urlparse

import requests
from bs4 import BeautifulSoup

# ===== НАСТРОЙКИ =====
VERSION = "latest"  # или конкретная версия: "0.10.0"
CRATE_NAME = "calloop"
MODULE_NAME = "calloop"
BASE_URL = f"https://docs.rs/{CRATE_NAME}/{VERSION}/{MODULE_NAME}/"

# Все ссылки на элементы документации (относительные пути)
DOC_LINKS = [
    # ===== RE-EXPORTS (из error) =====
    "error/enum.Error.html",
    "error/struct.InsertError.html",
    "error/type.Result.html",
    # ===== MODULES =====
    "channel/index.html",
    "error/index.html",
    "futures/index.html",  # feature: executor
    "generic/index.html",
    "io/index.html",
    "ping/index.html",
    "signals/index.html",  # Linux only
    "stream/index.html",  # feature: stream
    "timer/index.html",
    "transient/index.html",
    # ===== MACROS =====
    "macro.batch_register.html",
    "macro.batch_reregister.html",
    "macro.batch_unregister.html",
    # ===== STRUCTS =====
    "struct.Dispatcher.html",
    "struct.EventIterator.html",
    "struct.EventLoop.html",
    "struct.Idle.html",
    "struct.Interest.html",
    "struct.LoopHandle.html",
    "struct.LoopSignal.html",
    "struct.Poll.html",
    "struct.Readiness.html",
    "struct.RegistrationToken.html",
    "struct.Token.html",
    "struct.TokenFactory.html",
    # ===== ENUMS =====
    "enum.Mode.html",
    "enum.PostAction.html",
    # ===== TRAITS =====
    "trait.EventSource.html",
]

HEADERS = {
    "User-Agent": "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36",
    "Accept": "text/html,application/xhtml+xml;q=0.9,*/*;q=0.8",
}


def get_actual_version() -> str:
    """Определяет актуальную версию через crates.io API."""
    if VERSION != "latest":
        return VERSION
    try:
        resp = requests.get(
            f"https://crates.io/api/v1/crates/{CRATE_NAME}", headers=HEADERS, timeout=15
        )
        if resp.status_code == 200:
            return resp.json()["crate"]["newest_version"]
    except Exception as e:
        print(f"⚠️ Не удалось определить версию: {e}", file=sys.stderr)
    return "latest"


def normalize_url(path: str) -> list[str]:
    """Генерирует варианты URL для одной страницы."""
    variants = []
    if path.endswith("/"):
        variants.extend([path, path + "index.html"])
    elif path.endswith(".html"):
        variants.append(path)
        # Для macro.* пробуем также без префикса (в старых версиях)
        if path.startswith("macro."):
            variants.append(path[6:])  # убираем "macro."
        # Для error/* пробуем прямой путь
        if path.startswith("error/"):
            variants.append(path.replace("error/", ""))
    else:
        variants.extend([path, path + ".html", path + "/index.html"])
    return variants


def fetch_page(url: str) -> str | None:
    """Скачивает страницу с обработкой ошибок."""
    try:
        resp = requests.get(url, headers=HEADERS, timeout=20)
        if resp.status_code == 404:
            return None
        resp.raise_for_status()
        return resp.text
    except requests.RequestException as e:
        print(f"⚠️ Ошибка {url}: {e}", file=sys.stderr)
        return None


def extract_doc_content(html: str, page_title: str) -> str:
    """Извлекает основное содержание из HTML rustdoc."""
    soup = BeautifulSoup(html, "html.parser")

    # Удаляем навигацию и служебные элементы
    for selector in [
        "script",
        "style",
        "nav",
        "footer",
        "rustdoc-toolbar",
        ".sidebar",
        "#sidebar",
        "#copy-path",
        ".src",
        ".sub-heading",
        "#settings-menu",
        "#help-button",
        ".main-heading",
        ".nav-container",
        "#search-button",
        "#sidebar-button",
        ".toggle",
        "rustdoc-toolbar",
    ]:
        for tag in soup.select(selector):
            tag.decompose()

    # Ищем контент
    main = (
        soup.find("section", id="main-content")
        or soup.find("div", class_="docblock")
        or soup.body
    )
    if not main:
        return f"⚠️ Не удалось извлечь контент для: {page_title}\n"

    return html_to_markdown(main, page_title)


def html_to_markdown(element, title: str) -> str:
    """Конвертация HTML → Markdown."""

    def process(node, indent=0):
        if isinstance(node, str):
            txt = node.strip()
            return (" " * indent + re.sub(r"\s+", " ", txt) + "\n") if txt else ""

        if not hasattr(node, "name") or not node.name:
            return ""

        tag = node.name

        # === Заголовки ===
        if tag in ["h1", "h2", "h3", "h4", "h5", "h6"]:
            for btn in node.find_all(
                ["a", "button"], class_=lambda c: c and "anchor" in str(c)
            ):
                btn.decompose()
            text = node.get_text(strip=True)
            if not text:
                return ""
            level = min(int(tag[1]), 6)
            return f"\n{'#' * level} {text}\n\n"

        # === Параграфы ===
        if tag == "p":
            text = node.get_text(strip=True)
            return (" " * indent + text + "\n\n") if text else ""

        # === Списки ===
        if tag in ["ul", "ol"]:
            result = "\n"
            for i, li in enumerate(node.find_all("li", recursive=False), 1):
                txt = li.get_text(strip=True)
                prefix = f"{i}. " if tag == "ol" else "- "
                result += " " * indent + prefix + txt + "\n"
            return result + "\n"

        # === Описание списков (dl/dt/dd) ===
        if tag == "dl":
            result = "\n"
            for dt in node.find_all("dt"):
                # Очищаем dt от кода и ссылок для чистого текста
                term = dt.get_text(strip=True)
                dd = dt.find_next_sibling("dd")
                desc = dd.get_text(strip=True) if dd else ""
                if term:
                    result += f"- **`{term}`**" + (f": {desc}\n" if desc else "\n")
            return result + "\n"

        # === Re-exports (специальная обработка) ===
        if tag == "dl" and node.get("class") and "reexports" in str(node.get("class")):
            result = "\n| Экспорт | Описание |\n|---------|----------|\n"
            for dt in node.find_all("dt", id=lambda x: x and x.startswith("reexport.")):
                name = dt.get("id", "").replace("reexport.", "")
                # Извлекаем тип и путь
                code = dt.find("code")
                if code:
                    result += f"| `{name}` | `{code.get_text(strip=True)}` |\n"
            return result + "\n"

        # === Код ===
        if tag == "pre":
            code = node.find("code")
            if code:
                cls = code.get("class", [])
                lang = next(
                    (
                        c.replace("language-", "")
                        for c in cls
                        if isinstance(c, str) and c.startswith("language-")
                    ),
                    "rust",
                )
                return f"\n```{lang}\n{code.get_text().rstrip()}\n```\n\n"
            return f"\n```\n{node.get_text().rstrip()}\n```\n\n"

        if tag == "code":
            if node.parent and node.parent.name == "pre":
                return ""
            return f"`{node.get_text(strip=True)}`"

        # === Ссылки ===
        if tag == "a":
            text = node.get_text(strip=True)
            href = node.get("href", "")
            if not text:
                return ""
            if href and not href.startswith("#"):
                full = urljoin(BASE_URL, href) if not href.startswith("http") else href
                return f"[{text}]({full})"
            return text

        # === Форматирование ===
        if tag == "strong":
            return f"**{node.get_text(strip=True)}**"
        if tag == "em":
            return f"*{node.get_text(strip=True)}*"

        # === Бейджи (feature-флаги, платформы) ===
        if tag == "span" and node.get("class"):
            classes = " ".join(str(c) for c in node.get("class", []))
            if "stab" in classes or "portability" in classes:
                return f" > *{node.get_text(strip=True)}*\n\n"

        # === Рекурсивная обработка ===
        return "".join(process(child, indent) for child in node.children)

    content = "".join(process(child) for child in element.children)
    return re.sub(r"\n{3,}", "\n\n", content).strip()


def get_title(html: str, fallback: str) -> str:
    """Извлекает заголовок страницы."""
    soup = BeautifulSoup(html, "html.parser")
    h1 = soup.find("h1")
    if h1:
        for btn in h1.find_all(["button", "a", "span"]):
            if btn.get("class") and any(
                c in str(btn.get("class", "")) for c in ["src", "stab", "anchor"]
            ):
                btn.decompose()
        text = h1.get_text(strip=True)
        if text and "crate" not in text.lower():
            return re.sub(r"^crate\s+", "", text, flags=re.I)
    return fallback


def slugify(text: str) -> str:
    """Текст → slug для якорей Markdown."""
    text = text.lower().strip()
    text = re.sub(r"[^\w\s-]", "", text)
    return re.sub(r"[\s_-]+", "-", text)


def categorize_links(links: list[str]) -> dict[str, list[str]]:
    """Группирует ссылки по типам для оглавления."""
    cats = {
        "modules": [],
        "macros": [],
        "structs": [],
        "enums": [],
        "traits": [],
        "reexports": [],
    }
    for link in links:
        if "/index.html" in link or link.endswith("/"):
            cats["modules"].append(link)
        elif link.startswith("macro."):
            cats["macros"].append(link)
        elif link.startswith("struct."):
            cats["structs"].append(link)
        elif link.startswith("enum."):
            cats["enums"].append(link)
        elif link.startswith("trait."):
            cats["traits"].append(link)
        elif link.startswith("error/"):
            cats["reexports"].append(link)
    return cats


def main():
    actual_version = get_actual_version()
    base = f"https://docs.rs/{CRATE_NAME}/{actual_version}/{MODULE_NAME}/"
    output = f"{MODULE_NAME}_doc.md"

    print(f"📥 Скачивание: {CRATE_NAME} v{actual_version}")
    print(f"📄 Вывод: {output}\n")

    categories = categorize_links(DOC_LINKS)
    success = 0

    with open(output, "w", encoding="utf-8") as f:
        # Заголовок
        f.write(f"# Документация {MODULE_NAME} v{actual_version}\n\n")
        f.write(f"> Источник: [{base}]({base})\n")
        f.write(f"> Сгенерировано: {time.strftime('%Y-%m-%d %H:%M:%S')}\n")
        f.write(f"\n> Calloop — callback-based event loop для Rust.\n\n---\n\n")

        # Оглавление
        f.write("## 📑 Оглавление\n")
        for cat_name, cat_title in [
            ("modules", "Модули"),
            ("reexports", "Re-exports"),
            ("macros", "Макросы"),
            ("structs", "Структуры"),
            ("enums", "Перечисления"),
            ("traits", "Трейты"),
        ]:
            if categories[cat_name]:
                f.write(f"\n### {cat_title}\n")
                for link in categories[cat_name]:
                    name = (
                        link.rstrip("/")
                        .replace("index.html", "")
                        .replace("macro.", "")
                        .replace("struct.", "")
                        .replace("enum.", "")
                        .replace("trait.", "")
                        .replace(".html", "")
                    )
                    anchor = slugify(name)
                    f.write(f"- [`{name}`](#{anchor})\n")

        f.write("\n---\n")

        # Обработка ссылок
        for i, rel_path in enumerate(DOC_LINKS, 1):
            item_name = (
                rel_path.rstrip("/")
                .replace("index.html", "")
                .replace("macro.", "")
                .replace("struct.", "")
                .replace("enum.", "")
                .replace("trait.", "")
                .replace(".html", "")
            )
            print(f"[{i:2d}/{len(DOC_LINKS)}] {item_name}", end=" ... ")

            # Пробуем разные варианты URL
            html = None
            used_url = None
            for variant in normalize_url(rel_path):
                url = urljoin(base, variant)
                html = fetch_page(url)
                if html:
                    used_url = url
                    print(f"✓")
                    break
            else:
                print("✗")
                f.write(f"\n### ⚠️ Не найдено: `{rel_path}`\n\n")
                continue

            title = get_title(html, item_name)
            anchor = slugify(title)

            # Якорь для навигации
            f.write(f'<a id="{anchor}"></a>\n')
            f.write(f"\n## `{title}`\n")

            # Добавляем метаданные (feature-флаги, платформы)
            soup = BeautifulSoup(html, "html.parser")
            badges = []
            for span in soup.find_all(
                "span",
                class_=lambda c: c and ("stab" in str(c) or "portability" in str(c)),
            ):
                badge = span.get_text(strip=True)
                if badge:
                    badges.append(badge)
            if badges:
                f.write(f"*{'* | *'.join(badges)}*\n\n")

            f.write("---\n")
            f.write(extract_doc_content(html, title))
            f.write("\n\n[🔝 Наверх](#-оглавление)\n")
            success += 1

            time.sleep(0.15)  # Вежливость

        # Итог
        f.write(f"\n---\n")
        f.write(f"> ✅ Скачано: **{success}/{len(DOC_LINKS)}** страниц\n")
        if success < len(DOC_LINKS):
            f.write(
                f"> ❌ Пропущено: **{len(DOC_LINKS) - success}** страниц (404 или недоступно)\n"
            )

    print(f"\n✅ Готово: `{output}` ({success}/{len(DOC_LINKS)})")


if __name__ == "__main__":
    try:
        import requests
        from bs4 import BeautifulSoup
    except ImportError as e:
        print(f"❌ Установите зависимости: pip install requests beautifulsoup4")
        sys.exit(1)
    main()
