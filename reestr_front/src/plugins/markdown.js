import markdownIt from 'markdown-it'
import markdownItAnchor from 'markdown-it-anchor'

// Создаем экземпляр markdown-it
const md = markdownIt({
  html: true,
  linkify: true,
  typographer: true,
})

// Добавляем плагин markdown-it-anchor
md.use(markdownItAnchor, {
  permalink: markdownItAnchor.permalink.headerLink(), // Добавляет ссылку-якорь к заголовкам
})

// Экспортируем функцию для преобразования Markdown в HTML
export function renderMarkdown(markdownText) {
  return md.render(markdownText)
}
