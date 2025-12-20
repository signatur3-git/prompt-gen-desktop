import {defineConfig} from 'vitepress'
import {withMermaid} from 'vitepress-plugin-mermaid'

export default withMermaid(defineConfig({
    title: 'Random Prompt Generator',
    description: 'Specification for a deterministic composite prompt generation system (v1.0.0)',
    srcDir: '.',
    cleanUrls: true,
    markdown: {
        theme: 'material-theme-palenight',
        lineNumbers: true
    },
    themeConfig: {
        nav: [
            {text: 'Home', link: '/'},
            {
                text: 'For Package Authors',
                items: [
                    {text: '📚 Getting Started', link: '/guides/getting-started'},
                    {text: '🎓 Tutorial Series', link: '/guides/tutorial-series/01-basic-package'},
                    {text: '📝 Template Syntax', link: '/architecture/template-syntax'},
                    {text: '🎨 Examples', link: '/examples/text-to-image-prompts'}
                ]
            },
            {
                text: 'For Implementers',
                items: [
                    {text: '🏗️ Architecture Overview', link: '/architecture/overview'},
                    {text: '📦 Components & Data Model', link: '/architecture/components'},
                    {text: '⚙️ Engine Primitives', link: '/architecture/engine-primitives'},
                    {text: '🔧 Context System', link: '/architecture/context-interactions'},
                    {text: '🏷️ Tag Filtering', link: '/architecture/tag-filtering'}
                ]
            },
            {
                text: 'Reference',
                items: [
                    {text: 'Template Syntax', link: '/architecture/template-syntax'},
                    {text: 'Context & Rules', link: '/architecture/context-interactions'},
                    {text: 'Tag Filtering', link: '/architecture/tag-filtering'},
                    {text: 'Glossary', link: '/glossary'}
                ]
            },
            {
                text: 'Project',
                items: [
                    {text: 'Milestones', link: '/milestones/index'},
                    {text: 'Examples', link: '/examples/text-to-image-prompts'}
                ]
            }
        ],
        sidebar: {
            // Guides sidebar
            '/guides/': [
                {
                    text: 'Getting Started',
                    items: [
                        {text: 'Introduction', link: '/guides/getting-started'}
                    ]
                },
                {
                    text: 'Tutorial Series',
                    items: [
                        {text: '1. Basic Package', link: '/guides/tutorial-series/01-basic-package'},
                        {text: '2. Tag Filtering', link: '/guides/tutorial-series/02-tag-filtering'},
                        {text: '3. Context Rules', link: '/guides/tutorial-series/03-context-rules'},
                        {text: '4. Lists & Separators', link: '/guides/tutorial-series/04-lists-separators'}
                    ]
                }
            ],

            // Architecture sidebar
            '/architecture/': [
                {
                    text: 'Architecture Documentation',
                    items: [
                        {text: 'Overview', link: '/architecture/overview'},
                        {text: 'Components & Data Model', link: '/architecture/components'},
                        {text: 'Template Syntax', link: '/architecture/template-syntax'},
                        {text: 'Context Interactions', link: '/architecture/context-interactions'},
                        {text: 'Tag Filtering', link: '/architecture/tag-filtering'},
                        {text: 'Engine Primitives', link: '/architecture/engine-primitives'}
                    ]
                }
            ],

            // Examples sidebar
            '/examples/': [
                {
                    text: 'Examples & Analysis',
                    items: [
                        {text: 'Text-to-Image Prompts', link: '/examples/text-to-image-prompts'},
                        {text: 'Authoring Analysis', link: '/examples/authoring-analysis'},
                        {text: 'Tag Filtering Examples', link: '/examples/tag-filtering-overrides'}
                    ]
                }
            ],

            // Milestones sidebar
            '/milestones/': [
                {
                    text: 'Development Milestones',
                    items: [
                        {text: 'Overview', link: '/milestones/index'},
                        {text: 'M1: Design Validation', link: '/milestones/M1_COMPLETE'},
                        {text: 'M2: Foundation', link: '/milestones/M2_VERIFIED_WORKING'},
                        {text: 'M3: Basic Rendering', link: '/milestones/M3_FINAL_COMPLETE'},
                        {text: 'M4: Context Operations', link: '/milestones/M4_COMPLETE'},
                        {text: 'M5: Advanced Features', link: '/milestones/M5_COMPLETE'},
                        {text: 'M6: Validation & CLI', link: '/milestones/M6_DONE_EPIC_DAY'},
                        {text: 'M7: Authoring Tool', link: '/milestones/M7_COMPLETE'},
                        {text: 'M8: Documentation', link: '/milestones/M8_COMPLETE'}
                    ]
                }
            ],

            // Default/home sidebar
            '/': [
                {
                    text: 'Introduction',
                    items: [
                        {text: 'Welcome', link: '/'},
                        {text: 'Getting Started', link: '/guides/getting-started'},
                        {text: 'Glossary', link: '/glossary'}
                    ]
                },
                {
                    text: 'Quick Links',
                    items: [
                        {text: 'Architecture Overview', link: '/architecture/overview'},
                        {text: 'Tutorial 1: Basics', link: '/guides/tutorial-series/01-basic-package'},
                        {text: 'Milestone Progress', link: '/milestones/index'}
                    ]
                }
            ]
        },

        // Footer
        footer: {
            message: 'Released under the MIT License.',
            copyright: 'Random Prompt Generator Specification v1.0.0'
        },

        // Search
        search: {
            provider: 'local'
        },

        // Social links
        socialLinks: [
            {icon: 'github', link: 'https://github.com/yourusername/prompt-gen-spec'}
        ]
    },

    mermaid: {
        theme: 'dark'
    },

    head: [
        [
            'link',
            {
                rel: 'stylesheet',
                href: 'https://cdn.jsdelivr.net/npm/katex@0.16.11/dist/katex.min.css'
            }
        ]
    ]
}))
