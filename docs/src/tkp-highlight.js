hljs.registerLanguage('tkp', (hljs) => {
  const KEYWORDS = {
    keyword: 'pali pana ijo awen la ante sin lon pini tawa kulupu lukin alasa jo sama insa ken nanpa',
    literal: 'kin ala weka',
    type: 'nanpa_kind kipisi sitelen lawa weka',
    built_in: 'toki kute lili_nanpa wawa_nanpa suli_nanpa nanpa_ante kipisi_ante suli_ijo lipu_lukin lipu_sitelen lipu_sin lipu_lon sitelen_pali toki_pakala nasin lape lawa_pali toki_ijo tenpo_ni suno_ni nanpa_tenpo nasin_alasa nasin_sama nasin_ante tawa_kama tawa_pana kulupu_lukin kulupu_pali kulupu_pona'
  };

  return {
    name: 'TKP',
    aliases: ['tkp', 'tokipona'],
    keywords: KEYWORDS,
    contains: [
      hljs.C_LINE_COMMENT_MODE,
      hljs.QUOTE_STRING_MODE,
      {
        className: 'number',
        variants: [
          { begin: '\\b\\d+(\\.\\d+)?\\b' }
        ],
        relevance: 0
      },
      {
        className: 'function',
        beginKeywords: 'pali', end: /\{/,
        excludeEnd: true,
        contains: [
          hljs.UNDERSCORE_TITLE_MODE,
          {
            className: 'params',
            begin: /\(/, end: /\)/,
            keywords: KEYWORDS,
            contains: [
              hljs.C_LINE_COMMENT_MODE,
              hljs.QUOTE_STRING_MODE
            ]
          }
        ]
      },
      {
        className: 'operator',
        begin: /==|!=|<=|>=|<|>|&&|\|\||!|=>|->|[+\-*/%]/,
        relevance: 0
      }
    ]
  };
});
// Re-initialize highlighting to include the new language
document.addEventListener('DOMContentLoaded', () => {
  document.querySelectorAll('pre code.language-tkp, pre code.language-tokipona').forEach((block) => {
    hljs.highlightBlock(block);
  });
});


