import type { Monaco } from '@monaco-editor/react';

/**
 * FSH (FHIR Shorthand) language definition for Monaco Editor
 * Based on FSH grammar from https://fshschool.org/
 */
export function registerFSHLanguage(monaco: Monaco) {
  // Check if already registered
  if (monaco.languages.getLanguages().some((lang) => lang.id === 'fsh')) {
    return;
  }

  // Register the language
  monaco.languages.register({ id: 'fsh' });

  // Define tokens
  monaco.languages.setMonarchTokensProvider('fsh', {
    keywords: [
      'Alias',
      'Profile',
      'Extension',
      'Instance',
      'Invariant',
      'ValueSet',
      'CodeSystem',
      'RuleSet',
      'Mapping',
      'Logical',
      'Resource',
      'Parent',
      'Id',
      'Title',
      'Description',
      'Expression',
      'XPath',
      'Severity',
      'InstanceOf',
      'Usage',
      'Source',
      'Target',
      'Mixins',
      'Characteristics',
      'from',
      'contains',
      'named',
      'and',
      'or',
      'only',
      'obeys',
      'true',
      'false',
      'include',
      'exclude',
      'codes',
      'where',
      'valueset',
      'system',
      'exactly',
      'insert',
      'contentReference',
      'Reference',
      'Canonical',
      'CodeableReference',
    ],

    operators: ['=', '|', '->', '+', '..', 'MS', 'SU', 'N', 'TU', 'D', '?!'],

    bindingStrengths: ['#required', '#extensible', '#preferred', '#example'],

    // Token patterns
    tokenizer: {
      root: [
        // Comments
        [/\/\/.*$/, 'comment'],
        [/\/\*/, 'comment', '@comment'],

        // Strings
        [/"([^"\\]|\\.)*$/, 'string.invalid'],
        [/"/, 'string', '@string'],

        // Numbers
        [/\d+/, 'number'],

        // Binding strengths (special)
        [/#(required|extensible|preferred|example)/, 'keyword.binding'],

        // Flags
        [/\b(MS|SU|N|TU|D|\?!)\b/, 'keyword.flag'],

        // Cardinality
        [/\d+\.\.\d+/, 'number.cardinality'],
        [/\d+\.\.\*/, 'number.cardinality'],

        // Keywords
        [
          /\b(Alias|Profile|Extension|Instance|Invariant|ValueSet|CodeSystem|RuleSet|Mapping|Logical|Resource)\b/,
          'keyword.declaration',
        ],
        [
          /\b(Parent|Id|Title|Description|Expression|XPath|Severity|InstanceOf|Usage|Source|Target|Mixins|Characteristics)\b/,
          'keyword.metadata',
        ],
        [
          /\b(from|contains|named|and|or|only|obeys|include|exclude|codes|where|valueset|system|exactly|insert|contentReference)\b/,
          'keyword',
        ],
        [/\b(true|false)\b/, 'keyword.boolean'],
        [/\b(Reference|Canonical|CodeableReference)\b/, 'type.reference'],

        // URLs
        [/https?:\/\/[^\s]+/, 'string.url'],

        // Paths
        [/[A-Z][a-zA-Z0-9]*(\.[a-zA-Z][a-zA-Z0-9]*)+/, 'variable.path'],

        // Type names (PascalCase)
        [/[A-Z][a-zA-Z0-9]*/, 'type'],

        // Identifiers
        [/[a-z][a-zA-Z0-9_-]*/, 'identifier'],

        // Operators
        [/[=|+]/, 'operator'],
        [/->/, 'operator.arrow'],
        [/\.\./, 'operator.range'],

        // Delimiters
        [/[{}()[\]]/, 'delimiter.bracket'],
        [/[,:]/, 'delimiter'],

        // Whitespace
        [/\s+/, 'white'],
      ],

      comment: [
        [/[^/*]+/, 'comment'],
        [/\*\//, 'comment', '@pop'],
        [/[/*]/, 'comment'],
      ],

      string: [
        [/[^\\"]+/, 'string'],
        [/\\./, 'string.escape'],
        [/"/, 'string', '@pop'],
      ],
    },
  });

  // Define theme colors for FSH
  monaco.editor.defineTheme('fsh-theme', {
    base: 'vs',
    inherit: true,
    rules: [
      { token: 'keyword.declaration', foreground: '0000FF', fontStyle: 'bold' },
      { token: 'keyword.metadata', foreground: '0000FF' },
      { token: 'keyword', foreground: '0000FF' },
      { token: 'keyword.flag', foreground: 'FF6600', fontStyle: 'bold' },
      { token: 'keyword.binding', foreground: '9900CC' },
      { token: 'keyword.boolean', foreground: '0000FF' },
      { token: 'type', foreground: '267F99' },
      { token: 'type.reference', foreground: '267F99', fontStyle: 'italic' },
      { token: 'variable.path', foreground: '001080' },
      { token: 'string', foreground: 'A31515' },
      { token: 'string.url', foreground: '0066CC', fontStyle: 'underline' },
      { token: 'number', foreground: '098658' },
      { token: 'number.cardinality', foreground: 'FF6600' },
      { token: 'comment', foreground: '008000' },
      { token: 'operator', foreground: '000000' },
      { token: 'operator.arrow', foreground: 'FF6600' },
    ],
    colors: {},
  });

  // Register code completion provider
  monaco.languages.registerCompletionItemProvider('fsh', {
    provideCompletionItems: (model, position) => {
      const word = model.getWordUntilPosition(position);
      const range = {
        startLineNumber: position.lineNumber,
        endLineNumber: position.lineNumber,
        startColumn: word.startColumn,
        endColumn: word.endColumn,
      };

      const suggestions = [
        // Declaration keywords
        {
          label: 'Profile',
          kind: monaco.languages.CompletionItemKind.Keyword,
          insertText:
            'Profile: ${1:ProfileName}\nParent: ${2:Resource}\nId: ${3:profile-id}\nTitle: "${4:Profile Title}"\nDescription: "${5:Description}"',
          insertTextRules: monaco.languages.CompletionItemInsertTextRule.InsertAsSnippet,
          range,
        },
        {
          label: 'Extension',
          kind: monaco.languages.CompletionItemKind.Keyword,
          insertText:
            'Extension: ${1:ExtensionName}\nId: ${2:extension-id}\nTitle: "${3:Extension Title}"\nDescription: "${4:Description}"\n* value[x] only ${5:type}',
          insertTextRules: monaco.languages.CompletionItemInsertTextRule.InsertAsSnippet,
          range,
        },
        {
          label: 'ValueSet',
          kind: monaco.languages.CompletionItemKind.Keyword,
          insertText:
            'ValueSet: ${1:ValueSetName}\nId: ${2:valueset-id}\nTitle: "${3:ValueSet Title}"\nDescription: "${4:Description}"\n* include codes from system ${5:system-url}',
          insertTextRules: monaco.languages.CompletionItemInsertTextRule.InsertAsSnippet,
          range,
        },
        // Metadata
        {
          label: 'Parent',
          kind: monaco.languages.CompletionItemKind.Keyword,
          insertText: 'Parent: ${1:Resource}',
          insertTextRules: monaco.languages.CompletionItemInsertTextRule.InsertAsSnippet,
          range,
        },
        {
          label: 'Id',
          kind: monaco.languages.CompletionItemKind.Keyword,
          insertText: 'Id: ${1:id}',
          insertTextRules: monaco.languages.CompletionItemInsertTextRule.InsertAsSnippet,
          range,
        },
        {
          label: 'Title',
          kind: monaco.languages.CompletionItemKind.Keyword,
          insertText: 'Title: "${1:title}"',
          insertTextRules: monaco.languages.CompletionItemInsertTextRule.InsertAsSnippet,
          range,
        },
        {
          label: 'Description',
          kind: monaco.languages.CompletionItemKind.Keyword,
          insertText: 'Description: "${1:description}"',
          insertTextRules: monaco.languages.CompletionItemInsertTextRule.InsertAsSnippet,
          range,
        },
        // Flags
        { label: 'MS', kind: monaco.languages.CompletionItemKind.Keyword, insertText: 'MS', range },
        { label: 'SU', kind: monaco.languages.CompletionItemKind.Keyword, insertText: 'SU', range },
        // Binding strengths
        {
          label: '#required',
          kind: monaco.languages.CompletionItemKind.Value,
          insertText: '#required',
          range,
        },
        {
          label: '#extensible',
          kind: monaco.languages.CompletionItemKind.Value,
          insertText: '#extensible',
          range,
        },
        {
          label: '#preferred',
          kind: monaco.languages.CompletionItemKind.Value,
          insertText: '#preferred',
          range,
        },
        {
          label: '#example',
          kind: monaco.languages.CompletionItemKind.Value,
          insertText: '#example',
          range,
        },
      ];

      return { suggestions };
    },
  });
}
