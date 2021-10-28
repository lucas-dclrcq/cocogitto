"use strict";(self.webpackChunkdocs=self.webpackChunkdocs||[]).push([[654],{3327:(e,a,n)=>{n.r(a),n.d(a,{default:()=>o});const t=(0,n(6252).uE)('<h2 id="repository-initialization" tabindex="-1"><a class="header-anchor" href="#repository-initialization" aria-hidden="true">#</a> Repository initialization</h2><p>⚠️ If you don&#39;t care about automatic versioning and just want to use <code>coco</code> to create conventional commits you can skip this section and jump to the next one.</p><p>To get the most out of cocogitto you need to have a <code>cog.toml</code> config at the root of your repository. You can create this file manually or generate the default one with <code>cog init</code>.</p><p>The default config file generated by cog init looks like this. You can read the configuration reference for more information.</p><div class="language-toml ext-toml"><pre class="language-toml"><code><span class="token key property">changelog_path</span> <span class="token punctuation">=</span> <span class="token string">&quot;CHANGELOG.md&quot;</span>\n<span class="token key property">pre_bump_hooks</span> <span class="token punctuation">=</span> <span class="token punctuation">[</span><span class="token punctuation">]</span>\n<span class="token key property">post_bump_hooks</span> <span class="token punctuation">=</span> <span class="token punctuation">[</span><span class="token punctuation">]</span>\n<span class="token key property">authors</span> <span class="token punctuation">=</span> <span class="token punctuation">[</span><span class="token punctuation">]</span>\n\n<span class="token punctuation">[</span><span class="token table class-name">commit_types</span><span class="token punctuation">]</span>\n</code></pre></div><h2 id="creating-a-new-repository" tabindex="-1"><a class="header-anchor" href="#creating-a-new-repository" aria-hidden="true">#</a> Creating a new repository</h2><div class="language-bash ext-sh"><pre class="language-bash"><code><span class="token function">mkdir</span> my_repo <span class="token operator">&amp;&amp;</span> <span class="token builtin class-name">cd</span> my_repo\ncog init\n</code></pre></div><p><code>cog init</code> works like <code>git init</code> except it create a template <code>cog.toml</code> config file, and a default initial commit with the following message : <code>chore: initial commit</code>.</p><p>Optionally you can specify the target path of the repository you want to create :</p><div class="language-bash ext-sh"><pre class="language-bash"><code>cog init my_repo\n</code></pre></div><h2 id="initializing-an-existing-repo" tabindex="-1"><a class="header-anchor" href="#initializing-an-existing-repo" aria-hidden="true">#</a> Initializing an existing repo</h2><p>Running <code>cog init</code> on an existing repository will just create a template configuration without creating any commit :</p><div class="language-bash ext-sh"><pre class="language-bash"><code><span class="token function">git</span> init my_repo <span class="token operator">&amp;&amp;</span> <span class="token builtin class-name">cd</span> my_repo\ncog init\n</code></pre></div><div class="language-text ext-text"><pre class="language-text"><code>❯ git status\nOn branch master\nChanges to be committed:\n  (use &quot;git restore --staged &lt;file&gt;...&quot; to unstage)\n\tnew file:   cog.toml\n</code></pre></div>',14),o={render:function(e,a){return t}}},8211:(e,a,n)=>{n.r(a),n.d(a,{data:()=>t});const t={key:"v-6ec693cd",path:"/guide/initialization.html",title:"",lang:"en-US",frontmatter:{},excerpt:"",headers:[{level:2,title:"Repository initialization",slug:"repository-initialization",children:[]},{level:2,title:"Creating a new repository",slug:"creating-a-new-repository",children:[]},{level:2,title:"Initializing an existing repo",slug:"initializing-an-existing-repo",children:[]}],filePathRelative:"guide/initialization.md",git:{contributors:[{name:"Paul Delafosse",email:"paul.delafosse@protonmail.com",commits:1}]}}}}]);