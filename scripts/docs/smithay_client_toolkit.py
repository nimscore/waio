#!/usr/bin/env python3
"""
Скрипт для скачивания документации smithay_client_toolkit в единый Markdown-файл.
Исправленная версия с авто-определением версии и гибкой обработкой URL.
"""

import re
import sys
import time
from pathlib import Path
from urllib.parse import urljoin, urlparse, urlunparse

import requests
from bs4 import BeautifulSoup

# ===== НАСТРОЙКИ =====
# Используйте "latest" для автоматического получения последней версии
# Или укажите конкретную: "0.20.0", "0.19.0" и т.д.
VERSION = "latest"  # ← Измените при необходимости

CRATE_NAME = "smithay-client-toolkit"
MODULE_NAME = "smithay_client_toolkit"
BASE_URL = f"https://docs.rs/{CRATE_NAME}/{VERSION}/{MODULE_NAME}/"

# Ссылки на элементы (относительные пути)
DOC_LINKS = [
    # ===== MODULES =====
    "activation/",
    "compositor/",
    "data_device_manager/",
    "dmabuf/",
    "error/",
    "foreign_toplevel_list/",
    "globals/",
    "output/",
    "presentation_time/",
    "primary_selection/",
    "reexports/",
    "registry/",
    "seat/",
    "session_lock/",
    "shell/",
    "shm/",
    "subcompositor/",
    # ===== MACROS =====
    "macro.delegate_activation.html",
    "macro.delegate_compositor.html",
    "macro.delegate_data_device.html",
    "macro.delegate_dmabuf.html",
    "macro.delegate_foreign_toplevel_list.html",
    "macro.delegate_input_method.html",
    "macro.delegate_input_method_v3.html",
    "macro.delegate_keyboard.html",
    "macro.delegate_layer.html",
    "macro.delegate_output.html",
    "macro.delegate_pointer.html",
    "macro.delegate_pointer_constraints.html",
    "macro.delegate_presentation_time.html",
    "macro.delegate_primary_selection.html",
    "macro.delegate_registry.html",
    "macro.delegate_relative_pointer.html",
    "macro.delegate_seat.html",
    "macro.delegate_session_lock.html",
    "macro.delegate_shm.html",
    "macro.delegate_simple.html",
    "macro.delegate_subcompositor.html",
    "macro.delegate_touch.html",
    "macro.delegate_xdg_popup.html",
    "macro.delegate_xdg_shell.html",
    "macro.delegate_xdg_window.html",
    "macro.registry_handlers.html",
]

HEADERS = {
    "User-Agent": "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36",
    "Accept": "text/html,application/xhtml+xml;q=0.9,*/*;q=0.8",
}


def get_actual_version() -> str:
    """Определяет актуальную версию пакета, если указано 'latest'."""
    if VERSION != "latest":
        return VERSION

    try:
        # Запрашиваем страницу crates.io для получения последней версии
        resp = requests.get(
            f"https://crates.io/api/v1/crates/{CRATE_NAME}", headers=HEADERS, timeout=15
        )
        if resp.status_code == 200:
            data = resp.json()
            latest = data["crate"]["newest_version"]
            print(f"🔍 Определена актуальная версия: {latest}")
            return latest
    except Exception as e:
        print(f"⚠️ Не удалось определить версию автоматически: {e}")
        print("   Используем 'latest' в URL")
    return "latest"


def normalize_url(path: str) -> list[str]:
    """
    Генерирует варианты URL для одной страницы.
    Пробует разные форматы: с / без index.html, с / без конечного слеша.
    """
    variants = []

    # Если это модуль (заканчивается на /)
    if path.endswith("/"):
        variants.append(path)  # как есть: activation/
        variants.append(path + "index.html")  # activation/index.html
    # Если это файл .html
    elif path.endswith(".html"):
        variants.append(path)
        # Для макросов иногда путь может быть без macro. префикса в некоторых версиях
        if path.startswith("macro."):
            variants.append(path.replace("macro.", ""))
    else:
        variants.append(path)
        variants.append(path + ".html")
        variants.append(path + "/index.html")

    return variants


def fetch_page(url: str) -> str | None:
    """Скачивает страницу с обработкой ошибок."""
    try:
        response = requests.get(url, headers=HEADERS, timeout=20)
        if response.status_code == 404:
            return None
        response.raise_for_status()
        return response.text
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
    """Конвертация HTML → Markdown (упрощённая, но рабочая)."""

    def process(node, indent=0):
        if isinstance(node, str):
            txt = node.strip()
            return (" " * indent + re.sub(r"\s+", " ", txt) + "\n") if txt else ""

        if not hasattr(node, "name") or not node.name:
            return ""

        tag = node.name

        # Заголовки
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

        # Параграфы
        if tag == "p":
            text = node.get_text(strip=True)
            return (" " * indent + text + "\n\n") if text else ""

        # Списки
        if tag in ["ul", "ol"]:
            result = "\n"
            for i, li in enumerate(node.find_all("li", recursive=False), 1):
                txt = li.get_text(strip=True)
                prefix = f"{i}. " if tag == "ol" else "- "
                result += " " * indent + prefix + txt + "\n"
            return result + "\n"

        # Описание (dl/dt/dd)
        if tag == "dl":
            result = "\n"
            for dt in node.find_all("dt"):
                term = dt.get_text(strip=True)
                dd = dt.find_next_sibling("dd")
                desc = dd.get_text(strip=True) if dd else ""
                result += f"- **`{term}`**" + (f": {desc}\n" if desc else "\n")
            return result + "\n"

        # Код
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
                    "",
                )
                return f"\n```{lang}\n{code.get_text().rstrip()}\n```\n\n"
            return f"\n```\n{node.get_text().rstrip()}\n```\n\n"

        if tag == "code":
            if node.parent and node.parent.name == "pre":
                return ""
            return f"`{node.get_text(strip=True)}`"

        # Ссылки
        if tag == "a":
            text = node.get_text(strip=True)
            href = node.get("href", "")
            if not text:
                return ""
            if href and not href.startswith("#"):
                full = urljoin(BASE_URL, href) if not href.startswith("http") else href
                return f"[{text}]({full})"
            return text

        # Форматирование
        if tag == "strong":
            return f"**{node.get_text(strip=True)}**"
        if tag == "em":
            return f"*{node.get_text(strip=True)}*"

        # Бейджи (feature-флаги)
        if (
            tag == "span"
            and node.get("class")
            and any("stab" in str(c) for c in node.get("class", []))
        ):
            return f" > *{node.get_text(strip=True)}*\n\n"

        # Рекурсия
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
    """Текст → slug для якорей."""
    text = text.lower().strip()
    text = re.sub(r"[^\w\s-]", "", text)
    return re.sub(r"[\s_-]+", "-", text)


def main():
    actual_version = get_actual_version()
    base = f"https://docs.rs/{CRATE_NAME}/{actual_version}/{MODULE_NAME}/"

    output = f"{MODULE_NAME}_doc.md"
    print(f"📥 Скачивание: {CRATE_NAME} v{actual_version}")
    print(f"📄 Вывод: {output}\n")

    success = 0
    with open(output, "w", encoding="utf-8") as f:
        f.write(f"# Документация {MODULE_NAME} v{actual_version}\n\n")
        f.write(f"> Источник: [{base}]({base})\n")
        f.write(f"> Сгенерировано: {time.strftime('%Y-%m-%d %H:%M:%S')}\n\n---\n\n")

        # Оглавление
        f.write("## 📑 Оглавление\n### Модули\n")
        for link in [l for l in DOC_LINKS if l.endswith("/")]:
            name = link.rstrip("/")
            f.write(f"- [`{name}`](#{slugify(name)})\n")
        f.write("\n### Макросы\n")
        for link in [l for l in DOC_LINKS if l.startswith("macro.")]:
            name = link.replace("macro.", "").replace(".html", "")
            f.write(f"- [`{name}`](#{slugify(name)})\n")
        f.write("\n---\n")

        # Обработка ссылок
        for i, rel_path in enumerate(DOC_LINKS, 1):
            item_name = rel_path.rstrip("/").replace("macro.", "").replace(".html", "")
            print(f"[{i:2d}/{len(DOC_LINKS)}] {item_name}", end=" ... ")

            # Пробуем разные варианты URL
            html = None
            for variant in normalize_url(rel_path):
                url = urljoin(base, variant)
                html = fetch_page(url)
                if html:
                    print(f"✓ ({variant})")
                    break
            else:
                print("✗ 404")
                f.write(f"\n### ⚠️ Не найдено: `{rel_path}`\n\n")
                continue

            title = get_title(html, item_name)
            anchor = slugify(title)

            f.write(f'<a id="{anchor}"></a>\n')
            f.write(f"\n## `{title}`\n---\n")
            f.write(extract_doc_content(html, title))
            f.write("\n\n[🔝 Наверх](#-оглавление)\n")
            success += 1

            time.sleep(0.2)  # Вежливость

        # Итог
        f.write(f"\n---\n> ✅ {success}/{len(DOC_LINKS)} страниц скачано успешно\n")

    print(f"\n✅ Готово: `{output}` ({success}/{len(DOC_LINKS)})")


if __name__ == "__main__":
    try:
        import requests
        from bs4 import BeautifulSoup
    except ImportError as e:
        print(f"❌ Установите зависимости: pip install requests beautifulsoup4")
        sys.exit(1)
    main()
