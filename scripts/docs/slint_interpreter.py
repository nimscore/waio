#!/usr/bin/env python3
"""
Скрипт для скачивания документации slint_interpreter в единый Markdown-файл.
"""

import requests
from bs4 import BeautifulSoup
import time
import re
import sys
from urllib.parse import urljoin, urlparse

# Базовый URL документации
BASE_URL = "https://docs.rs/slint-interpreter/1.15.1/slint_interpreter/"

# Все внутренние ссылки на элементы документации (относительные)
DOC_LINKS = [
    # Structs
    "struct.BackendSelector.html",
    "struct.Color.html",
    "struct.CompilationResult.html",
    "struct.Compiler.html",
    "struct.ComponentCompiler.html",
    "struct.ComponentDefinition.html",
    "struct.ComponentInstance.html",
    "struct.Diagnostic.html",
    "struct.Image.html",
    "struct.JoinHandle.html",
    "struct.LoadImageError.html",
    "struct.LogicalPosition.html",
    "struct.LogicalSize.html",
    "struct.OklchColor.html",
    "struct.PhysicalPosition.html",
    "struct.PhysicalSize.html",
    "struct.RgbaColor.html",
    "struct.SharedPixelBuffer.html",
    "struct.SharedString.html",
    "struct.SharedVector.html",
    "struct.Struct.html",
    "struct.Weak.html",
    "struct.Window.html",
    "struct.WindowHandle.html",

    # Enums
    "enum.Brush.html",
    "enum.CloseRequestResponse.html",
    "enum.DefaultTranslationContext.html",
    "enum.DiagnosticLevel.html",
    "enum.EventLoopError.html",
    "enum.GetPropertyError.html",
    "enum.GraphicsAPI.html",
    "enum.InvokeError.html",
    "enum.PlatformError.html",
    "enum.RenderingState.html",
    "enum.SetCallbackError.html",
    "enum.SetPropertyError.html",
    "enum.SetRenderingNotifierError.html",
    "enum.Value.html",
    "enum.ValueType.html",
    "enum.WindowPosition.html",
    "enum.WindowSize.html",

    # Traits
    "trait.ComponentHandle.html",
    "trait.Global.html",
    "trait.ToSharedString.html",

    # Functions
    "fn.invoke_from_event_loop.html",
    "fn.print_diagnostics.html",
    "fn.quit_event_loop.html",
    "fn.run_event_loop.html",
    "fn.set_xdg_app_id.html",
    "fn.spawn_local.html",

    # Macros & Modules
    "macro.format.html",
    "testing/index.html",

    # Type Aliases
    "type.Rgb8Pixel.html",
    "type.Rgba8Pixel.html",
]

# Заголовки для запросов (имитация браузера)
HEADERS = {
    "User-Agent": "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36",
    "Accept": "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8",
    "Accept-Language": "en-US,en;q=0.9,ru;q=0.8",
}

def fetch_page(url: str) -> str | None:
    """Скачивает содержимое страницы с обработкой ошибок."""
    try:
        response = requests.get(url, headers=HEADERS, timeout=30)
        response.raise_for_status()
        return response.text
    except requests.RequestException as e:
        print(f"⚠️ Ошибка загрузки {url}: {e}", file=sys.stderr)
        return None

def extract_doc_content(html: str, page_title: str) -> str:
    """Извлекает основное содержание документации из HTML rustdoc."""
    soup = BeautifulSoup(html, "html.parser")

    # Удаляем ненужные элементы
    for tag in soup(["script", "style", "nav", "footer", "rustdoc-toolbar",
                     "#sidebar", ".sidebar", "#copy-path", ".src"]):
        tag.decompose()

    # Ищем основной контент
    main_content = soup.find("section", id="main-content") or soup.find("div", class_="docblock") or soup.body

    if not main_content:
        return f"⚠️ Не удалось извлечь контент для: {page_title}\n"

    # Конвертируем в Markdown
    return html_to_markdown(main_content, page_title)

def html_to_markdown(element, title: str) -> str:
    """Преобразует HTML-элемент в Markdown-формат."""
    lines = []

    # Заголовок страницы
    lines.append(f"\n## {title}\n")
    lines.append("---\n")

    # Обрабатываем текстовые узлы и теги
    def process_node(node, indent=0):
        if isinstance(node, str):
            text = node.strip()
            if text:
                # Очищаем от лишних пробелов и переносов
                text = re.sub(r'\s+', ' ', text)
                return ' ' * indent + text + '\n'
            return ''

        tag_name = node.name if hasattr(node, 'name') else None

        if not tag_name:
            return ''

        # Заголовки
        if tag_name in ['h1', 'h2', 'h3', 'h4', 'h5', 'h6']:
            level = min(int(tag_name[1]), 6)
            text = node.get_text(strip=True)
            return f"\n{'#' * level} {text}\n\n"

        # Параграфы
        if tag_name == 'p':
            text = node.get_text(strip=True)
            if text:
                return ' ' * indent + text + '\n\n'
            return ''

        # Списки
        if tag_name == 'ul':
            result = '\n'
            for li in node.find_all('li', recursive=False):
                item_text = li.get_text(strip=True)
                result += ' ' * indent + f"- {item_text}\n"
            return result + '\n'

        if tag_name == 'ol':
            result = '\n'
            for i, li in enumerate(node.find_all('li', recursive=False), 1):
                item_text = li.get_text(strip=True)
                result += ' ' * indent + f"{i}. {item_text}\n"
            return result + '\n'

        # Код и преформатированный текст
        if tag_name == 'pre':
            code = node.find('code')
            if code:
                lang = code.get('class', [''])[0].replace('language-', '') if code.get('class') else ''
                code_text = code.get_text()
                return f"\n```{lang}\n{code_text.rstrip()}\n```\n\n"
            return f"\n```\n{node.get_text().rstrip()}\n```\n\n"

        if tag_name == 'code':
            # Инлайн-код
            parent = node.parent
            if parent and parent.name == 'pre':
                return ''  # Уже обработано выше
            return f"`{node.get_text(strip=True)}`"

        # Ссылки
        if tag_name == 'a':
            text = node.get_text(strip=True)
            href = node.get('href', '')
            if href and not href.startswith('#'):
                # Относительные ссылки делаем абсолютными для справки
                full_url = urljoin(BASE_URL, href) if not href.startswith('http') else href
                return f"[{text}]({full_url})"
            return text

        # Жирный и курсив
        if tag_name == 'strong' or (node.get('class') and 'strong' in node.get('class')):
            return f"**{node.get_text(strip=True)}**"
        if tag_name == 'em' or (node.get('class') and 'em' in node.get('class')):
            return f"*{node.get_text(strip=True)}*"

        # Таблицы (упрощённо)
        if tag_name == 'table':
            return process_table(node)

        # Рекурсивная обработка детей
        result = ''
        for child in node.children:
            result += process_node(child, indent)
        return result

    def process_table(table):
        """Обрабатывает HTML-таблицу в Markdown."""
        rows = []
        for tr in table.find_all('tr'):
            cells = [td.get_text(strip=True) for td in tr.find_all(['td', 'th'])]
            if cells:
                rows.append(cells)

        if not rows:
            return ''

        # Формируем Markdown-таблицу
        max_cols = max(len(row) for row in rows)
        md_lines = []

        # Заголовок
        header = rows[0] if len(rows) > 1 else ['Column'] * max_cols
        md_lines.append('| ' + ' | '.join(header) + ' |')
        md_lines.append('| ' + ' | '.join(['---'] * len(header)) + ' |')

        # Тело
        for row in rows[1:]:
            padded = row + [''] * (max_cols - len(row))
            md_lines.append('| ' + ' | '.join(padded) + ' |')

        return '\n' + '\n'.join(md_lines) + '\n\n'

    # Обрабатываем все дочерние элементы
    content = ''
    for child in element.children:
        content += process_node(child)

    # Очищаем от множественных переносов
    content = re.sub(r'\n{3,}', '\n\n', content)
    return content.strip()

def get_item_title(url: str, html: str) -> str:
    """Извлекает заголовок элемента из HTML или URL."""
    soup = BeautifulSoup(html, "html.parser")

    # Пробуем найти заголовок страницы
    title_tag = soup.find('h1')
    if title_tag:
        # Удаляем кнопки и лишние элементы из заголовка
        for btn in title_tag.find_all(['button', 'a', 'span']):
            if btn.get('class') and any(c in btn.get('class') for c in ['src', 'stab']):
                btn.decompose()
        text = title_tag.get_text(strip=True)
        if text:
            return text

    # Фоллбэк: извлекаем из URL
    filename = urlparse(url).path.split('/')[-1]
    return filename.replace('.html', '').replace('_', ' ').title()

def main():
    output_file = "slint_interpreter_doc.md"

    print(f"📥 Скачивание документации slint_interpreter v1.15.1")
    print(f"📄 Результат будет сохранён в: {output_file}\n")

    with open(output_file, "w", encoding="utf-8") as f:
        # Заголовок документа
        f.write("# Документация slint_interpreter v1.15.1\n\n")
        f.write(f"> Источник: [{BASE_URL}]({BASE_URL})\n")
        f.write(f"> Сгенерировано: {time.strftime('%Y-%m-%d %H:%M:%S')}\n\n")
        f.write("---\n")
        f.write("\n## Оглавление\n")

        # Сначала собираем оглавление
        toc_items = []
        for link in DOC_LINKS:
            item_name = link.split('/')[-1].replace('.html', '')
            item_type = link.split('.')[0]  # struct, enum, fn, etc.
            toc_items.append(f"- [{item_type}] `{item_name}` — [`{link}`](#{item_name.lower().replace('_', '-')})")

        f.write('\n'.join(toc_items))
        f.write("\n\n---\n")

        # Скачиваем и обрабатываем каждую страницу
        success_count = 0
        for i, link in enumerate(DOC_LINKS, 1):
            url = urljoin(BASE_URL, link)
            print(f"[{i}/{len(DOC_LINKS)}] Загрузка: {link}")

            html = fetch_page(url)
            if not html:
                f.write(f"\n### ⚠️ Не удалось загрузить: `{link}`\n\n")
                continue

            title = get_item_title(url, html)
            content = extract_doc_content(html, title)

            f.write(content)
            f.write("\n\n[🔝 Наверх](#оглавление)\n")
            success_count += 1

            # Вежливая задержка между запросами
            time.sleep(0.5)

        # Итоговая статистика
        f.write("\n---\n")
        f.write(f"\n> ✅ Успешно скачано: {success_count}/{len(DOC_LINKS)} страниц\n")

    print(f"\n✅ Готово! Документация сохранена в `{output_file}`")
    print(f"📊 Статистика: {success_count}/{len(DOC_LINKS)} страниц загружено успешно")

if __name__ == "__main__":
    # Проверка зависимостей
    try:
        import requests
        from bs4 import BeautifulSoup
    except ImportError as e:
        print(f"❌ Не установлены необходимые зависимости: {e}")
        print("Установите их командой: pip install requests beautifulsoup4")
        sys.exit(1)

    main()
